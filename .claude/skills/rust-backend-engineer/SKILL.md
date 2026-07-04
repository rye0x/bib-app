---
name: rust-backend-engineer
description: Backend conventions for the cbob-bond / bond-backend service — Rust, Actix-web, sqlx/Postgres, JWT auth extractors, AppError, utoipa/OpenAPI, and ts-rs type export. Use this whenever writing or reviewing backend code for this project: adding or editing an HTTP handler, wiring authentication/roles, touching the sqlx db layer, defining request/response DTOs, handling errors, adding an endpoint + its Swagger docs, exporting types to the frontend, or writing backend tests. Trigger it even when the request is just "add an endpoint for X", "fix this handler", "write the query for Y", or "add a route" without naming Actix/sqlx — this backend has strong house rules that must be applied automatically.
---

# Rust Backend Engineer (cbob-bond / bond-backend)

House conventions for the `bond-backend` Actix-web service. Apply these automatically to all
backend code in this project — handlers, db layer, models, config, tests. The point is a codebase
where every handler looks the same, errors and auth are never re-implemented inline, and the
frontend's generated types always match the backend.

Before writing, read a neighbouring handler / db module and mirror it. The rules below are the
distilled house style; the two reference files hold the deeper recipes:

- **`references/database-sqlx.md`** — read when touching `src/db/`: executor generics, transactions,
  bulk upserts, batch fetches (avoiding N+1).
- **`references/openapi-and-types.md`** — read when adding/changing an endpoint or a DTO: the
  three-files-together Swagger rule and the ts-rs export contract.

## The core loop for a new endpoint

Adding an endpoint almost always touches these together — do them as one unit:
1. **Handler** in `src/handlers/` — `Result<HttpResponse, AppError>`, auth via extractor, `?` on
   fallible calls, returns a named struct directly.
2. **Model/DTO** — named request/response structs with the right derives (`ToSchema`, `TS`).
3. **DB method** on `DbPool` in `src/db/` — raw SQL stays here, atomic where needed.
4. **Route** registered under the versioned scope in `routes.rs` (path relative, no `/api/v1/`).
5. **OpenAPI** — `#[utoipa::path]` on the handler + registration in `src/openapi.rs`.
6. **Verify** — `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`,
   `cargo test` (regenerates TS). See `references/openapi-and-types.md` for the doc/type details.

## 1. Rust conventions

- **Format and lint before every commit.** From `src/bond-backend/`: `cargo fmt --all`, then
  `cargo clippy --all-targets --all-features -- -D warnings` and fix *every* diagnostic. CI and the
  pre-commit hook run `cargo fmt --all -- --check` and treat all clippy warnings as errors — so
  unformatted or warning-laden code is rejected, not merged.
- **Common clippy traps** worth pre-empting:
  - Compare with `entry.field`, not `entry.field.to_string()` (avoids `cmp_owned`).
  - `.unwrap_or(val)` for cheap values, not `.unwrap_or_else(|| val)` (avoids `unnecessary_lazy_evaluations`).
  - `if let Some(x) = opt { … }` instead of `.is_some()` then a separate `.unwrap()` (avoids `option_if_let_else`).
- Rust 2021 edition; add `#![deny(unused_imports)]` in new modules.
- Type public API signatures explicitly — don't lean on inference at API boundaries.
- `&str` for string params that don't need ownership; `String` for owned values stored in structs.
- **Never `.clone()` to escape a borrow error** — restructure ownership instead. A clone to silence
  the borrow checker is a smell, not a fix.
- `#[derive(Debug, Clone, Serialize, Deserialize)]` on model structs — but derive only what the type
  genuinely needs.
- Split a module when it passes ~1000 lines or starts mixing unrelated concerns.
- **No `unwrap()`/`expect()` on runtime paths** — use `?` for fallible work and `require_env` at boot
  (see §6). Panicking mid-request takes the whole worker down.
- Prefer `From`/`Into` impls over ad-hoc `convert_x_to_y` functions at domain-type boundaries.

## 2. Handler patterns

Handlers live in `src/handlers/`. Every handler:

- Returns **`Result<HttpResponse, AppError>`** — never bare `HttpResponse` or `impl Responder`.
  The `Result` is what lets `?` and `AppError` do the error mapping for you.
- Uses **`?`** on all fallible operations (DB, serialization, service calls).
- Declares its auth constraints **in the signature** via an extractor (§3), not with inline JWT code.
- **Returns data directly.** The frontend's `core.ts` already wraps every response as
  `{ success, data }`, so a handler that adds its own `SuccessResponse` wrapper double-wraps it.

```rust
// Good
pub async fn get_item(
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    user.require_role(&[Role::Admin, Role::Issuer])?;
    let item = db.get_item(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(item))
}

// Bad — manual JWT extraction repeated per handler, panics on error, wrong return type
pub async fn get_item(req: HttpRequest, config: web::Data<Config>, ...) -> HttpResponse {
    let claims = extract_and_verify_jwt(&req, &config).unwrap();
    ...
}
```

## 3. Authentication & authorization — `FromRequest` extractors

Take `AuthenticatedClaims` or `AuthenticatedUser` (from `src/extractors.rs`) as a handler parameter.
The extractor runs the JWT verification before your handler body, so you never repeat the match
block inline.

| Extractor | Provides | Use when |
|-----------|----------|----------|
| `AuthenticatedClaims` | Verified JWT claims (no DB fetch) | You only need token claims |
| `AuthenticatedUser` | Verified claims + DB-fetched user row | You role-gate or need user data |

Enforce roles with the one-liner — it returns `Err(AppError::Forbidden)` on failure, so `?`
propagates the 403 for you:

```rust
user.require_role(&[Role::Admin])?;
```

Don't hand-roll the `match extract_and_verify_jwt … / get_user_by_sub … / if !has_role` ladder in a
handler — that's exactly the boilerplate the extractors exist to delete.

## 4. Error handling — `AppError`

`AppError` (in `src/errors.rs`) implements `actix_web::ResponseError` and emits `{"error":"..."}`
with the right status. Because of its `From` impls, `?` converts underlying errors automatically:

- `From<sqlx::Error>`: `RowNotFound` → `AppError::NotFound`, everything else → `AppError::Internal`.
- `From<anyhow::Error>` → `AppError::Internal`.

So a DB call is one line — `let bond = db.get_bond(guid).await?;` — not a per-handler match block
that logs and returns a manual status.

**Domain rejections are successes, not errors.** When a valid business rule refuses a request (wrong
state, workflow conflict), that's an expected outcome — return it on the `Ok` path with the right
status, don't model it as an `Err`:

```rust
if bond.status != BondStatus::Active {
    return Ok(HttpResponse::Conflict().json(ErrorResponse::new("Bond is not active")));
}
```

## 5. Database patterns — sqlx  → see `references/database-sqlx.md`

DB logic lives in `src/db/`; handlers call methods on `DbPool` and raw SQL never leaks into a
handler. The headline rules, with full code in the reference:

- **Executor generic** (`E: sqlx::Executor<'e, Database = Postgres>`) for helpers that must run both
  standalone (`&pool`) and inside a transaction (`&mut *tx`).
- **Transactions for anything atomic** — multi-table writes *and* any read that a subsequent write
  depends on (read + conditional write in the same tx, or you get race conditions). Prefer named
  `*_atomic` helpers on `DbPool` over firing independent calls from the handler.
- **Bulk upserts** via `QueryBuilder::push_values` — one `INSERT … ON CONFLICT DO UPDATE`, never a
  per-row loop. Short-circuit on empty slices to avoid emitting `VALUES ()`.
- **Batch fetches** with `WHERE id = ANY($1)` + `HashMap` grouping — never one `SELECT` per id.

Read `references/database-sqlx.md` before writing db-layer code.

## 6. Configuration & boot

- Required env vars go through **`require_env("KEY")`** (from `src/config.rs`) — it panics with a
  clear message at startup, which is the *right* time to fail. Never `std::env::var("KEY").unwrap()`.
- Optional vars use `.unwrap_or` / `.unwrap_or_else` with a default.
- **CORS**: whitelist origins from `FRONTEND_URL` (required) and `ALLOWED_ORIGINS` (optional,
  comma-separated). Never `Cors::permissive()`.

## 7. Logging

Use the `log` macros (`log::error!`, `warn!`, `info!`, `debug!`) backed by `env_logger`; control
verbosity with `RUST_LOG`. Never `eprintln!` for diagnostics.

- Drop bracket prefixes that just restate the level (`[ERROR]`, `[WARNING]`) — redundant.
- Keep domain tags (`[AUDIT]`, `[EMAIL-TASK]`, `[GOOGLE-AUTH]`) — they carry meaning the level doesn't.

## 8. API response convention  → detail in `references/openapi-and-types.md`

- **Success**: return the data directly as JSON. No `SuccessResponse` wrapper, no `"success": true`
  field on any DTO — `core.ts` already wraps responses in `{ success, data }`, so both double up.
- **Errors**: use `AppError` (best) or the canonical `ErrorResponse` struct — never ad-hoc
  `serde_json::json!()` or `.body("text")`.
- **Named structs only** on every `Ok(HttpResponse::…)` path — never `serde_json::json!()` literals,
  bare booleans, or bare strings. Every struct that crosses the HTTP boundary is exported to
  TypeScript via `ts-rs` (see the reference for the exact derive/attribute block and the
  `cargo test` regeneration step).

## 9. Testing

- Integration tests in `tests/` run against a **real** Postgres (`sqlx::test` or a test `PgPool`) —
  do not mock the database; the value is in exercising the real SQL.
- Unit-test pure functions in the module's `#[cfg(test)]` block.
- Handler-level tests use `actix_web::test` helpers.
- Keep every test isolated (transactions or per-test schemas) so state doesn't leak between tests.
- E2E coverage is required for the common paths: create, read, update, delete, and auth-gated routes.

## 10. API versioning

Routes register under the versioned scope in `routes.rs`. Handler path strings are **relative** —
put `/api/v1/` only in `routes.rs`, never hardcoded in the handler or its `#[utoipa::path]` macro
path... except the `utoipa` `path` attribute, which does spell out the full `/api/v1/…` (see the
reference — this is the one documented exception, because Swagger needs the absolute path).

## 11. OpenAPI / Swagger + TypeScript export  → see `references/openapi-and-types.md`

Every new endpoint updates **three files together**: the handler (`#[utoipa::path]`), the model
(`utoipa::ToSchema` on every request/response struct), and `src/openapi.rs` (register the path,
schemas, and any new tag). Every boundary struct also carries the `ts-rs` export attributes, and
`cargo test` regenerates the frontend types. Full templates and the checklist are in the reference —
read it whenever you add or change an endpoint or DTO.

## Quick reference — do / don't

| Do | Don't |
|----|-------|
| `Result<HttpResponse, AppError>` on every handler | `HttpResponse` return or bare `impl Responder` |
| `?` for all fallible calls | Inline `match` repeating log + return per handler |
| `AuthenticatedUser` / `AuthenticatedClaims` params | `extract_and_verify_jwt` inside the handler body |
| `user.require_role(&[...])?` | `if !user.has_role(...) { return Forbidden() }` |
| `db.create_bond_atomic(...)` for multi-table writes | Independent `db.*` calls without a transaction |
| `QueryBuilder::push_values` for bulk inserts | Per-row `INSERT` loop |
| `WHERE id = ANY($1)` + `HashMap` grouping | Per-row `SELECT` loop in a list fetch |
| `require_env("VAR")` for required env vars | `std::env::var("VAR").unwrap()` |
| `log::error!(...)` / `log::info!(...)` | `eprintln!(...)` for diagnostics |
| `cargo fmt --all` after every backend change | Committing unformatted (CI/hook rejects) |
| `cargo clippy … -D warnings` and fix all | Ignoring clippy — CI fails on warnings |
| Return data directly from handlers | Wrap in `SuccessResponse { success, data }` |
| Named response struct on every `Ok` path | `serde_json::json!()`, `.json(true)`, `.json("str")` |
| No `"success": true` field in DTOs | Adding `success: bool` to DTOs |
| `#[ts(export, export_to = "generated/")]` on DTOs | Hand-written TS mirrors of backend types |
| `cargo test` to regenerate TS output | Editing `schema/generated/` by hand |
| Integration tests against a real DB | Mocked database in tests |
| Register endpoints in the versioned scope | Hardcode `/api/v1/` in the route registration |
| `#[utoipa::path(...)]` on every new handler | Skipping the macro or `openapi.rs` registration |
| `utoipa::ToSchema` on every request/response struct | Registering a struct without `ToSchema` |

# OpenAPI / Swagger docs + TypeScript export

Two contracts that must never drift: the Swagger docs must describe every endpoint, and the frontend
must have generated TypeScript types for every struct that crosses the HTTP boundary. Both are kept
honest by updating the right files whenever an endpoint or DTO changes.

## Typed success responses — named structs only

Every `Ok(HttpResponse::…)` success path returns a **named Rust struct** — never a
`serde_json::json!()` literal, a bare bool, or a bare string. Named structs are what get schema docs
and TS types; anonymous JSON gets neither and silently breaks the frontend's type safety.

And never add a `"success": true` field: `core.ts` on the frontend already wraps every response in
`{ success, data }`, so a `success` field inside `data` is redundant and confusing.

## ts-rs export contract

Every struct that appears in a request or response is exported to TypeScript via `ts-rs`. Use this
exact attribute block so casing and output location stay consistent:

```rust
#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "../../bond-frontend/src/lib/schema/generated/")]
pub struct MyResponse {
    pub field: String,
}
```

- Run **`cargo test`** to regenerate the TypeScript output — the export happens through the test
  harness.
- **Never hand-edit files in `schema/generated/`** — they're regenerated and your edits will be lost
  (and they'll disagree with the Rust source of truth).

## The three-files-together rule for a new/changed endpoint

Update all three in the same change, or the docs/types drift:

### 1. Handler file — `#[utoipa::path(...)]`

```rust
#[utoipa::path(
    get,
    path = "/api/v1/items/{id}",              // absolute path here (Swagger needs it)
    tag = "items",
    security(("bearer" = [])),
    params(("id" = Uuid, Path, description = "Item id")),
    // request_body = CreateItemRequest,      // when the endpoint takes a body
    responses(
        (status = 200, description = "The item", body = ItemResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
    ),
)]
pub async fn get_item(...) -> Result<HttpResponse, AppError> { ... }
```

Include the correct `tag`, `security(("bearer" = []))` for authed routes, `request_body` when
applicable, and **every** response status the handler can produce.

Note the tension with §10: the *route registration* in `routes.rs` uses a relative path under the
versioned scope, but the `utoipa` `path` attribute spells out the full `/api/v1/…` — this is the one
documented place the version prefix is written by hand, because the OpenAPI spec needs the absolute
path.

### 2. Model file — `utoipa::ToSchema`

Every struct used in a request or response derives `ToSchema` (alongside `Serialize`/`TS`):

```rust
#[derive(Serialize, TS, ToSchema)]
#[serde(rename_all = "camelCase")]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "../../bond-frontend/src/lib/schema/generated/")]
pub struct ItemResponse {
    pub id: Uuid,
    pub name: String,
}
```

### 3. `src/openapi.rs` — register it

- Add the handler to `paths(…)`.
- Add every new struct to `components(schemas(…))`.
- Add a new tag to `tags(…)` if this endpoint opens a new resource group.

A struct registered in `components(schemas(...))` without deriving `ToSchema` won't compile the docs;
a handler with `#[utoipa::path]` that isn't added to `paths(...)` won't appear in Swagger. Do all
three.

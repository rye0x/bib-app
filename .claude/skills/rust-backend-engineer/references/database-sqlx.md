# Database patterns — sqlx (`src/db/`)

All DB logic lives in `src/db/`. Handlers call methods on `DbPool`; raw SQL never appears in a
handler. These patterns exist to keep writes correct under concurrency and to keep the DB from being
hammered with N+1 round-trips.

## Executor generic — one helper, standalone or in a transaction

When a query needs to run both on its own and as part of a larger transaction, don't write it twice.
Take an executor generic so the caller decides:

```rust
async fn do_upsert_bond<'e, E>(executor: E, bond: &Bond) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    sqlx::query!("INSERT INTO bonds ...").execute(executor).await?;
    Ok(())
}
```

Call it with `&pool` for standalone use, or `&mut *tx` (the double-deref) inside a transaction.

## Transactions — atomic operations

Wrap any set of operations that must all succeed or all fail in a single sqlx transaction. This
covers two cases people often miss:

1. **Multi-table writes** — inserting a bond plus its KYC list plus its cross-chain rows.
2. **Read-then-conditional-write** — if a decision about what to write is based on data read from the
   DB, that read and the write must be in the *same* transaction. Otherwise another request can slip
   between them and you write based on stale state (a race → inconsistent data).

Prefer a named atomic helper on `DbPool` over orchestrating the steps from the handler:

```rust
// Good — one atomic call; all-or-nothing
db.create_bond_atomic(&bond, &kyc_list, &chain_list).await?;

// Bad — three independent calls; a failure on the 2nd leaves the DB half-written
db.upsert_bond(&bond).await?;
db.upsert_bond_kyc(&kyc).await?;
db.upsert_cross_chain(&chain).await?;
```

Inside the helper, open the tx, run each step against `&mut *tx`, then `tx.commit().await?`.

## Bulk upserts — one query, not N

Batch multiple rows into a single `INSERT … ON CONFLICT DO UPDATE` with
`sqlx::QueryBuilder::push_values`. A per-row loop is O(N) network round-trips and will dominate
latency on any real list.

```rust
// Good — single query
db.upsert_bond_kyc_bulk(&kyc_list).await?;

// Bad — O(N) round-trips
for kyc in &kyc_list {
    db.upsert_bond_kyc(kyc).await?;
}
```

Always short-circuit on an empty slice first — `push_values` over nothing emits invalid `VALUES ()`
SQL:

```rust
if items.is_empty() {
    return Ok(());
}
```

## Batch fetches — `ANY($1)`, not a query per id

When loading related rows for a list of ids, bind the whole array and group in memory. One query
replaces N.

```rust
// Good — one query, group in memory
let chains = db.get_cross_chains_for_bonds(&bond_guids).await?;
let by_bond: HashMap<Uuid, Vec<CrossChain>> = chains.into_iter().fold(
    HashMap::new(),
    |mut map, c| {
        map.entry(c.bond_guid).or_default().push(c);
        map
    },
);

// Bad — O(N) queries
for bond in &bonds {
    let chains = db.get_cross_chain_by_bond_guid(bond.guid).await?;
}
```

The db method runs `... WHERE bond_guid = ANY($1)` binding `&[Uuid]`; the handler/service does the
`HashMap` grouping.

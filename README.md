# 🥔 recipe-ranger

Web app written in [rust](https://www.rust-lang.org/) with [leptos](https://leptos.dev/).

Here is what it can do for you:

1. Enter the text of a recipe.
2. AI extracts the ingredients.
3. It searches for matching items from grocery stores.
4. Composes a shopping list, with prices & quantities.

## Development

1. Rename `.env.example` to `.env` and fill in the values.
2. Start the [Surreal](https://surrealdb.com/) database with docker compose.
3. Run the migrations with [surrealdb-migrations](https://github.com/Odonno/surrealdb-migrations).
4. Start the server with with `cargo leptos watch`.

```sh
# env vars
cp .env.example .env
# start db
docker compose up -d
# run migrations
cargo install surrealdb-migrations
surrealdb-migrations apply
# start server
cargo leptos watch
```

## TODOs

- [x] DB migration for users, invites and stuff
- [x] Accept invite
- [x] User Login
- [x] User Logout
- [x] Authentication wrapper for views
- [x] Submit recipe
- [x] Search for items matching an ingredient
- [x] Bookkeeping of AI costs
- [x] Deny requests if costs in past 24h exceed 50 cents
- [x] Nicer server errors
- [ ] CI file
- [ ] Deploy
- [ ] Add dev and deployment info to README

```

```

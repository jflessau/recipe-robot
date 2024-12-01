# 🥔 recipe-robot

[![CI](https://github.com/jflessau/recipe-robot/actions/workflows/ci.yml/badge.svg)](https://github.com/jflessau/recipe-robot/actions/workflows/ci.yml)

<img alt="A cute robot holding and looking at a piece of paper with a recipe on it." src="web/public/img/logo.png" width="180px"/>

Here is what it can do for you:

- 📝 Enter the text of a recipe
- 🧠 AI extracts the ingredients
- 🔍 It searches for matching items from grocery stores
- 🛒 Composes a shopping list, with prices, quantities & more

## Development

Web app is written in [svelte](https://svelte.dev/) and server in [rust](https://www.rust-lang.org/).

1. Insert your credentials in the `.env` file
2. Start [Surreal](https://surrealdb.com/) database
3. Run migrations
4. Start server
5. Install web app dependencies
6. Start web app

```sh

# switch to server dir
cd server
# populate .env file
cp .env.example .env
# start db
docker compose up -d
# run migrations
cargo install surrealdb-migrations
surrealdb-migrations apply
# start server
cargo run

# switch to web dir
cd ../web
# install dependencies with bun, npm, etc.
bun run i
# start web app
bun run dev
```

## Deployment

Docker images are built (and pushed to ghcr.io) within the CI actions.

# 🥔 recipe-robot

[![CI](https://github.com/jflessau/recipe-robot/actions/workflows/ci.yml/badge.svg)](https://github.com/jflessau/recipe-robot/actions/workflows/ci.yml)

<img alt="A cute robot holding and looking at a piece of paper with a recipe on it." src="web/public/img/logo.png" width="180px"/>

## How it works

- 📝 Enter the text of a recipe
- 🧠 AI extracts the ingredients
- 🔍 It searches for matching items from grocery stores
- 🛒 Composes a shopping list, with prices, quantities & more

## Demo

A demo is available at [rezept-roboter.jflessau.com](https://rezept-roboter.jflessau.com/).

Extracting and matching ingredients is done with a heavy LLM, which is not exactly cheap.
Therefore the demo limits requests per day. At the bottom of the demo app is a `xx %` indicator, showing how much of the daily quota is already used.

<details>
  <summary>Video</summary>

![Demo](demo.mp4)

</details>

## Limitations

As of right now, the apps UI is only available in German and the only grocery store queried is [Rewe Germany](https://www.rewe.de/).
Plans to expand the UI to other languages and grocery stores are in the works, but could take a while.

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

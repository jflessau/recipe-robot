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
  <summary>Demo GIF</summary>
  <img alt="Screen recording of the demo app. User enters a recipe into a text field, then hits the submit button. After a few seconds the app has extracted the ingredients like 'salt' & 'olive oil' and matched them to items from a grocery store. Ingredients with their matching items are displayed in a list, with prices and quantities. Above that list is another one of ingredients the user probably already has at home, for the user to quickly remove from the shopping list. With a click on a 'more info'-button on one of the listed ingredients, the user sees alternative items for an ingredient and then selects one of those as it matches the ingredient better (swaps parsley in a pot with parsley in a bag). Finally the users scrools back up and the recording ends." src="demo.gif" width="320px"/>

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

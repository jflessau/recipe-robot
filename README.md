# 🥔 recipe-robot

<img alt="A cute robot holding and looking at a piece of paper with a recipe on it." src="web/public/img/logo.png" width="180px"/>

Web app written in [rust](https://www.rust-lang.org/) and [svelte](https://svelte.dev/).

Here is what it can do for you:

- 📝 Enter the text of a recipe.
- 🧠 AI extracts the ingredients.
- 🔍 It searches for matching items from grocery stores.
- 🛒 Composes a shopping list, with prices, quantities & more.

## Development

1. Rename `./server/.env.example` to `./server/.env` and fill in the values
2. Start the [Surreal](https://surrealdb.com/) database
3. Run the migrations
4. Start the server
5. Install the web app dependencies
6. Start the web app

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
# start the web app
bun run dev
```

## Deployment

Docker images are built (and pushed to ghcr.io) within the CI actions.
For local building of the images, see `./.github/workflows/ci.yml`.

## ToDos

- [x] Swap logo and favicon
- [ ] CI file
- [ ] Deploy
- [ ] Beautify Readme

```

```

```

```

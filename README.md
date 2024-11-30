# 🥔 recipe-ranger

Web app written in [rust](https://www.rust-lang.org/) with [leptos](https://leptos.dev/).

Here is what it can do for you:

1. Enter the text of a recipe.
2. AI extracts the ingredients.
3. It searches for matching items from grocery stores.
4. Composes a shopping list, with prices & quantities.

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

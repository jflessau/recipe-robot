use super::shopping_list::{Ingredient, IngredientStatus};
use crate::prelude::*;

use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_base_url, set_key,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ai {
    max_chars: i32,
}

impl Ai {
    pub fn new(max_chars: i32) -> Result<Self> {
        Ok(Self { max_chars })
    }

    async fn ask<M: AsRef<str>>(&self, message: M) -> Result<String> {
        let mut message = message.as_ref().to_string();
        message = message.trim().to_string();

        if message.is_empty() {
            bail!("message is empty")
        }

        let msg_len = message.chars().count() as i32;

        if msg_len > self.max_chars {
            bail!(
                "message is too long, has {msg_len} chars, max is {}",
                self.max_chars
            )
        }

        let token = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

        set_key(token);
        set_base_url("https://api.openai.com/v1/".to_string());

        let mut messages = vec![ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some(r#"
                You are integrated into a recipe web app. Users enter recipes and you extract ingredients.
                Then the app calls an API of a grocery store and tries to find matches for the ingredients.
                You help to find the best match for the ingredients.
                If not told otherwise, you assume the API is talking german.
                You are agnostic to the recipe language.

                The app is in the early stages of development and you are the first AI to be integrated into it.
                Good luck!
            "#.to_string()),
            name: None,
            function_call: None,
        }];

        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(message),
            name: None,
            function_call: None,
        });

        let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
            .create()
            .await
            .context("failed to create chat completion")?;

        let Some(response) = chat_completion.choices.first() else {
            bail!("no response from ai")
        };

        let Some(message) = response.message.content.clone() else {
            bail!("no message in response from ai")
        };

        Ok(message)
    }

    pub async fn get_ingredients(&mut self, recipe: &String) -> Result<Vec<Ingredient>> {
        let message = r#"
            Extract all ingredients from the recipe.
            Translate the ingredients to german and convert the amounts to metric if necessary.

            The "name" will be used to search for an item in a grocery store API. So if e.g. the recipe says "diced onions", the ingredient name should be onion
            because "diced onions" is not a common grocery store item and won't work as a search term.

            If the recipe mentions eg. "eggs" and does not specify the kind of egg (chicken, quail, etc.), then assume the most common kind and make the best serach term for it.
            If the ingredient is e.g. "Extra virgin olive oil", then the ingredient name should be "olive oil" to increase the chances of finding via the API.
            If the ingredient's name is somewhat vague, e.g. "curry", use the specified amount to determin what it means. E.g. for 1 tsp of curry the best
            search term is probably "curry powder", not just "curry" as that would be too vague and yield results like curry paste. 

            The full name shall be stored in name_og.

            If the ingredient are very likely present in a regular household, set probably_at_home to true.
            Examples are pepper, salt, sugar, water, ice cubes,  etc. But thigs like saffron, truffle, eggs, milk, etc. are not.

            If the same ingredient is mentioned several times, e.g. for the dough and the sauce, for preparation and garnish, etc., then
            merge them into one ingredient. If the amounts are different, then add them up.

            Answer in the following format, so that your response can be parsed:
            [
                {
                    "name": "oliven öl",
                    "name_og": "extra virgin olive oil", 
                    "amount": "ingredient amount",
                    "probably_at_home": true,
                },
                ...
            ]
        "#;

        let response = self
            .ask(format!(
                "{}
                Recipe:
                {}
            ",
                message, recipe
            ))
            .await?;

        info!("ai response: {response}");

        let ingredients: Vec<Ingredient> = serde_json::from_str(&response)?;

        Ok(ingredients)
    }

    pub async fn match_item(
        &self,
        ingredient: &mut Ingredient,
        themes: &Vec<String>,
    ) -> Result<()> {
        // check if item list is empty

        if matches!(ingredient.status(), IngredientStatus::NoSearchResults) {
            return Ok(());
        }

        let IngredientStatus::SearchResults { ref items } = ingredient.status() else {
            bail!("ingredient has no search results")
        };

        if items.is_empty() {
            ingredient.set_status(IngredientStatus::NoSearchResults);
            return Ok(());
        }

        // compose prompt

        let prompt = r#"
            I give you a list of ingredients for a recipe.
            Each ingredient has a list of item candiates. These items are from a supermarktes API. 
            I want you to match the best item for each ingredient. 
            item_index is the index of the item in the list of item candidates.
            pieces_required means how many times one needs to purchase the item. If e.g. the recipe says 2.5 liters of milk and the selected item is 1 liter of milk,
            you must set the pieces_required to 3. 

            Keep in mind: if the recipe mentions e.g. 200g of olive oil and the candidates are measured in liters, convert the units.
            Also convert units like tsp, EL (esslöffel) to be able to compare them.
            Infer the meaning of abbreviation like EL, TL, etc. from the context.

            While selecting the best candidate, try to keep the excess quantity as low as possible and set the pieces_required accordingly.

            Also, respect user selected themes. Which could be an empty list, or something like "organic", "cheap", "regional", etc.

            Respond in this format: 
            ``json
            {
                "item_index": 0,
                "pieces_required": 1
            }
            ```
            but withou the backticks, just plain json and just one object, not an array.

            if there is no match for the ingredient, set item_index to null.
        "#;
        let prompt = format!("{prompt}\n\n{themes:?}\n\n, Ingredient: {ingredient:?}");

        // ask ai

        let response = self.ask(prompt).await.context("failed to ask ai")?;
        let response: IngredientItemMatch =
            serde_json::from_str(&response).context("failed to parse ai response")?;

        // ckeck if ai found a match

        let Some(index) = response.item_index else {
            ingredient.set_status(IngredientStatus::AiRefusedToSelectItem {
                alternatives: items.clone(),
            });
            return Ok(());
        };

        // check if index of match is in range

        let Some(item) = items.get(index).cloned() else {
            ingredient.set_status(IngredientStatus::AiSelectedInvalidItem {
                alternatives: items.clone(),
            });
            return Ok(());
        };

        // set item

        ingredient.set_status(IngredientStatus::Matched {
            item,
            pieces: response.pieces_required,
            alternatives: items.clone(),
        });

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct IngredientItemMatch {
    item_index: Option<usize>,
    pieces_required: usize,
}

use crate::prelude::*;

use crate::shopping_list::{Ingredient, IngredientStatus};

use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_base_url, set_key,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ai {
    max_chars: i32,
}

impl Ai {
    pub fn new() -> Self {
        Self { max_chars: 16_000 }
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
                Du bist in eine Rezept-Webanwendung integriert. Benutzer geben Rezepte ein, und du extrahierst die Zutaten.
                Anschließend ruft die App eine API eines Lebensmittelgeschäfts auf und versucht, passende Zutaten zu finden.
                Du hilfst dabei, die beste Übereinstimmung für die Zutaten zu finden.
                Du kennst dich sehr gut mit Lebensmitteln aus und kannst die besten Artikel für die Zutaten auswählen.
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
            // .model("gpt-4o")
            .model("gpt-4o-mini")
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

    pub async fn get_ingredients(self, recipe: &String) -> Result<Vec<Ingredient>> {
        let message = r#"
            Extrahiere alle Zutaten aus dem Rezept.
            Übersetze die Zutaten ins Deutsche, wenn nötig.

            "name" wird verwendet, um einen Artikel in einer API für Lebensmittelgeschäfte zu suchen. Wenn z. B. im Rezept „gewürfelte Zwiebeln“ steht, sollte der Zutatenname „Zwiebel“ sein, da „gewürfelte Zwiebeln“ kein gängiger Artikel in einem Lebensmittelgeschäft ist und als Suchbegriff nicht funktioniert.
            „name“ sollte korrekt großgeschrieben werden, z. B. „Zwiebel“.
            Wenn im Rezept z. B. „Eier“ erwähnt werden und die Art des Eis (Huhn, Wachtel, etc.) nicht angegeben ist, gehe von der häufigsten Art aus und wähle den besten Suchbegriff dafür.
            Wenn die Zutat z. B. „extra natives Olivenöl“ ist, sollte der Zutatenname „Olivenöl“ lauten, um die Chancen zu erhöhen, dass es über die API gefunden wird.
            Wenn der Name der Zutat vage ist, z. B. „Curry“, verwende die angegebene Menge, um zu bestimmen, was gemeint ist. Für 1 TL Curry wäre z. B. der beste Suchbegriff „Currypulver“, nicht nur „Curry“, da letzteres zu vage ist und Ergebnisse wie Currypaste liefern könnte.
            Falls dieselbe Zutat mehrfach erwähnt wird, z. B. für Teig und Sauce, dann liste sie nur einmal und addiere die Mengen.

            Für "unit" sind einzig und allein diese werte zulässig: "Gramm", "Kilogramm", "Milliliter", "Liter", "Stück".
            "quantity" gibt die Menge der Zutat in der Einheit an. Nur ganze Zahlen sind zulässig.
            Rechne "unit" und "quantity" entsprechend um, fall die im Rezept angegebene einheit nicht in der liste der zulässigen einheiten ist.
            
            Wenn die Zutat sehr wahrscheinlich in einem normalen Haushalt vorhanden ist, setze "probably_at_home" auf „true“.
            Beispiele dafür sind Pfeffer, Salz, Zucker, Wasser, Eiswürfel usw.

            Antwort im folgenden Format, damit die Antwort geparst werden kann. Verzichte auf backticks oder andere formatierung.

            [
                {
                    "name": "Olivenöl",
                    "unit": "milliliter",
                    "quantity": 50,
                    "probably_at_home": true
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

        match serde_json::from_str::<Vec<Ingredient>>(&response) {
            Ok(mut ingredients) => {
                ingredients.iter_mut().for_each(|i| i.enrich());
                Ok(ingredients)
            }
            Err(e) => {
                error!("failed to parse ai response: {}, error: {}", response, e);
                bail!("failed to parse ai response")
            }
        }
    }

    pub async fn match_item(&self, ingredient: &mut Ingredient) -> Result<()> {
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
            Ich gebe dir eine Zutat (Ingredient) für ein Rezept.
            Jede Zutat hat eine Liste von Artikelkandidaten. Diese Artikel stammen aus der API eines Supermarktes.
            Ich möchte, dass du den besten Artikel für die Zutat auswählst.

            item_index ist der Index des Artikels in der
            pieces_required gibt an, wie oft der Artikel gekauft werden muss. Wenn das Rezept z. B. 2,5 Liter Milch verlangt und der ausgewählte Artikel 1 Liter Milch umfasst, musst du pieces_required auf 3 setzen.

            Falls es keine Übereinstimmung für die Zutat gibt, setze item_index auf null.

            Antwort im folgenden Format, damit die Antwort geparst werden kann. Verzichte auf backticks oder andere formatierung.

            {
                "item_index": 0,
                "pieces_required": 1
            }
        "#;
        let prompt = format!(
            "{prompt}n\n, Zutat: {}\n\nArtikel aus dem Supermakrt: {:?}",
            ingredient.name, items
        );

        // ask ai

        let response_txt = self.ask(prompt).await.context("failed to ask ai")?;
        let response = serde_json::from_str::<IngredientItemMatch>(&response_txt);

        let Ok(response) = response else {
            error!("failed to parse ai response for matching item: {response_txt:?}, error: {response:?}");
            ingredient.set_status(IngredientStatus::AiFailsToSelectItem {
                alternatives: items.clone(),
            });
            return Ok(());
        };

        // ckeck if ai found a match

        let Some(index) = response.item_index else {
            error!("ai refused to select item for ingredient: {ingredient:?}");
            ingredient.set_status(IngredientStatus::AiFailsToSelectItem {
                alternatives: items.clone(),
            });
            return Ok(());
        };

        // check if index of match is in range

        let Some(item) = items.get(index).cloned() else {
            error!("ai selected item index out of range for ingredient: {ingredient:?}");
            ingredient.set_status(IngredientStatus::AiFailsToSelectItem {
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
struct IngredientItemMatch {
    item_index: Option<usize>,
    pieces_required: usize,
}

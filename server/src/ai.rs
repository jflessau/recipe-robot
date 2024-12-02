use crate::prelude::*;

use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_base_url, set_key,
};

pub enum AiUsage {
    InputToken(usize),
    OutputToken(usize),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiCostsDb {
    costs: f64,
}

impl AiUsage {
    pub fn input_token(characters: usize) -> Self {
        let token = characters as f32 / 3.6;
        Self::InputToken(token.ceil() as usize)
    }

    pub fn output_token(characters: usize) -> Self {
        let token = characters as f32 / 3.6;
        Self::OutputToken(token.ceil() as usize)
    }

    pub fn costs_in_micro_dollar(&self) -> usize {
        match self {
            Self::InputToken(token) => {
                (*token as f64 / 1_000_000.0 * 15.0 * 10_000.0).ceil() as usize
            }
            Self::OutputToken(token) => {
                (*token as f64 / 1_000_000.0 * 60.0 * 10_000.0).ceil() as usize
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ai {
    max_chars: i32,
}

impl Default for Ai {
    fn default() -> Self {
        Self { max_chars: 16_000 }
    }
}

impl Ai {
    async fn ask(
        &self,
        db: &Surreal<Any>,
        username: &String,
        message: &str,
    ) -> Result<String, Error> {
        let input_message = message.trim().to_string();

        if input_message.is_empty() {
            warn!("empty message");
            return Err(Error::BadRequest("empty message".to_string()));
        }

        let msg_len = input_message.chars().count() as i32;

        if msg_len > self.max_chars {
            warn!(
                "message too long: {msg_len} > {}, truncating",
                self.max_chars
            );
            return Err(Error::PayloadTooLarge);
        }

        check_limits(db, username).await?;

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
            content: Some(input_message.clone()),
            name: None,
            function_call: None,
        });

        let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
            .model("gpt-4o-mini")
            .create()
            .await?;

        let Some(response) = chat_completion.choices.first() else {
            warn!("no response from ai");
            return Err(Error::InternalServer);
        };

        let Some(output_message) = response.message.content.clone() else {
            warn!("no output message from ai");
            return Err(Error::InternalServer);
        };

        let ai_usages = vec![
            AiUsage::input_token(input_message.chars().count()),
            AiUsage::output_token(output_message.chars().count()),
        ];
        if let Err(err) = CashFlow::attribute_ai_costs(db, username, ai_usages).await {
            error!("failed to attribute ai costs: {:?}", err);
        }

        Ok(output_message)
    }

    pub async fn get_ingredients(
        self,
        db: &Surreal<Any>,
        username: &String,
        recipe: &String,
    ) -> Result<Vec<Ingredient>, Error> {
        let prompt = r#"
            Extrahiere alle Zutaten aus dem Rezept.
            Übersetze die Zutaten ins Deutsche, wenn nötig.

            "name" wird verwendet, um einen Artikel in einer API für Lebensmittelgeschäfte zu suchen. Wenn z. B. im Rezept „gewürfelte Zwiebeln“ steht, sollte der Zutatenname „Zwiebel“ sein, da „gewürfelte Zwiebeln“ kein gängiger Artikel in einem Lebensmittelgeschäft ist und als Suchbegriff nicht funktioniert.
            „name“ sollte korrekt großgeschrieben werden, z. B. „Zwiebel“.
            Wenn im Rezept z. B. „Eier“ erwähnt werden und die Art des Eis (Huhn, Wachtel, etc.) nicht angegeben ist, gehe von der häufigsten Art aus und wähle den besten Suchbegriff dafür.
            Wenn die Zutat z. B. „extra natives Olivenöl“ ist, sollte der Zutatenname „Olivenöl“ lauten, um die Chancen zu erhöhen, dass es über die API gefunden wird.
            Wenn der Name der Zutat vage ist, z. B. „Curry“, verwende die angegebene Menge, um zu bestimmen, was gemeint ist. Für 1 TL Curry wäre z. B. der beste Suchbegriff „Currypulver“, nicht nur „Curry“, da letzteres zu vage ist und Ergebnisse wie Currypaste liefern könnte.
            Falls dieselbe Zutat mehrfach erwähnt wird, z. B. für Teig und Sauce, dann liste sie nur einmal und addiere die Mengen.

            Für "unit" sind einzig und allein diese werte zulässig: "Gramm", "Kilogramm", "Milliliter", "Liter", "Stück".
            "quantity" gibt die Menge der Zutat in der Einheit an. Wenn möglich als Ganzzahl, ansonsten als Dezimalzahl.
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

        let prompt = format!("{prompt}\n\nRezept: {recipe}");
        let response = self.ask(db, username, &prompt).await?;
        let mut ingredients = serde_json::from_str::<Vec<Ingredient>>(&response)?;
        ingredients.iter_mut().for_each(|i| i.enrich());
        Ok(ingredients)
    }

    pub async fn match_item(
        &self,
        db: &Surreal<Any>,
        username: &String,
        ingredient: &mut Ingredient,
    ) -> Result<(), Error> {
        // check if item list is empty

        if ingredient.alternatives.is_empty() {
            warn!("tried to find items for ingredient without search results");
            return Err(Error::NotFound);
        }

        // compose prompt

        let prompt = r#"
            Ich gebe dir eine Zutat (Ingredient) für ein Rezept.
            Jede Zutat hat eine Liste von Artikelkandidaten. Diese Artikel stammen aus der API eines Supermarktes.
            Ich möchte, dass du den besten Artikel für die Zutat auswählst.

            item_index ist der Index des Artikels in der
            pieces_required gibt an, wie oft der Artikel gekauft werden muss um die Menge der Zutat zu decken.

            Falls es keine Übereinstimmung für die Zutat gibt, setze item_index auf null.

            Antwort im folgenden Format, damit die Antwort geparst werden kann. Verzichte auf backticks oder andere formatierung.

            {
                "item_index": 0,
                "pieces_required": 1
            }
        "#;
        let prompt = format!(
            "{prompt}\n\nZutat: {}\n\nBenötigte Menge der Zutat: {} {}\n\nArtikel aus dem Supermakrt: {:?}",
            ingredient.name,
            ingredient.quantity,
            ingredient.unit,
            ingredient.alternatives,
        );

        // ask ai

        let response = self.ask(db, username, &prompt).await?;
        let response = serde_json::from_str::<IngredientItemMatch>(&response)?;

        // ckeck if ai found a match

        let Some(index) = response.item_index else {
            error!("ai refused to select item for ingredient: {ingredient:?}");
            return Err(Error::InternalServer);
        };

        // check if index of match is in range

        let Some(item) = ingredient.alternatives.get(index).cloned() else {
            error!("ai selected item index out of range");
            return Ok(());
        };

        // set item

        ingredient.select_item(item.id, Some(response.pieces_required));

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
struct IngredientItemMatch {
    item_index: Option<usize>,
    pieces_required: i64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AiCostLimit {
    pub application_daily: f64, // in dollar
    pub user_daily: f64,        // in dollar
}

pub fn limits() -> AiCostLimit {
    let application_daily = std::env::var("APPLICATION_WIDE_DAILY_LIMIT_DOLLAR")
        .unwrap_or("1.0".to_string())
        .parse::<f64>()
        .unwrap_or(1.0);
    let user_daily = std::env::var("USER_DAILY_LIMIT_DOLLAR")
        .unwrap_or("0.1".to_string())
        .parse::<f64>()
        .unwrap_or(0.1);

    AiCostLimit {
        application_daily,
        user_daily,
    }
}

pub async fn application_daily_cost(db: &Surreal<Any>) -> Result<f64, Error> {
    let Some(application_daily_cost): Option<f64> = db
        .query(
            r#"
                (
                    select 
                        math::sum(->cash_flow.amount) as sum
                    from 
                        generates
                    where 
                        created_at > time::now() - 1d
                ).fold(100, |$a, $b| $a + $b.sum) / 1_000_000.0
            "#,
        )
        .await?
        .take(0)?
    else {
        error!("failed to query application wide daily costs");
        return Err(Error::InternalServer);
    };

    Ok(application_daily_cost)
}

pub async fn user_daily_cost(db: &Surreal<Any>, username: &String) -> Result<f64, Error> {
    let Some(user_daily_cost): Option<f64> = db
        .query(
            r#"
                (   
                    select 
                        math::sum(->cash_flow.amount) as sum
                    from 
                        generates
                    where 
                        created_at > time::now() - 1d
                        and array::first(<-user) = $user
                ).fold(100, |$a, $b| $a + $b.sum) / 1_000_000.0;
            "#,
        )
        .bind(("user", thing(&format!("user:{username}"))?))
        .await?
        .take(0)?
    else {
        error!("failed to query user daily costs");
        return Err(Error::InternalServer);
    };

    Ok(user_daily_cost)
}

pub async fn user_total_cost(db: &Surreal<Any>, username: &String) -> Result<f64, Error> {
    let Some(user_daily_cost): Option<f64> = db
        .query(
            r#"
                (   
                    select 
                        math::sum(->cash_flow.amount) as sum
                    from 
                        generates
                    where 
                        array::first(<-user) = $user
                ).fold(100, |$a, $b| $a + $b.sum) / 1_000_000.0;
            "#,
        )
        .bind(("user", thing(&format!("user:{username}"))?))
        .await?
        .take(0)?
    else {
        error!("failed to query user daily costs");
        return Err(Error::InternalServer);
    };

    Ok(user_daily_cost)
}

pub async fn check_limits(db: &Surreal<Any>, username: &String) -> Result<(), Error> {
    let limits = limits();
    let application_daily_limit = limits.application_daily;
    let user_daily_limit = limits.user_daily;

    // check if application wide limit is exceeded

    let application_wide_daily_costs = application_daily_cost(db).await?;
    info!("💶 application wide daily costs: ${application_wide_daily_costs:.2}, limit: ${application_daily_limit:.2}");
    if application_wide_daily_costs > application_daily_limit {
        warn!(
            "💶🔥 application wide daily limit exceeded: ${application_wide_daily_costs:.2} > ${application_daily_limit:.2}"
        );
        return Err(Error::TooManyRequests);
    }

    let user_daily_costs = user_daily_cost(db, username).await?;

    if user_daily_costs > user_daily_limit {
        warn!(
                "user '{username}' exceeded daily limit: ${user_daily_costs:.2} > ${user_daily_limit:.2}"
            );
        return Err(Error::PaymentRequired);
    }

    Ok(())
}

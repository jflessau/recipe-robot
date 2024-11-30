use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CashFlow {
    pub amount: i64, // micro dollar (- = out, + = in)
    pub origin: CashFlowOrigin,
}

impl CashFlow {
    pub async fn attribute_ai_costs(
        db: &Surreal<Any>,
        username: &String,
        ai_usages: Vec<AiUsage>,
    ) -> Result<()> {
        let cash_flows = ai_usages
            .iter()
            .map(|usage| {
                let amount = usage.costs_in_micro_dollar() as i64;
                let origin = CashFlowOrigin::from(usage);
                CashFlow { amount, origin }
            })
            .collect::<Vec<_>>();

        // insert cash flows

        for cash_flow in cash_flows {
            // create cash_flow

            let cash_flow_id = new_id();
            let _r: Option<CashFlow> = db
                .create(("cash_flow", &cash_flow_id))
                .content(cash_flow)
                .await?;

            // relate user to cash_flow

            let _r = db
                .insert::<Vec<Relation>>("generates")
                .relation(Relation {
                    r#in: thing(&format!("user:{username}"))?,
                    out: thing(&format!("cash_flow:{cash_flow_id}"))?,
                })
                .await?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CashFlowOrigin {
    #[serde(rename = "ai_input_token")]
    AiInputToken,
    #[serde(rename = "ai_output_token")]
    AiOutputToken,
    #[serde(rename = "private_assets")]
    PrivateAssets,
    #[serde(rename = "donation")]
    Donation,
}

impl From<&AiUsage> for CashFlowOrigin {
    fn from(usage: &AiUsage) -> Self {
        match usage {
            AiUsage::InputToken(_) => CashFlowOrigin::AiInputToken,
            AiUsage::OutputToken(_) => CashFlowOrigin::AiOutputToken,
        }
    }
}

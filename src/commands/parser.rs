use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "proposal_id")]
    pub proposal_id: String,
    pub content: Content,
    pub status: String,
    #[serde(rename = "final_tally_result")]
    pub final_tally_result: FinalTallyResult,
    #[serde(rename = "submit_time")]
    pub submit_time: String,
    #[serde(rename = "deposit_end_time")]
    pub deposit_end_time: String,
    #[serde(rename = "total_deposit")]
    pub total_deposit: Vec<TotalDeposit>,
    #[serde(rename = "voting_start_time")]
    pub voting_start_time: String,
    #[serde(rename = "voting_end_time")]
    pub voting_end_time: String,
    #[serde(rename = "is_expedited")]
    pub is_expedited: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub title: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalTallyResult {
    pub yes: String,
    pub abstain: String,
    pub no: String,
    #[serde(rename = "no_with_veto")]
    pub no_with_veto: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalDeposit {
    pub denom: String,
    pub amount: String,
}

impl Root {
    pub fn new(data: Vec<u8>) -> Root {
        let json_string =  String::from_utf8(data).unwrap();
        let json: Root = serde_json::from_str(&json_string).unwrap();
        json
    }
}
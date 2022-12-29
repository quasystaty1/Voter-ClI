use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub height: String,
    pub txhash: String,
    pub codespace: String,
    pub code: i64,
    pub data: String,
    #[serde(rename = "raw_log")]
    pub raw_log: String,
    pub logs: Vec<Log>,
    pub info: String,
    #[serde(rename = "gas_wanted")]
    pub gas_wanted: String,
    #[serde(rename = "gas_used")]
    pub gas_used: String,
    pub tx: Tx,
    pub timestamp: String,
    pub events: Vec<Event2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    #[serde(rename = "msg_index")]
    pub msg_index: i64,
    pub log: String,
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tx {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub body: Body,
    #[serde(rename = "auth_info")]
    pub auth_info: AuthInfo,
    pub signatures: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub messages: Vec<Message>,
    pub memo: String,
    #[serde(rename = "timeout_height")]
    pub timeout_height: String,
    #[serde(rename = "extension_options")]
    pub extension_options: Vec<Value>,
    #[serde(rename = "non_critical_extension_options")]
    pub non_critical_extension_options: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "proposal_id")]
    pub proposal_id: String,
    pub voter: String,
    pub option: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthInfo {
    #[serde(rename = "signer_infos")]
    pub signer_infos: Vec<SignerInfo>,
    pub fee: Fee,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignerInfo {
    #[serde(rename = "public_key")]
    pub public_key: PublicKey,
    #[serde(rename = "mode_info")]
    pub mode_info: ModeInfo,
    pub sequence: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModeInfo {
    pub single: Single,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Single {
    pub mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
    pub amount: Vec<Amount>,
    #[serde(rename = "gas_limit")]
    pub gas_limit: String,
    pub payer: String,
    pub granter: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    pub denom: String,
    pub amount: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Vec<Attribute2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute2 {
    pub key: String,
    pub value: String,
    pub index: bool,
}

impl Root {
    pub fn new(data: Vec<u8>) -> Root {
        let json_string =  String::from_utf8(data).unwrap();
        let json: Root = serde_json::from_str(&json_string).unwrap();
        json
    }
}

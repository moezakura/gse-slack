use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DialogRequestWrap {
    pub trigger_id: String,
    pub dialog: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DialogRequest {
    pub callback_id: String,
    pub title: String,
    pub submit_label: String,
    pub state: String,
    pub elements: Vec<DialogRequestElement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DialogRequestElement {
    #[serde(rename = "type")]
    pub _type: String,
    pub label: String,
    pub name: String,
    pub optional: bool,
}

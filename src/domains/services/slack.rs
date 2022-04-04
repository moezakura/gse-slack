use crate::domains::models;
use serde::Serialize;
use std::error::Error;

pub struct SlackService {
    token: String,
}

impl SlackService {
    pub fn new(token: String) -> SlackService {
        SlackService {
            token: token.clone(),
        }
    }

    fn get_token_header(&self) -> String {
        "Bearer ".to_string() + &*self.token
    }

    pub fn generate_dialog_request(&self) -> models::dialog_request::DialogRequest {
        models::dialog_request::DialogRequest {
            callback_id: "new-mail-address-dialog".to_string(),
            title: "New Mail address".to_string(),
            submit_label: "CREATE".to_string(),
            state: "state".to_string(),
            elements: vec![
                models::dialog_request::DialogRequestElement {
                    _type: "text".to_string(),
                    label: "Mail address".to_string(),
                    name: "mail-address".to_string(),
                    optional: false,
                },
                models::dialog_request::DialogRequestElement {
                    _type: "text".to_string(),
                    label: "Title".to_string(),
                    name: "mail-title".to_string(),
                    optional: false,
                },
                models::dialog_request::DialogRequestElement {
                    _type: "text".to_string(),
                    label: "Description".to_string(),
                    name: "mail-description".to_string(),
                    optional: true,
                },
            ],
        }
    }

    pub async fn send_text<T: Serialize + ?Sized>(
        &self,
        json: &T,
    ) -> Result<String, Box<dyn Error>> {
        let token_header = self.get_token_header();
        let client = reqwest::Client::new();
        let url = "https://slack.com/api/chat.postMessage";
        let resp = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", token_header)
            .json(&json)
            .send()
            .await?;

        Ok(resp.text().await?)
    }

    pub async fn open_dialog(
        &self,
        dialog: models::dialog_request::DialogRequestWrap,
    ) -> Result<String, Box<dyn Error>> {
        let token_header = self.get_token_header();
        let client = reqwest::Client::new();
        let url = "https://slack.com/api/dialog.open";
        let resp = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", token_header)
            .json(&dialog)
            .send()
            .await?;

        Ok(resp.text().await?)
    }
}

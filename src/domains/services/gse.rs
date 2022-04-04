use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct GseService {
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEmailRequest {
    pub name: String,
    pub description: String,
    pub mail_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEmailResponse {
    pub status: bool,
    pub message: String,
}

impl GseService {
    pub fn new(token: String) -> GseService {
        GseService { token }
    }

    fn get_token_header(&self) -> String {
        "Bearer ".to_string() + &*self.token
    }

    pub async fn create_mail(
        &self,
        req: CreateEmailRequest,
    ) -> Result<CreateEmailResponse, Box<dyn Error>> {
        let token_header = self.get_token_header();
        let client = reqwest::Client::new();
        let url = "https://gse.mox.si/create";
        let resp = client
            .put(url)
            .header("Content-Type", "application/json")
            .header("Authorization", token_header)
            .json(&req)
            .send()
            .await?;

        let result: CreateEmailResponse = resp.json().await?;
        Ok(result)
    }
}

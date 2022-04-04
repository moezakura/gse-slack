use crate::domains::models::data_set::ServiceSet;
use crate::domains::services::gse;
use crate::domains::services::gse::GseService;
use crate::domains::services::slack::SlackService;
use crate::domains::{models, services};
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct PostShortcutRequest {
    payload: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostShortcutRequestPayloadOnlyType {
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostShortcutRequestPayload {
    token: String,
    action_ts: String,
    callback_id: String,
    trigger_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostDialogSubmitRequestPayload {
    token: String,
    action_ts: String,
    callback_id: String,
    channel: PostDialogSubmitRequestPayloadChannel,
    submission: PostDialogSubmitRequestPayloadSubmission,
}
#[derive(Serialize, Deserialize, Debug)]
struct PostDialogSubmitRequestPayloadChannel {
    id: String,
    name: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct PostDialogSubmitRequestPayloadSubmission {
    #[serde(rename = "mail-address")]
    mail_address: String,
    #[serde(rename = "mail-description")]
    mail_description: Option<String>,
    #[serde(rename = "mail-title")]
    mail_title: String,
}

#[post("/show_dialog")]
pub async fn post(
    req: web::Form<PostShortcutRequest>,
    data: web::Data<models::data_set::ServiceSet>,
) -> impl Responder {
    if req.payload == "" {
        return HttpResponse::BadRequest().json(json!({
            "message": "invalid request",
        }));
    }

    let app_data = data.clone();
    let created = request(req.payload.clone(), app_data).await;

    if created.is_err() {
        println!("failed to create email: {:?}", created.err());
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

async fn request(payload: String, app_data: web::Data<ServiceSet>) -> Result<bool, Box<dyn Error>> {
    let req_type: Result<PostShortcutRequestPayloadOnlyType, serde_json::Error> =
        serde_json::from_str(payload.as_str());
    let req_type = req_type?._type;

    let app_data = app_data.clone();
    let slack_service = &app_data.slack;

    if req_type == "shortcut" {
        let req_payload: Result<PostShortcutRequestPayload, serde_json::Error> =
            serde_json::from_str(payload.as_str());

        let dialog_request = slack_service.generate_dialog_request();

        let trigger_id = req_payload?.trigger_id;

        let j = serde_json::to_string(&dialog_request)?;
        let dialog_data = models::dialog_request::DialogRequestWrap {
            trigger_id: trigger_id.to_string(),
            dialog: j,
        };
        slack_service.open_dialog(dialog_data).await?;
        return Ok(true);
    } else if req_type == "dialog_submission" {
        let req_payload: Result<PostDialogSubmitRequestPayload, serde_json::Error> =
            serde_json::from_str(payload.as_str());
        let req_payload = req_payload?;
        let create_options = req_payload.submission;
        let channel = req_payload.channel;

        tokio::spawn(async move {
            create_email_thread(channel.id.clone(), create_options, app_data).await;
        });
        return Ok(true);
    }

    Ok(true)
}

async fn create_email_thread(
    target_channel_id: String,
    create_options: PostDialogSubmitRequestPayloadSubmission,
    app_data: web::Data<ServiceSet>,
) -> Result<bool, Box<dyn Error>> {
    let description = match create_options.mail_description {
        None => create_options.mail_title.clone(),
        Some(s) => s,
    };

    let gse_service = &app_data.gse;
    let slack_service = &app_data.slack;

    let mail_address = create_options.mail_address.to_string() + "@mox.si";

    let create_request = gse::CreateEmailRequest {
        name: create_options.mail_title.to_string(),
        description,
        mail_address: mail_address.clone(),
    };
    let channel_id = target_channel_id;

    let send_text = "creating mail address: ".to_string() + &*mail_address.clone();
    let req_data = json!({
        "channel": channel_id.clone(),
        "text": send_text,
    });
    slack_service.send_text(&req_data).await;

    let create_result = gse_service.create_mail(create_request).await?;

    let send_text = if create_result.status {
        format!("success!\ncreated email: {}", create_result.message)
    } else {
        format!("failed!\nmessage: {}", create_result.message)
    };

    let req_data = json!({
        "channel": channel_id,
        "text": send_text,
    });
    slack_service.send_text(&req_data).await?;
    Ok(true)
}

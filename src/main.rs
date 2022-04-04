use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger;
use env_logger::Env;
use getopts::Options;
use gse_slack::controllers;
use gse_slack::domains::{models, services};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("c", "config", "config file path", "config.yaml");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("failed to parse command line args: {}", f.to_string())
        }
    };

    let config_path = matches.opt_str("c");
    let config_path = match config_path {
        None => "config.yaml".to_string(),
        Some(c) => c.to_string(),
    };

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config_service = services::config::ConfigService::new(config_path);
    if config_service.is_err() {
        panic!("failed to init: config service: {:?}", config_service.err());
    }
    let config_service = config_service.unwrap();
    let config_data = config_service.get_data();

    let gse_service = services::gse::GseService::new(config_data.gse_token);
    let slack_service = services::slack::SlackService::new(config_data.slack_token);

    let service_set = models::data_set::ServiceSet {
        config: config_service,
        gse: gse_service,
        slack: slack_service,
    };
    let controller_inject_item = web::Data::new(service_set);

    HttpServer::new(move || {
        App::new()
            .app_data(controller_inject_item.clone())
            .wrap(Logger::default())
            .service(controllers::show_dialog::post)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

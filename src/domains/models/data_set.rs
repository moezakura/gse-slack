use crate::domains::services;

pub struct ServiceSet {
    pub config: services::config::ConfigService,
    pub gse: services::gse::GseService,
    pub slack: services::slack::SlackService,
}

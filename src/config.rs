use std::env;

static GLANCES_URI: &str = "https://api.pushover.net/1/glances.json";

#[derive(Debug)]
pub struct Config {
    pub pushover_user_token: String,
    pub pushover_client_token: String,
    pub netdata_uri: String,
    pub glances_uri: String,
    pub update_cooldown: u64,
}

impl Config {
    pub fn load() -> Config {
        Config {
            pushover_user_token: env::var("PO_USER").unwrap(),
            pushover_client_token: env::var("PO_TOKEN").unwrap(),
            netdata_uri: env::var("ND_URI").unwrap(),
            glances_uri: env::var("PO_GLACES_URI")
                .unwrap_or(GLANCES_URI.to_string()),
            update_cooldown: env::var("COOLDOWN").unwrap().parse::<u64>().unwrap(),
        }
    }
}


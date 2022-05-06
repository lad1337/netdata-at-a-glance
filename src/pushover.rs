use serde::{Deserialize,Serialize}; 
use reqwest::blocking::Client;

use super::config;

pub fn update_glance(config: &config::Config, streams: i64){
    println!("Updating glance with {} people watching!", streams);
    let data = Glance {
        token: config.pushover_client_token.to_string(),
        user: config.pushover_user_token.to_string(),
        text: streams.to_string(),
        count: streams,
        percent: streams
    };
    let client = Client::new();
    let res = client.post(&config.glances_uri)
        .json(&data)
        .send();
    let resp = res.unwrap().json::<GlanceStatus>().unwrap();
    println!("Glances status: {:?}: {:?}", resp.status, resp.errors);
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Glance {
    pub token: String,
    pub user: String,
    pub text: String,
    pub count: i64,
    pub percent: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlanceStatus {
    pub status: i64,
    pub request: String,
    #[serde(default)]
    pub errors: Vec<String>,
}


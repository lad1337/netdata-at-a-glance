use serde::{Deserialize,Serialize}; 
use reqwest::blocking::get;

use super::config;

pub fn get_value(config: &config::Config) -> i64 {
    let res = get(&config.netdata_uri).unwrap();
    let resp = res.json::<NetdataResponse>().unwrap();
    let value = resp.latest_values.last();
    let mut s: i64 = 0;
    match value {
        Some(p) =>  (s = *p),
        None => (),
    }
    println!("Netadata value: {}", s);
    s
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NetdataResponse {
    pub api: i64,
    pub id: String,
    pub name: String,
    #[serde(rename = "view_update_every")]
    pub view_update_every: i64,
    #[serde(rename = "update_every")]
    pub update_every: i64,
    #[serde(rename = "first_entry")]
    pub first_entry: i64,
    #[serde(rename = "last_entry")]
    pub last_entry: i64,
    pub before: i64,
    pub after: i64,
    #[serde(rename = "dimension_names")]
    pub dimension_names: Vec<String>,
    #[serde(rename = "dimension_ids")]
    pub dimension_ids: Vec<String>,
    #[serde(rename = "latest_values")]
    pub latest_values: Vec<i64>,
    #[serde(rename = "view_latest_values")]
    pub view_latest_values: Vec<i64>,
    pub dimensions: i64,
    pub points: i64,
    pub format: String,
    pub result: Vec<i64>,
    pub min: i64,
    pub max: i64,
}

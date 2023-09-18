use super::EncryptedProfile;
use reqwest::{header::HeaderMap, header::AUTHORIZATION, header::CONTENT_TYPE, Client};
use serde::{Deserialize, Serialize};
use std::error::Error;
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub profiles: Vec<EncryptedProfile>,
    pub count: u32,
}

pub struct Config {
    pub api_key: String,
    pub endpoint: String,
}

const MAX_COUNT: u32 = 1000;

pub async fn get(config: &Config, count: u32) -> Result<Vec<EncryptedProfile>, Box<dyn Error>> {
    let api = Client::new();

    let calls = (count as f32 / MAX_COUNT as f32).ceil() as u32;
    let count_last_call = count - (calls - 1) * MAX_COUNT;

    let mut profiles: Vec<EncryptedProfile> = Vec::new();
    let mut err = None;
    for i in 0..calls {
        let count: u32 = if i == (calls - 1) {
            count_last_call
        } else {
            MAX_COUNT
        };

        log::debug!("Fetching {} profiles", count);

        let resp = get_profiles_helper(&api, count, config).await;

        let resp = match resp {
            Ok(r) => r,
            Err(e) => {
                log::error!("An error occurred while retrieving profiles. : {}", e);
                err = Some(e);
                break;
            }
        };

        profiles.extend(resp);
    }

    if !profiles.is_empty() {
        log::info!("Got {} profiles", profiles.len());
        Ok(profiles)
    } else {
        Err(err.unwrap_or("No profiles found".into()))
    }
}

#[derive(Serialize)]
struct RequestBody {
    count: u32,
}

async fn get_profiles_helper(
    client: &Client,
    count: u32,
    config: &Config,
) -> Result<Vec<EncryptedProfile>, Box<dyn Error>> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let body = RequestBody { count };
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, config.api_key.parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let response = client
        .get(config.endpoint.clone())
        .query(&[("version", VERSION)])
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(response.text().await?.into());
    }
    let a = response.json::<Response>().await?;
    Ok(a.profiles)
}

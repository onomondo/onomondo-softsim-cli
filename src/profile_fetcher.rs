use futures::{stream, StreamExt};
use reqwest::{header::AUTHORIZATION, Client, ResponseBuilderExt};
use serde::{Deserialize, Serialize};

// This `derive` requires the `serde` dependency.
#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    id: u32,
    iccid: String,
    data: String,
}

pub struct ApiConfig {
    pub api_key: String,
    pub endpoint: String,
}

pub struct ProfileFetcher {
    api_config: ApiConfig,
    num_of_profiles: u32,
}

// impl ProfileFetcher {
//     pub fn new(api_config: ApiConfig, num_of_profiles: u32) -> Self {
//         Self {
//             api_config,
//             num_of_profiles,
//         }
//     }

//     pub async fn fetch_many(&self) -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
//         let client = Client::new();

//         let concurrent_request = 10;

//         let bodies: Vec<Result<Profile, reqwest::Error>> = stream::iter(0..)
//             .map(|_| get_profile(&client, &self.api_config))
//             .take(self.num_of_profiles as usize)
//             .buffer_unordered(concurrent_request)
//             .collect()
//             .await;

//         Ok(bodies
//             .into_iter()
//             .collect::<Result<Vec<Profile>, reqwest::Error>>()?)
//     }
// }

// async fn get_profile(client: &Client, config: &ApiConfig) -> Result<Profile, reqwest::Error> {
//     let resp = client
//         .get(config.endpoint.clone())
//         .header(AUTHORIZATION, config.api_key.clone())
//         .send()
//         .await?;
//     resp.json::<Profile>().await
// }

// get count of profiles from enpoint in apiconfig
// async fn get_profiles(
//     client: &Client,
//     config: &ApiConfig,
//     count: u32,
// ) -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
//     assert!(count <= 1000);
//     let resp = client
//         .get(config.endpoint.clone())
//         .header(AUTHORIZATION, config.api_key.clone())
//         .header("count", count)
//         .send()
//         .await?;

//     test = resp.json::<Vec<Profile>>().await?
// }

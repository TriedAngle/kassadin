mod api;
use anyhow::Result;
use kassatypes::consts::Region;
use reqwest::header::HeaderMap;
use reqwest::Client;
use reqwest::ClientBuilder;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub struct API {
    pub client: Client,
}

#[derive(Debug, Default)]
pub struct Builder {
    header: HeaderMap,
    builder: ClientBuilder,
}

impl Builder {
    pub fn with_header(mut self, key: &'static str, value: &'static str) -> Self {
        self.header.append(key, value.parse().unwrap());
        self
    }

    pub fn with_riot_key(self, key: &'static str) -> Self {
        self.with_header("X-Riot-Token", key)
    }

    pub fn build(self) -> API {
        let builder = self.builder.default_headers(self.header);
        API {
            client: builder.build().unwrap(),
        }
    }
}

impl API {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn simple_request<T: DeserializeOwned + Debug>(
        &self,
        region: Region,
        url: &str,
    ) -> Result<T> {
        let url = format!("https://{}.api.riotgames.com{}", &region.to_string(), url);

        let response = self.client.get(url).send().await?;

        let parsed = response.json::<T>().await?;

        Ok(parsed)
    }

    pub fn account(&self) -> Account {
        Account { api: self }
    }

    pub fn champion_mastery(&self) -> ChampionMastery {
        ChampionMastery { api: self }
    }

    pub fn champion(&self) -> Champion {
        Champion { api: self }
    }

    pub fn clash(&self) -> Clash {
        Clash { api: self }
    }

    pub fn league(&self) -> League {
        League { api: self }
    }

    pub fn lol_status(&self) -> LoLStatus {
        LoLStatus { api: self }
    }

    pub fn summoner(&self) -> Summoner {
        Summoner { api: self }
    }
}

pub struct Account<'a> {
    api: &'a API,
}

pub struct ChampionMastery<'a> {
    api: &'a API,
}

pub struct Champion<'a> {
    api: &'a API,
}

pub struct Clash<'a> {
    api: &'a API,
}

pub struct League<'a> {
    api: &'a API,
}

pub struct LoLStatus<'a> {
    api: &'a API,
}

pub struct Summoner<'a> {
    api: &'a API,
}

#[cfg(test)]
mod test {
    use crate::API;
    use kassatypes::consts::Queue;
    use kassatypes::consts::Region;
    const KEY: &str = "RGAPI-e8648ab6-a883-4e6b-8596-d40b02ef0917";

    #[test]
    fn create_api() {
        let api = API::builder()
            .with_header("Content-Type", "application/json;charset=utf-8")
            .with_header("Content-Encoding", "gzip")
            .with_riot_key(KEY)
            .build();
    }

    #[test]
    fn make_request() {
        env_logger::init();
        let api = API::builder().with_riot_key(KEY).build();

        let challenger =
            tokio_test::block_on(api.league().challenger(Region::EUW, Queue::RankedSolo5x5))
                .unwrap();
        println!("challenger: {:?}", challenger)
    }
}

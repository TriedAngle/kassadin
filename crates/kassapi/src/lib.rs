mod api;
use reqwest::header::HeaderMap;
use reqwest::Client;
use reqwest::ClientBuilder;
use anyhow::Result;
use serde::de::DeserializeOwned;
use kassatypes::consts::Region;

pub struct API {
    pub requester: Client,
}

#[derive(Debug, Default)]
pub struct Builder {
    pub header: HeaderMap,
    pub builder: ClientBuilder,
}

impl Builder {
    pub fn add_header(mut self, key: &'static str, value: &'static str) -> Self {
        self.header.append(key, value.parse().unwrap());
        self
    }

    pub fn add_riot_key(self, key: &'static str) -> Self {
        self.add_header("X-Riot-Token", key)
    }

    pub fn build(self) -> API {
        API {
            requester: self.builder.build().unwrap()
        }
    }
}


impl API {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn simple_request<T: DeserializeOwned>(&self, region: Region, url: &str) -> Result<T> {
        let url = format!("https://{}.api.riotgames.com{}", region.as_str(), url);

        let request = self.requester.get(url)
            .send()
            .await?
            .json::<>()
            .await?;

        Ok(request)
    }

    pub fn account(&self) -> Account {
        Account { api: self}
    }

    pub fn champion_mastery(&self) -> ChampionMastery {
        ChampionMastery { api: self}
    }

    pub fn champion(&self) -> Champion {
        Champion { api: self}
    }

    pub fn clash(&self) -> Clash {
        Clash { api: self}
    }

    pub fn league(&self) -> League {
        League { api: self}
    }

    pub fn lol_status(&self) -> LOLStatus {
        LOLStatus { api: self}
    }

    pub fn summoner(&self) -> Summoner {
        Summoner { api: self}
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

pub struct LOLStatus<'a> {
    api: &'a API,
}

pub struct Summoner<'a> {
    api: &'a API,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let _client = API::builder()
            .add_riot_key("1020310")
            .build();
    }
}
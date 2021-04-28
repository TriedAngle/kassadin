use anyhow::Result;
use kassatypes::api;
use kassaroutes as routes;
use crate::{Account, ChampionMastery, Champion};
use kassatypes::consts;

impl<'a> Account<'a> {
    pub async fn by_puuid(&self, region: consts::Region, puuid: &str) -> Result<api::account::Account> {
        let url = routes::account::BY_PUUID.replace("{puuid}", puuid);
        self.api.simple_request(region, &url).await
    }

    pub async fn by_riot_id(&self, region: consts::Region, game_name: &str, tag_line: &str) -> Result<api::account::Account> {
        let url = routes::account::BY_RIOT_ID
            .replace("{gameName}", game_name)
            .replace("{tagLine}", tag_line);

        self.api.simple_request(region, &url).await
    }

    pub async fn active_shards(&self, region: consts::Region, game: &str, puuid: &str) -> Result<api::account::ActiveShard> {
        let url = routes::account::ACTIVE_SHARDS
            .replace("{game}", game)
            .replace("{puuid}", puuid);

        self.api.simple_request(region, &url).await
    }
}

impl<'a> ChampionMastery<'a> {
    pub async fn by_summoner(&self, region: consts::Region, summoner_id: &str) -> Result<Vec<api::champion_mastery::ChampionMastery>> {
        let url = routes::champion_mastery::BY_SUMMONER.replace("{encryptedSummonerId}", summoner_id);

        self.api.simple_request(region, &url).await
    }

    pub async fn by_summoner_by_champion(&self, region: consts::Region, summoner_id: &str, champion: consts::Champion) -> Result<api::champion_mastery::ChampionMastery> {
        let url = routes::champion_mastery::BY_SUMMONER_BY_CHAMPION
            .replace("{encryptedSummonerId}", summoner_id)
            .replace("{championId}", &champion.id().to_string());

        self.api.simple_request(region, &url).await
    }

    pub async fn scores_by_summoner(&self, region: consts::Region, summoner_id: &str) -> Result<i32> {
        let url = routes::champion_mastery::SCORES_BY_SUMMONER
            .replace("{encryptedSummonerId}", summoner_id);

        self.api.simple_request(region, &url).await
    }
}

impl<'a> Champion<'a> {
    pub async fn champion_rotations(&self, region: consts::Region) -> Result<api::champion::ChampionInfo> {
        let url = routes::champion::CHAMPION_ROTATIONS;

        self.api.simple_request(region, &url).await
    }
}
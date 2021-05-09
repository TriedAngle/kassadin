use crate::{Account, Champion, ChampionMastery, Clash, League, LoLStatus, Summoner};
use anyhow::Result;
use kassaroutes as routes;
use kassatypes::api;
use kassatypes::consts;
use log::{info, trace, warn};

impl<'a> Account<'a> {
    pub async fn by_puuid(
        &self,
        region: consts::Region,
        puuid: &str,
    ) -> Result<api::account::Account> {
        let url = routes::account::BY_PUUID.replace("{puuid}", puuid);
        self.api.simple_request(region, &url).await
    }

    pub async fn by_riot_id(
        &self,
        region: consts::Region,
        game_name: &str,
        tag_line: &str,
    ) -> Result<api::account::Account> {
        let url = routes::account::BY_RIOT_ID
            .replace("{gameName}", game_name)
            .replace("{tagLine}", tag_line);

        self.api.simple_request(region, &url).await
    }

    pub async fn active_shards(
        &self,
        region: consts::Region,
        game: &str,
        puuid: &str,
    ) -> Result<api::account::ActiveShard> {
        let url = routes::account::ACTIVE_SHARDS
            .replace("{game}", game)
            .replace("{puuid}", puuid);

        self.api.simple_request(region, &url).await
    }
}

impl<'a> ChampionMastery<'a> {
    pub async fn by_summoner(
        &self,
        region: consts::Region,
        summoner_id: &str,
    ) -> Result<Vec<api::champion_mastery::ChampionMastery>> {
        let url =
            routes::champion_mastery::BY_SUMMONER.replace("{encryptedSummonerId}", summoner_id);

        self.api.simple_request(region, &url).await
    }

    pub async fn by_summoner_by_champion(
        &self,
        region: consts::Region,
        summoner_id: &str,
        champion: consts::Champion,
    ) -> Result<api::champion_mastery::ChampionMastery> {
        let url = routes::champion_mastery::BY_SUMMONER_BY_CHAMPION
            .replace("{encryptedSummonerId}", summoner_id)
            .replace("{championId}", &champion.id().to_string());

        self.api.simple_request(region, &url).await
    }

    pub async fn scores_by_summoner(
        &self,
        region: consts::Region,
        summoner_id: &str,
    ) -> Result<i32> {
        let url = routes::champion_mastery::SCORES_BY_SUMMONER
            .replace("{encryptedSummonerId}", summoner_id);

        self.api.simple_request(region, &url).await
    }
}

impl<'a> Champion<'a> {
    pub async fn champion_rotations(
        &self,
        region: consts::Region,
    ) -> Result<api::champion::ChampionInfo> {
        let url = routes::champion::CHAMPION_ROTATIONS;

        self.api.simple_request(region, &url).await
    }
}

impl<'a> Clash<'a> {
    pub async fn by_summoner(
        &self,
        region: consts::Region,
        summoner_id: &str,
    ) -> Result<Vec<api::clash::Player>> {
        let url = routes::clash::BY_SUMMONER.replace("{encryptedSummonerId}", summoner_id);

        self.api.simple_request(region, &url).await
    }

    pub async fn team_by_id(
        &self,
        region: consts::Region,
        team_id: &str,
    ) -> Result<api::clash::Team> {
        let url = routes::clash::TEAM_BY_ID.replace("{teamId}", team_id);

        self.api.simple_request(region, &url).await
    }

    pub async fn tournaments(&self, region: consts::Region) -> Result<Vec<api::clash::Team>> {
        let url = routes::clash::TOURNAMENTS;

        self.api.simple_request(region, &url).await
    }

    pub async fn tournaments_by_team(
        &self,
        region: consts::Region,
        team_id: &str,
    ) -> Result<api::clash::Tournament> {
        let url = routes::clash::TOURNAMENTS_BY_TEAM.replace("{teamId}", team_id);

        self.api.simple_request(region, &url).await
    }

    pub async fn tournaments_by_id(
        &self,
        region: consts::Region,
        tournament_id: &str,
    ) -> Result<api::clash::Tournament> {
        let url = routes::clash::TOURNAMENTS_BY_ID.replace("{tournamentId}", tournament_id);

        self.api.simple_request(region, &url).await
    }
}

impl<'a> League<'a> {
    pub async fn challenger(
        &self,
        region: consts::Region,
        queue: consts::Queue,
    ) -> Result<api::league::LeagueList> {
        let url = routes::league::CHALLENGER_BY_QUEUE.replace("{queue}", &queue.to_string());

        self.api.simple_request(region, &url).await
    }

    pub async fn grandmaster(
        &self,
        region: consts::Region,
        queue: consts::Queue,
    ) -> Result<api::league::LeagueList> {
        let url = routes::league::GRANDMASTER_BY_QUEUE.replace("{queue}", &queue.to_string());

        self.api.simple_request(region, &url).await
    }

    pub async fn master(
        &self,
        region: consts::Region,
        queue: consts::Queue,
    ) -> Result<api::league::LeagueList> {
        let url = routes::league::MASTER_BY_QUEUE.replace("{queue}", &queue.to_string());

        self.api.simple_request(region, &url).await
    }

    pub async fn by_summoner(
        &self,
        region: consts::Region,
        summoner_id: &str,
    ) -> Result<Vec<api::league::LeagueEntry>> {
        let url = routes::league::BY_SUMMONER.replace("{encryptedSummonerId}", summoner_id);

        self.api.simple_request(region, &url).await
    }

    pub async fn by_id(&self, region: consts::Region, id: &str) -> Result<api::league::LeagueList> {
        let url = routes::league::BY_ID.replace("{leagueId}", id);

        self.api.simple_request(region, &url).await
    }

    pub async fn entries(
        &self,
        region: consts::Region,
        queue: consts::Queue,
        tier: consts::Tier,
        division: consts::Division,
        page: Option<i32>,
    ) -> Result<Vec<api::league::LeagueEntry>> {
        let url = match page {
            Some(page) => routes::league::ENTRIES_PAGED
                .replace("{queue}", &queue.to_string())
                .replace("{tier}", &tier.to_string())
                .replace("{division}", &division.to_string())
                .replace("{page}", &page.to_string()),
            None => routes::league::ENTRIES
                .replace("{queue}", &queue.to_string())
                .replace("{tier}", &tier.to_string())
                .replace("{division}", &division.to_string()),
        };

        self.api.simple_request(region, &url).await
    }

    pub async fn entries_exp(
        &self,
        region: consts::Region,
        queue: consts::Queue,
        tier: consts::Tier,
        division: consts::Division,
        page: Option<i32>,
    ) -> Result<Vec<api::league::LeagueEntry>> {
        let url = match page {
            Some(page) => routes::league::ENTRIES_EXP_PAGED
                .replace("{queue}", &queue.to_string())
                .replace("{tier}", &tier.to_string())
                .replace("{division}", &division.to_string())
                .replace("{page}", &page.to_string()),
            None => routes::league::ENTRIES_EXP
                .replace("{queue}", &queue.to_string())
                .replace("{tier}", &tier.to_string())
                .replace("{division}", &division.to_string()),
        };

        self.api.simple_request(region, &url).await
    }
}

impl<'a> LoLStatus<'a> {
    pub async fn challenger(
        &self,
        region: consts::Region,
    ) -> Result<api::lol_status::PlatformData> {
        let url = routes::lol_status::PLATFORM_DATA;

        self.api.simple_request(region, &url).await
    }
}

impl<'a> Summoner<'a> {
    pub async fn by_account(
        &self,
        region: consts::Region,
        account_id: &str,
    ) -> Result<api::summoner::Summoner> {
        let url = routes::summoner::BY_ACCOUNT.replace("{encryptedAccountId}", account_id);

        self.api.simple_request(region, &url).await
    }

    pub async fn by_name(
        &self,
        region: consts::Region,
        name: &str,
    ) -> Result<api::summoner::Summoner> {
        let url = routes::summoner::BY_NAME.replace("{summonerName}", name);

        self.api.simple_request(region, &url).await
    }

    pub async fn by_puuid(
        &self,
        region: consts::Region,
        puuid: &str,
    ) -> Result<api::summoner::Summoner> {
        let url = routes::summoner::BY_PUUID.replace("{encryptedPUUID}", puuid);

        self.api.simple_request(region, &url).await
    }

    pub async fn by_summoner_id(
        &self,
        region: consts::Region,
        summoner_id: &str,
    ) -> Result<api::summoner::Summoner> {
        let url = routes::summoner::BY_SUMMONER_ID.replace("{encryptedSummonerId}", summoner_id);

        self.api.simple_request(region, &url).await
    }
}

pub mod account {
    //
    pub const BY_PUUID: &str = "/riot/account/v1/accounts/by-puuid/{puuid}";
    pub const BY_RIOT_ID: &str = "/riot/account/v1/accounts/by-riot-id/{gameName}/{tagLine}";
    pub const ACTIVE_SHARDS: &str =
        "/riot/account/v1/active-shards/by-game/{game}/by-puuid/{puuid}";
}

pub mod champion_mastery {
    pub const BY_SUMMONER: &str =
        "/lol/champion-mastery/v4/champion-masteries/by-summoner/{encryptedSummonerId}";
    pub const BY_SUMMONER_BY_CHAMPION: &str = "/lol/champion-mastery/v4/champion-masteries/by-summoner/{encryptedSummonerId}/by-champion/{championId}";
    pub const SCORES_BY_SUMMONER: &str =
        "/lol/champion-mastery/v4/scores/by-summoner/{encryptedSummonerId}";
}

pub mod champion {
    pub const CHAMPION_ROTATIONS: &str = "/lol/platform/v3/champion-rotations";
}

pub mod clash {
    pub const BY_SUMMONER: &str = "/lol/clash/v1/players/by-summoner/{summonerId}";
    pub const TEAMS: &str = "/lol/clash/v1/teams/{teamId}";
    pub const TOURNAMENTS: &str = "/lol/clash/v1/tournaments";
    pub const BY_TEAMS: &str = "/lol/clash/v1/tournaments/by-team/{teamId}";
    pub const TOURNAMENTS_BY_ID: &str = "/lol/clash/v1/tournaments/{tournamentId}";
}

pub mod league {
    pub const CHALLENGER_BY_QUEUE: &str = "/lol/league/v4/challengerleagues/by-queue/{queue}";
    pub const GRANDMASTER_BY_QUEUE: &str = "/lol/league/v4/grandmasterleagues/by-queue/{queue}";
    pub const MASTER_BY_QUEUE: &str = "/lol/league/v4/masterleagues/by-queue/{queue}";
    pub const BY_SUMMONER: &str = "/lol/league/v4/entries/by-summoner/{encryptedSummonerId}";
    pub const BY_ID: &str = "/lol/league/v4/leagues/{leagueId}";
    pub const ENTRIES: &str = "/lol/league/v4/entries/{queue}/{tier}/{division}";
    pub const ENTRIES_FULL: &str = "/lol/league-exp/v4/entries/{queue}/{tier}/{division}";
}

pub mod lol_status {
    pub const PLATFORM_DATA: &str = "/lol/status/v4/platform-data";
}

pub mod summoner {
    pub const BY_ACCOUNT: &str = "/lol/summoner/v4/summoners/by-account/{encryptedAccountId}";
    pub const BY_NAME: &str = "/lol/summoner/v4/summoners/by-name/{summonerName}";
    pub const BY_PUUID: &str = "/lol/summoner/v4/summoners/by-puuid/{encryptedPUUID}";
    pub const BY_ID: &str = "/lol/summoner/v4/summoners/{encryptedSummonerId}";
}

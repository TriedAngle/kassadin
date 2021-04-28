pub mod account {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct Account {
        pub puuid: String,
        pub game_name: String,
        pub tag_line: String,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct ActiveShard {
        pub puuid: String,
        pub game: String,
        pub active_shard: String,
    }
}

pub mod champion_mastery {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct ChampionMastery {
        pub champion: i64,
        pub chest_granted: bool,
        pub champion_id: i64,
        pub last_play_time: i64,
        pub champion_level: i32,
        pub summoner_id: String,
        pub champion_points: i32,
        pub champion_points_since_last_level: i64,
        pub tokens_earned: i32,
    }
}

pub mod champion {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct ChampionInfo {
        pub max_new_player_level: i32,
        pub free_champion_ids_for_new_players: Vec<i32>,
        pub free_champion_ids: Vec<i32>,
    }
}

pub mod clash {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct Player {
        pub summoner_id: String,
        pub team_id: String,
        pub position: String,
        pub role: String,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct Team {
        pub id: String,
        pub tournament_id: i32,
        pub name: String,
        pub icon_id: i32,
        pub tier: i32,
        // summoner_id
        pub captain: String,
        pub abbreviation: String,
        pub players: Vec<Player>,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct Tournament {
        pub id: i32,
        pub theme_id: i32,
        pub name_key: String,
        pub name_key_secondary: String,
        pub schedule: Vec<TournamentPhase>,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct TournamentPhase {
        pub id: i32,
        pub registration_time: i64,
        pub start_time: i64,
        pub cancelled: bool,
    }
}

pub mod league {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct LeagueEntry {
        pub league_id: String,
        pub summoner_id: String,
        pub summoner_name: String,
        pub queue_type: String,
        pub tier: String,
        pub rank: String,
        pub league_points: i32,
        pub wins: i32,
        pub losses: i32,
        pub hot_streak: bool,
        pub veteran: bool,
        pub fresh_blood: bool,
        pub inactive: bool,
        pub mini_series: Option<MiniSeries>,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct LeagueList {
        pub league_id: String,
        pub entries: Vec<LeagueItem>,
        pub tier: String,
        pub name: String,
        pub queue: String,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct LeagueItem {
        pub fresh_blood: bool,
        pub wins: i32,
        pub summoner_name: String,
        pub mini_series: MiniSeries,
        pub inactive: bool,
        pub veteran: bool,
        pub hot_streak: bool,
        pub rank: String,
        pub league_points: i32,
        pub losses: i32,
        pub summoner: String,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct MiniSeries {
        pub losses: i32,
        pub progress: String,
        pub target: i32,
        pub wins: i32,
    }
}

pub mod lol_status {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct PlatformData {
        pub id: String,
        pub name: String,
        pub locales: Vec<String>,
        pub maintenance: Vec<Status>,
        pub incidents: Vec<Status>,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct Status {
        pub id: i32,
        pub maintenance_status: String,
        pub incident_severity: String,
        pub titles: Vec<Content>,
        pub updates: Vec<Update>,
        pub created_at: String,
        pub archived_at: String,
        pub updated_at: String,
        pub platforms: Vec<String>,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct Content {
        pub locale: String,
        pub content: String,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct Update {
        pub id: i32,
        pub author: String,
        pub publish: bool,
        pub publish_locations: Vec<String>,
        pub translations: Vec<Content>,
        pub created_at: String,
        pub updated_at: String,
    }
}

pub mod summoner {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct Summoner {
        pub account_id: String,
        pub profile_icon_id: i32,
        pub revision_date: i64,
        pub name: String,
        pub id: String,
        pub puuid: String,
        pub summoner_level: i64,
    }
}

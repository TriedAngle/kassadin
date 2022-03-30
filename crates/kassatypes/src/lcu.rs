pub mod consts {
    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};
    #[cfg(feature = "serde_repr")]
    use serde_repr::{Deserialize_repr, Serialize_repr};
    use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(
    EnumString,
    EnumIter,
    Display,
    AsRefStr,
    IntoStaticStr,
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    )]
    pub enum Position {
        TOP,
        JUNGLE,
        MIDDLE,
        BOTTOM,
        UTILITY,
        FILL,
        #[serde(other)]
        UNSELECTED,
    }

    impl Default for Position {
        fn default() -> Self {
            Self::TOP
        }
    }
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]
    #[derive(
    EnumString,
    EnumIter,
    Display,
    AsRefStr,
    IntoStaticStr,
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    )]
    pub enum Tier {
        None = 10,
        Challenger = 9,
        Grandmaster = 8,
        Master = 7,
        Diamond = 6,
        Platinum = 5,
        Gold = 4,
        Silver = 3,
        Bronze = 2,
        Iron = 1,
        Unranked = 0,
    }

    impl From<Tier> for crate::consts::Tier {
        fn from(t: Tier) -> Self {
            match t {
                Tier::Challenger => crate::consts::Tier::Challenger,
                Tier::Grandmaster => crate::consts::Tier::Grandmaster,
                Tier::Master => crate::consts::Tier::Master,
                Tier::Diamond => crate::consts::Tier::Diamond,
                Tier::Platinum => crate::consts::Tier::Platinum,
                Tier::Gold => crate::consts::Tier::Gold,
                Tier::Silver => crate::consts::Tier::Silver,
                Tier::Bronze => crate::consts::Tier::Bronze,
                Tier::Iron => crate::consts::Tier::Iron,
                Tier::Unranked => crate::consts::Tier::Unranked,
                Tier::None => panic!("illegal lol"),
            }
        }
    }

    impl From<crate::consts::Tier> for Tier {
        fn from(t: crate::consts::Tier) -> Self {
            match t {
                crate::consts::Tier::Challenger => Tier::Challenger,
                crate::consts::Tier::Grandmaster => Tier::Grandmaster,
                crate::consts::Tier::Master => Tier::Master,
                crate::consts::Tier::Diamond => Tier::Diamond,
                crate::consts::Tier::Platinum => Tier::Platinum,
                crate::consts::Tier::Gold => Tier::Gold,
                crate::consts::Tier::Silver => Tier::Silver,
                crate::consts::Tier::Bronze => Tier::Bronze,
                crate::consts::Tier::Iron => Tier::Iron,
                crate::consts::Tier::Unranked => Tier::Unranked,
            }
        }
    }

    #[repr(u32)]
    #[cfg_attr(feature = "serde_repr", derive(Serialize_repr, Deserialize_repr))]
    #[derive(
    EnumString,
    EnumIter,
    Display,
    AsRefStr,
    IntoStaticStr,
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    )]
    pub enum QueueId {
        Draft = 400,
        Solo = 420,
        Blind = 430,
        Flex = 440,
        Aram = 450,
        Clash = 700,
        BotsIntro = 830,
        BotsBeginner = 840,
        BotsIntermediate = 850,
        ARURF = 900,
        TFTNormal = 1090,
        TFTRanked = 1100,
        TFTTutorial = 1110,
        TFTHyperRoll = 1130,
        NexusBlitz = 1300,
        Tutorial1 = 2000,
        Tutorial2 = 2010,
        Tutorial3 = 2020,
    }
}

pub mod summoner {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct Summoner {
        pub account_id: i64,
        pub display_name: String,
        pub internal_name: String,
        pub name_change_flag: bool,
        pub percent_complete_for_next_level: i32,
        pub profile_icon_id: i32,
        pub puuid: String,
        pub reroll_points: RerollPoints,
        pub summoner_id: i64,
        pub summoner_level: i64,
        pub unnamed: bool,
        pub xp_since_last_level: i32,
        pub xp_until_next_level: i32,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct RerollPoints {
        pub current_points: i32,
        pub max_rolls: i32,
        pub number_of_rolls: i32,
        pub points_cost_to_roll: i32,
        pub points_to_reroll: i32,
    }
}

pub mod chat {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Default)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct Friend {
        pub availability: Option<String>,
        pub display_group_id: Option<i32>,
        pub display_group_name: String,
        pub game_name: String,
        pub game_tag: String,
        pub group_id: i32,
        pub group_name: String,
        pub icon: i32,
        pub id: String,
        #[serde(rename = "isP2PConversationMuted")]
        pub is_p2p_conversation_muted: Option<bool>,
        pub last_seen_online_timestamp: Option<u64>,
        pub lol: LoL,
        pub name: String,
        pub note: String,
        pub patchline: Option<String>,
        pub pid: Option<String>,
        pub platform_id: Option<String>,
        pub product: Option<String>,
        pub product_name: Option<String>,
        pub puuid: Option<String>,
        pub status_message: Option<String>,
        pub summary: Option<String>,
        pub summoner_id: i64,
        pub time: Option<u64>
    }

    #[derive(Debug, Clone, Default)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct Me {
        pub availability: Option<String>,
        pub game_name: Option<String>,
        pub game_tag: Option<String>,
        pub icon: Option<i32>,
        pub id: Option<String>,
        pub last_seen_online_timestamp: Option<i64>,
        pub lol: Option<LoL>,
        pub name: Option<String>,
        pub patchline: Option<String>,
        pub pid: Option<String>,
        pub platform_id: Option<String>,
        pub product: Option<String>,
        pub product_name: Option<String>,
        pub puuid: Option<String>,
        pub status_message: Option<String>,
        pub summary: Option<String>,
        pub summoner_id: Option<i64>,
        pub time: Option<i32>,
    }

    #[derive(Debug, Clone, Default)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct LoL {
        pub champion_id: Option<String>,
        pub companion_id: Option<String>,
        pub damage_skin_id: Option<String>,
        pub game_id: Option<String>,
        pub game_mode: Option<String>,
        pub game_queue_type: Option<String>,
        pub game_status: Option<String>,
        pub icon_override: Option<String>,
        pub init_rank_stat: Option<String>,
        pub is_observable: Option<String>,
        pub level: Option<String>,
        pub map_id: Option<String>,
        pub map_skin_id: Option<String>,
        pub mastery_score: Option<String>,
        pub profile_icon: Option<String>,
        pub pty: Option<String>,
        pub puuid: Option<String>,
        pub queue_id: Option<String>,
        pub ranked_league_division: Option<String>,
        pub ranked_league_queue: Option<String>,
        pub ranked_league_tier: Option<String>,
        pub ranked_losses: Option<String>,
        pub ranked_split_reward_level: Option<String>,
        pub ranked_prev_season_tier: Option<String>,
        pub ranked_wins: Option<String>,
        pub regalia: Option<String>,
        pub skin_variant: Option<String>,
        pub skinname: Option<String>,
        pub time_stamp: Option<String>,
    }
}

pub mod champ_select {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct ChampSelectSummoner {
        acting_background_animation_state: String,
        active_action_type: String,
        are_summoner_actions_complete: bool,
        assigned_position: String,
        ban_intent_square_portrait_path: String,
        cell_id: i32,
        champion_icon_style: String,
        champion_name: String,
        current_champion_vote_percent_integer: i32,
        entitled_feature_type: String,
        is_acting_now: bool,
        is_done_picking: bool,
        is_on_players_team: bool,
        is_pick_intenting: bool,
        is_placeholer: bool,
        is_self: bool,
        pick_sniped_class: String,
        should_show_acting_bar: bool,
        should_show_ban_intent_icon: bool,
        should_show_expanded: bool,
        should_show_ring_animations: bool,
        should_show_selected_skin: bool,
        should_show_spells: bool,
        show_muted: bool,
        show_trades: bool,
        skin_id: i32,
        skin_splash_path: String,
        slot_id: i32,
        spell1icon_path: String,
        spell2icon_path: String,
        status_message_key: String,
        summoner_id: String,
        trade_id: i32,
    }
}

pub mod lobby {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};
    use crate::lcu::consts::Position;

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct Lobby {
        pub can_start_activity: Option<bool>,
        pub chat_room_id: Option<String>,
        pub chat_room_key: Option<String>,
        pub game_config: Option<GameConfig>,
        pub invitations: Option<Vec<Invitation>>,
        pub local_member: Option<Member>,
        pub members: Option<Vec<Member>>,
        pub party_id: Option<String>,
        pub party_type: Option<String>,
        pub restrictions: Option<Vec<Restriction>>,
        pub warnings: Option<Vec<Warning>>,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct GameConfig {
        pub allowable_premade_sizes: Option<Vec<i32>>,
        pub custom_lobby_name: Option<String>,
        pub custom_mutator_name: Option<String>,
        pub custom_rewards_disabled_reasons: Option<Vec<String>>,
        pub custom_spectator_policy: Option<String>,
        pub custom_spectators: Option<Vec<Member>>,
        pub custom_team_100: Option<Vec<Member>>,
        pub custom_team_200: Option<Vec<Member>>,
        pub game_mode: Option<String>,
        pub is_custom: Option<bool>,
        pub is_lobby_full: Option<bool>,
        pub is_team_builder_managed: Option<bool>,
        pub map_id: Option<i64>,
        pub max_human_players: Option<i64>,
        pub max_lobby_size: Option<i64>,
        pub max_team_size: Option<i64>,
        pub pick_type: Option<String>,
        pub premade_size_allowed: Option<bool>,
        pub queue_id: Option<i64>,
        pub show_position_selector: Option<bool>,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct Invitation {
        pub invitation_id: Option<String>,
        pub invitation_type: Option<String>,
        pub state: Option<String>,
        pub timestamp: Option<String>,
        pub to_summoner_id: Option<u64>,
        pub to_summoner_name: Option<String>,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct Restriction {
        pub expired_timestamp: Option<i64>,
        // pub restriction_args:
        pub restriction_code: Option<String>,
        pub summoner_ids: Option<Vec<i64>>,
        pub summoner_ids_string: Option<String>,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct Warning {
        pub expired_timestamp: Option<i64>,
        // pub restriction_args:
        pub restriction_code: Option<String>,
        pub summoner_ids: Option<Vec<i64>>,
        pub summoner_ids_string: Option<String>,
    }


    #[derive(Debug, Clone, Default)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct Member {
        pub allowed_change_activity: Option<bool>,
        pub allowed_invite_others: Option<bool>,
        pub allowed_kick_others: Option<bool>,
        pub allowed_start_activity: Option<bool>,
        pub allowed_toggle_invite: Option<bool>,
        pub auto_fill_eligible: Option<bool>,
        pub auto_fill_protected_promos: Option<bool>,
        pub auto_fill_protected_for_soloing: Option<bool>,
        pub auto_fill_protected_for_streaking: Option<bool>,
        pub bot_champion_id: Option<i32>,
        pub bot_difficulty: Option<String>,
        pub bot_id: Option<String>,
        pub first_position_preference: Option<Position>,
        pub second_position_preference: Option<Position>,
        pub is_bot: Option<bool>,
        pub is_leader: Option<bool>,
        pub is_spectator: Option<bool>,
        pub puuid: Option<String>,
        pub ready: Option<bool>,
        pub show_ghost_banner: Option<bool>,
        pub summoner_icon_id: Option<i32>,
        pub summoner_id: i64,
        pub summoner_internal_name: Option<String>,
        pub summoner_level: Option<i64>,
        pub summoner_name: Option<String>,
        pub team_id: Option<i32>,
    }

    #[derive(Debug, Clone, Default)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct PositionPreference {
        pub first_preference: Option<super::consts::Position>,
        pub second_preference: Option<super::consts::Position>,
    }
}

pub mod login {

}

pub mod ranked {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};
    use crate::consts::{Division, Queue};
    use crate::lcu::consts::Tier;

    // TODO: ADD REMAINING
    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct RankedStatus {
        pub queue_map: QueueMap,
        pub queues: Vec<Ranked>,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct QueueMap {
        #[serde(rename = "RANKED_SOLO_5x5")]
        pub ranked_solo: Ranked,
        #[serde(rename = "RANKED_FLEX_SR")]
        pub ranked_flex: Ranked,
        #[serde(rename = "RANKED_TFT")]
        pub ranked_tft: Ranked,
        #[serde(rename = "RANKED_TFT_PAIRS")]
        pub ranked_tft_pairs: Ranked,
        #[serde(rename = "RANKED_TFT_TURBO")]
        pub ranked_tft_turbo: Ranked,
    }

    // TODO: ADD REMAINING
    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    pub struct Ranked {
        pub division: Division,
        pub is_provisional: bool,
        pub league_points: i64,
        pub losses: i64,
        pub wins: i64,
        pub mini_series_progress: Option<String>,
        pub previous_season_achieved_division: Division,
        pub previous_season_achieved_tier: Tier,
        pub previous_season_end_division: Division,
        pub previous_season_end_tier: Tier,
        pub provisional_game_threshold: i32,
        pub provisional_games_remaining: i32,
        pub queue_type: Queue,
        pub rated_rating: i32,
        pub rated_tier: String,
        pub tier: Tier,
    }
}
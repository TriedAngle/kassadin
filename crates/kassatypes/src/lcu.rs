pub mod summoner {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};

    // TODO: I'm not sure whether account ID or summoner ID really are i64
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
        pub game_id: Option<String>,
        pub game_mode: Option<String>,
        pub game_queue_type: Option<String>,
        pub game_status: Option<String>,
        pub icon_override: Option<String>,
        pub is_observable: Option<String>,
        pub level: Option<String>,
        pub map_id: Option<String>,
        pub mastery_score: Option<String>,
        pub pty: Option<String>,
        pub puuid: Option<String>,
        pub queue_id: Option<String>,
        pub ranked_league_division: Option<String>,
        pub ranked_league_queue: Option<String>,
        pub ranked_league_tier: Option<String>,
        pub ranked_losses: Option<String>,
        pub ranked_split_reward_level: Option<String>,
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

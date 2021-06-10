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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ChampSelectSummoner {
    acting_background_animation_state: String,
    active_action_type: String,
    are_summoner_actions_complete: bool,
    assigned_position: String,
    ban_intent_square_portrat_path: String,
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

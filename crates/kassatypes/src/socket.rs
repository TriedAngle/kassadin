#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};
use crate::lcu::chat::LoL;
use crate::lcu::consts::QueueId;

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
pub enum EventType {
    Create,
    Update,
    Delete,
    #[cfg_attr(feature = "serde", serde(other))]
    Other,
}

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
pub enum SearchState {
    Searching,
    Canceled,
    #[serde(rename(deserialize = "Found", serialize = "Found"))]
    Found,
    #[cfg_attr(feature = "serde", serde(other))]
    Other,
}

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
pub enum GameFlowPhase {
    Lobby,
    Matchmaking,
    ReadyCheck,
    ChampSelect,
    GameStart,
    InProgress,
    PreEndOfGame,
    EndOfGame,
    WaitingForStats,
    TerminatedInError,
    None,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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
pub enum Availability {
    Chat,
    Away,
    Dnd,
    Offline,
}

pub type LobbyEvent = super::lcu::lobby::Lobby;
pub type FriendEvent = super::lcu::chat::Friend;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum LeagueEventKind {
    Queue(Option<QueueEvent>),
    Lobby(Option<LobbyEvent>),
    Friend(Option<FriendEvent>),
    GameFlow(Option<GameFlowEvent>),
    // TODO: add content
    ChampSelect
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LeagueEvent {
    pub kind: Option<LeagueEventKind>,
    pub event_type: Option<EventType>,
    pub uri: Option<String>,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct GameFlowEvent {
    #[serde(skip)]
    pub game_client: Option<GameClientData>,
    pub game_data: Option<GameData>,
    pub game_dodge: Option<DodgeData>,
    pub map: Option<GameFlowMapData>,
    pub phase: Option<GameFlowPhase>,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct GameFlowMapData {
    #[serde(skip)]
    pub categorized_content_bundles: Option<ContentBundles>,
    pub description: Option<String>,
    pub game_mode: Option<String>,
    pub game_mode_name: Option<String>,
    pub game_mode_short_name: Option<String>,
    pub game_mutator: Option<String>,
    pub id: Option<i32>,
    #[serde(rename = "isRGM")]
    pub is_rgm: Option<bool>,
    pub map_string_id: Option<String>,
    pub platform_id: Option<String>,
    pub platform_name: Option<String>,
    #[serde(skip)]
    pub properties: Option<GameFlowProperties>,
    pub suppress_runes_masteries_perks: Option<bool>,
}
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct GameData {
    pub game_id: Option<i64>,
    pub game_name: Option<String>,
    pub is_custom_game: Option<bool>,
    pub password: Option<String>,
    #[serde(skip)]
    pub player_champion_selections: Option<PlayerChampionSelections>,
    pub queue: Option<QueueData>,
    pub spectators_allowed: Option<bool>,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct QueueData {
    pub allowable_premade_sizes: Option<Vec<i32>>,
    pub are_free_champions_allowed: Option<bool>,
    pub asset_mutator: Option<String>,
    pub category: Option<String>,
    pub champions_required_to_play: Option<i32>,
    pub description: Option<String>,
    pub game_mode: Option<String>,
    #[serde(skip)]
    pub game_type_config: Option<GameTypeConfig>,
    pub id: Option<i64>,
    pub is_ranked: Option<bool>,
    pub is_team_builder_managed: Option<bool>,
    pub is_team_only: Option<bool>,
    pub last_toggled_off_time: Option<i64>,
    pub last_toggled_on_time: Option<i64>,
    pub map_id: Option<i64>,
    pub max_level: Option<i64>,
    pub max_summoner_level_for_first_win_of_the_day: Option<i64>,
    pub maximum_participant_list_size: Option<i32>,
    pub min_level: Option<i64>,
    pub minimum_participant_list_size: Option<i32>,
    pub name: Option<String>,
    pub num_players_per_team: Option<i64>,
    pub queue_availability: Option<String>,
    #[serde(skip)]
    pub queue_rewards: Option<QueueRewards>,
    pub removal_from_game_allowed: Option<bool>,
    pub removal_from_game_delay_minutes: Option<i64>,
    pub short_name: Option<String>,
    pub show_position_selector: Option<bool>,
    pub spectator_enabled: Option<bool>,
    #[serde(rename = "type")]
    pub queue_type: Option<String>
}



#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct GameClientData {

}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct GameTypeConfig {

}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct QueueRewards {

}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ContentBundles {

}
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct GameFlowProperties {

}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PlayerChampionSelections {

}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct QueueEvent {
    pub dodge_data: Option<DodgeData>,
    pub errors: Option<Vec<String>>,
    pub estimated_queue_time: Option<f64>,
    pub is_currently_in_queue: Option<bool>,
    pub lobby_id: Option<String>,
    pub low_priority_data: Option<LowPriorityData>,
    pub queue_id: Option<QueueId>,
    pub ready_check: Option<ReadyCheck>,
    pub search_state: Option<SearchState>,
    pub time_in_queue: Option<f64>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct DodgeData {
    pub dodger_id: Option<i64>,
    pub phase: Option<String>,
    pub state: Option<String>,
}


#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LowPriorityData {
    pub busted_leaver_access_token: Option<String>,
    pub penalized_summoner_ids: Option<Vec<i64>>,
    pub penalty_time: Option<f64>,
    pub penalty_time_remaining: Option<f64>,
    pub reason: Option<String>
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ReadyCheck {
    pub decliner_ids: Option<Vec<i64>>,
    pub dodge_warning: Option<String>,
    pub player_response: Option<String>,
    pub state: Option<String>,
    pub suppress_ux: Option<bool>,
    pub timer: Option<f64>,
}
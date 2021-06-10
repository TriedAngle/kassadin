pub mod champ_select {
    pub const SUMMONER_BY_SLOT: &str = "/lol-champ-select/v1/summoners/{slotId}";
}

pub mod summoner_lcu {
    pub const SUMMONER_BY_ID: &str = "/lol-summoner/v1/summoners/{id}";
    pub const CURRENT: &str = "/lol-summoner/v1/current-summoner";
}

pub mod chat {
    pub const ME: &str = "/lol-chat/v1/me";
}

pub mod end_of_game {
    pub const REPORTED_PLAYERS: &str = "/lol-end-of-game/v1/reported-players";
}

pub mod lobby {
    pub const PARTY_PLAYER: &str = "/lol-lobby/v1/parties/player";
    pub const MEMBERS: &str = "/lol-lobby/v2/lobby/members";
}

pub mod game_flow {
    pub const BASE: &str = "/lol-gameflow";
    pub const SESSION: &str = "/lol-gameflow/v1/session";
}

pub mod champ_select {
    pub const BASE: &str = "/lol-champ-select/v1";
    pub const SUMMONER_BY_SLOT: &str = "/lol-champ-select/v1/summoners/{slotId}";       // GET
}

pub mod summoner_lcu {
    pub const BASE: &str = "/lol-summoner/v1";
    pub const SUMMONER_BY_ID: &str = "/lol-summoner/v1/summoners/{id}";                 // GET
    pub const CURRENT: &str = "/lol-summoner/v1/current-summoner";                      // GET
}

pub mod chat {
    pub const BASE: &str = "/lol-chat/v1";
    pub const FRIENDS: &str = "/lol-chat/v1/friends";                                   // GET
    pub const FRIENDS_BY_ID: &str = "/lol-chat/v1/friends/{id}";                        // GET
    pub const ME: &str = "/lol-chat/v1/me";                                             // GET, POST
}

pub mod end_of_game {
    pub const BASE: &str = "/lol-end-of-game/v1";
    pub const REPORTED_PLAYERS: &str = "/lol-end-of-game/v1/reported-players";          // GET
}

pub mod lobby {
    pub const BASE: &str = "/lol-lobby/v2";
    pub const MEMBERS: &str = "/lol-lobby/v2/lobby/members";                            // GET
    pub const INVITATIONS: &str = "/lol-lobby/v2/lobby/invitations";                    // GET, POST
    pub const MEMBERS_KICK: &str = "/lol-lobby/v2/lobby/members/{id}/kick";
    pub const MEMBERS_GRANT_INVITE: &str = "/lol-lobby/v2/lobby/members/{id}/grant-invite";
    pub const MEMBERS_PROMOTE: &str = "/lol-lobby/v2/lobby/members/{id}/promote";
    pub const SEARCH: &str = "/lol-lobby/v2/lobby/matchmaking/search";                  // POST, DELETE
    pub const LOBBY: &str = "/lol-lobby/v2/lobby";                                      // GET, DELETE, POST
    pub const POSITION_PREF: &str =
    "/lol-lobby/v2/lobby/members/localMember/position-preferences";                     // PUT



    pub mod team_builder {
        pub const BASE: &str = "/lol-lobby-team-builder/v1";
        pub const MATCHMAKING: &str = "/lol-lobby-team-builder/v1/matchmaking";
    }
}

pub mod login {
    pub const BASE: &str = "/lol-login/v1";
    pub const DODGE_LOBBY: &str = "/lol-login/v1/session/invoke?destination=lcdsServiceProxy&method=call&args=[\"\", \"teambuilder-draft\",\"quitV2\", \"\"]";
}

pub mod matchmaking {
    pub const BASE: &str = "/lol-matchmaking/v1";
    pub const SEARCH: &str = "/lol-matchmaking/v1/search";
    pub const ACCEPT: &str = "/lol-matchmaking/v1/ready-check/accept";              // POST
    pub const DECLINE: &str = "/lol-matchmaking/v1/ready-check/decline";            // POST
}

pub mod riot {
    pub const BASE: &str = "/riotclient/v1";
    pub const LAUNCH_UX: &str = "/riotclient/launch-ux";
    pub const KILL_UX: &str = "/riotclient/kill-ux";
}

pub mod ranked {
    pub const BASE: &str = "/lol-ranked/v1";
    pub const STATS: &str = "/lol-ranked/v1/ranked-stats/{puuid}";
}
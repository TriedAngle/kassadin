use std::collections::HashMap;
use crate::{ChampSelect, Chat, EndOfGame, Lobby, Login, Matchmaking, Ranked, Riot, Summoner, TeamBuilder};
use anyhow::Result;
use reqwest::Response;

use kassaroutes as routes;
use kassatypes::lcu;
use kassatypes::lcu::ranked::RankedStatus;

impl<'a> ChampSelect<'a> {
    pub async fn summoner_by_slot(
        &self,
        id: i32,
    ) -> Result<lcu::champ_select::ChampSelectSummoner> {
        let url = routes::champ_select::SUMMONER_BY_SLOT.replace("{slotId}", &id.to_string());
        self.lcu.get(&url).await
    }
}

impl<'a> Summoner<'a> {
    pub async fn current(&self) -> Result<lcu::summoner::Summoner> {
        let url = routes::summoner_lcu::CURRENT;
        self.lcu.get(&url).await
    }

    pub async fn by_id(&self, id: &str) -> Result<lcu::summoner::Summoner> {
        let url = routes::summoner_lcu::SUMMONER_BY_ID.replace("{id}", id);
        self.lcu.get(&url).await
    }
}

impl<'a> Chat<'a> {
    pub async fn friends(&self) -> Result<Vec<lcu::chat::Friend>> {
        let url = routes::chat::FRIENDS;
        self.lcu.get(&url).await
    }

    pub async fn me(&self) -> Result<lcu::chat::Me> {
        let url = routes::chat::ME;
        self.lcu.get(&url).await
    }

    pub async fn me_post(&self, me: &lcu::chat::Me) -> Result<lcu::chat::Me> {
        let url = routes::chat::ME;
        self.lcu.post_with_data(&url, &me).await
    }
}

impl<'a> EndOfGame<'a> {}

impl<'a> Lobby<'a> {
    pub async fn join_lobby(&self, queue_id: lcu::consts::QueueId) -> Result<lcu::lobby::Lobby> {
        let url = routes::lobby::LOBBY;
        let mut body = HashMap::new();
        body.insert("queueId", queue_id as u32);
        self.lcu.post_with_data(url, &body).await
    }

    pub async fn leave_lobby(&self) -> Result<Response> {
        let url = routes::lobby::LOBBY;
        self.lcu.delete_raw(url).await
    }

    pub async fn start_queue(&self) -> Result<Response> {
        let url = routes::lobby::SEARCH;
        self.lcu.post_raw(url).await
    }

    pub async fn stop_queue(&self) -> Result<Response> {
        let url = routes::lobby::SEARCH;
        self.lcu.delete_raw(url).await
    }

    pub async fn set_roles(&self, roles: &lcu::lobby::PositionPreference) -> Result<Response> {
        let url = routes::lobby::POSITION_PREF;
        self.lcu.put_raw_with_data(url, roles).await
    }

    pub async fn kick_member(&self, member: i64) -> Result<Response> {
        let url = routes::lobby::MEMBERS_KICK.replace("{id}", &member.to_string());
        self.lcu.post_raw(&url).await
    }

    pub async fn promote_member(&self, member: i64) -> Result<Response> {
        let url = routes::lobby::MEMBERS_PROMOTE.replace("{id}", &member.to_string());
        self.lcu.post_raw(&url).await
    }

    pub async fn grant_invite_member(&self, member: i64) -> Result<Response> {
        let url = routes::lobby::MEMBERS_GRANT_INVITE.replace("{id}", &member.to_string());
        self.lcu.post_raw(&url).await
    }

    pub async fn invite_member(&self, summoner_id: i64) -> Result<Response> {
        let url = routes::lobby::INVITATIONS;
        let mut data = HashMap::new();
        data.insert("toSummonerId", summoner_id);
        let body = vec![data];
        self.lcu.post_raw_with_data(url, &body).await
    }

    pub fn team_builder(&self) -> TeamBuilder<'a> {
        TeamBuilder { lcu: self.lcu }
    }
}

impl<'a> TeamBuilder<'a> {

}

impl<'a> Matchmaking<'a> {
    pub async fn accept(&self) -> Result<Response> {
        let url = routes::matchmaking::ACCEPT;
        self.lcu.post_raw(url).await
    }

    pub async fn decline(&self) -> Result<Response> {
        let url = routes::matchmaking::DECLINE;
        self.lcu.post_raw(url).await
    }
}

impl<'a> Login<'a> {
    pub async fn dodge_lobby(&self) -> Result<Response> {
        let url = routes::login::DODGE_LOBBY;
        self.lcu.post_raw(url).await
    }
}

impl<'a> Riot<'a> {
    pub async fn launch_ux(&self) -> Result<Response> {
        let url = routes::riot::LAUNCH_UX;
        self.lcu.post_raw(url).await
    }

    pub async fn kill_ux(&self) -> Result<Response> {
        let url = routes::riot::KILL_UX;
        self.lcu.post_raw(url).await
    }
}

impl<'a> Ranked<'a> {
    pub async fn stats(&self, puuid: &String) -> Result<RankedStatus> {
        let url = routes::ranked::STATS.replace("{puuid}", puuid);
        self.lcu.get(&url).await
    }
}
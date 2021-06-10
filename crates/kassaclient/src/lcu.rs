use crate::{ChampSelect, Chat, EndOfGame, Lobby, Summoner};
use anyhow::Result;

use kassaroutes as routes;
use kassatypes::lcu;

impl<'a> ChampSelect<'a> {
    pub async fn summoner_by_slot(&self, id: i32) -> Result<lcu::ChampSelectSummoner> {
        let url = routes::champ_select::SUMMONER_BY_SLOT.replace("{slotId}", &id.to_string());
        self.lcu.simple_request(&url).await
    }
}

impl<'a> Summoner<'a> {
    pub async fn current(&self) -> Result<lcu::Summoner> {
        let url = routes::summoner_lcu::CURRENT;
        self.lcu.simple_request(&url).await
    }

    pub async fn by_id(&self, id: &str) -> Result<lcu::Summoner> {
        let url = routes::summoner_lcu::SUMMONER_BY_ID.replace("{id}", id);
        self.lcu.simple_request(&url).await
    }
}

impl<'a> Chat<'a> {
    //todo: implement this
    pub async fn me(&self) -> String {
        let url = routes::chat::ME;
        self.lcu
            .simple_request_raw(&url)
            .await
            .text()
            .await
            .unwrap()
    }
}

impl<'a> EndOfGame<'a> {}

impl<'a> Lobby<'a> {}

use crate::{ChampSelect, Chat, EndOfGame, Lobby, Summoner};
use anyhow::Result;

use kassaroutes as routes;
use kassatypes::lcu;

impl<'a> ChampSelect<'a> {
    pub async fn summoner_by_slot(
        &self,
        id: i32,
    ) -> Result<lcu::champ_select::ChampSelectSummoner> {
        let url = routes::champ_select::SUMMONER_BY_SLOT.replace("{slotId}", &id.to_string());
        self.lcu.simple_get(&url).await
    }
}

impl<'a> Summoner<'a> {
    pub async fn current(&self) -> Result<lcu::summoner::Summoner> {
        let url = routes::summoner_lcu::CURRENT;
        self.lcu.simple_get(&url).await
    }

    pub async fn by_id(&self, id: &str) -> Result<lcu::summoner::Summoner> {
        let url = routes::summoner_lcu::SUMMONER_BY_ID.replace("{id}", id);
        self.lcu.simple_get(&url).await
    }
}

impl<'a> Chat<'a> {
    //todo: implement this
    pub async fn me(&self) -> Result<lcu::chat::Me> {
        let url = routes::chat::ME;
        self.lcu.simple_get(&url).await
    }

    pub async fn me_post(&self, me: &lcu::chat::Me) -> Result<lcu::chat::Me> {
        let url = routes::chat::ME;
        self.lcu.simple_put(&url, &me).await
    }
}

impl<'a> EndOfGame<'a> {}

impl<'a> Lobby<'a> {}

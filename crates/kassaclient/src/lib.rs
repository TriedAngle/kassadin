#![allow(unused)]

mod lcu;

use anyhow::Result;
use kassatypes::consts::Region;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::str::FromStr;
use sysinfo::{ProcessExt, SystemExt};

#[derive(Debug, Clone)]
pub struct LCU {
    pub info: Option<ClientInfo>,
    pub requester: Client,
}

impl LCU {
    pub fn new() -> Self {
        Self {
            info: None,
            requester: Default::default(),
        }
    }

    pub async fn simple_request<T: DeserializeOwned + Debug>(&self, url: &str) -> Result<T> {
        let url = format!("https://localhost:{}{}", self.info().port.as_str(), url);

        let response = self.requester.get(url).send().await?;

        let parsed = response.json::<T>().await?;

        Ok(parsed)
    }

    pub fn find(&mut self) -> Result<&ClientInfo> {
        self.info = Some(ClientInfo::new()?);
        Ok(self.info())
    }

    pub fn info(&self) -> &ClientInfo {
        &self.info.as_ref().unwrap()
    }

    pub fn champ_select(&self) -> ChampSelect {
        ChampSelect { lcu: &self }
    }

    pub fn summoner(&self) -> Summoner {
        Summoner { lcu: &self }
    }
}

#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub pid: usize,
    pub path: String,
    pub token: String,
    pub port: String,
    pub region: Region,
    pub locale: String,
}

impl ClientInfo {
    pub fn new() -> Result<Self> {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();

        let mut client_pid = None;
        let mut client_process = None;
        for (pid, process) in system.get_processes() {
            if process.name() == "LeagueClient.exe" {
                client_pid = Some(pid);
                client_process = Some(process)
            }
        }

        let pid = client_pid.unwrap().clone();
        let process = client_process.unwrap();
        let env = process.cmd();

        let path = env[0].to_string();
        let mut token = None;
        let mut port = None;
        let mut region = None;
        let mut locale = None;

        for var in env {
            if var.starts_with("--riotclient-auth-token=") {
                token = Some(var.split("--riotclient-auth-token=").collect::<Vec<&str>>()[1])
            } else if var.starts_with("--riotclient-app-port=") {
                port = Some(var.split("--riotclient-app-port=").collect::<Vec<&str>>()[1])
            } else if var.starts_with("--region=") {
                let string_region = var.split("--region=").collect::<Vec<&str>>()[1];
                region = Some(Region::from_str(string_region)?)
            } else if var.starts_with("--locale=") {
                locale = Some(var.split("--locale=").collect::<Vec<&str>>()[1])
            }
        }

        Ok(Self {
            pid,
            path,
            token: token.unwrap().to_string(),
            port: port.unwrap().to_string(),
            region: region.unwrap(),
            locale: locale.unwrap().to_string(),
        })
    }
}

pub struct ChampSelect<'a> {
    lcu: &'a LCU,
}

pub struct Summoner<'a> {
    lcu: &'a LCU,
}

pub struct Chat<'a> {
    lcu: &'a LCU,
}

pub struct EndOfGame<'a> {
    lcu: &'a LCU,
}

pub struct Lobby<'a> {
    lcu: &'a LCU,
}

#[cfg(test)]
mod tests {
    use crate::LCU;

    #[test]
    fn create_lcu() {
        let mut client = LCU::new();
        let info = client.find().unwrap();
        println!("{:?}", info)
    }
}

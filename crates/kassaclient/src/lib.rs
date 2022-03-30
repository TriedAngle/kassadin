#![allow(unused)]

mod lcu;
mod socket;

use anyhow::Result;
use kassatypes::consts::Region;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Body, Client, ClientBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use native_tls::TlsStream;
use sysinfo::{ProcessExt, SystemExt};
use websocket::stream::sync::TcpStream;

// TODO: make client an option
// TODO: add logging
#[derive(Debug, Clone)]
pub struct LCU {
    pub info: Option<ClientInfo>,
    pub requester: Client,
}

impl LCU {
    pub fn new() -> Self {
        let mut lcu = Self {
            info: None,
            requester: ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        };
        let info = lcu.try_find();
        lcu
    }

    pub fn is_initialized(&self) -> bool {
        return self.info.is_some();
    }

    pub fn with_info(mut self, info: ClientInfo) -> Self {
        self.info = Some(info);
        self
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.get(url).send().await?;
        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }

    pub(crate) async fn get_with_body<T: DeserializeOwned, U: Serialize>(&self, url: &str, body: U) -> Result<T> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.get(url).json(&body).send().await?;
        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }

    pub(crate) async fn get_raw(&self, url: &str) -> Response {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let request = self.requester.get(url).send().await.unwrap();
        request
    }

    pub(crate) async fn post<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.post(url).send().await?;
        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }

    pub(crate) async fn post_raw(&self, url: &str) -> Result<Response> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.post(url).send().await?;
        Ok(response)
    }

    pub(crate) async fn post_raw_with_data< U: Serialize>(&self, url: &str, data: &U,) -> Result<Response> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.post(url).json(&data).send().await?;
        Ok(response)
    }

    pub(crate) async fn post_with_data<T: DeserializeOwned, U: Serialize>(
        &self,
        url: &str,
        data: &U,
    ) -> Result<T> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.post(url).json(&data).send().await?;
        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }

    pub(crate) async fn delete<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.delete(url).send().await?;
        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }

    pub(crate) async fn delete_raw(&self, url: &str) -> Result<Response> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.delete(url).send().await?;
        Ok(response)
    }

    pub(crate) async fn delete_with_data<T: DeserializeOwned, U: Serialize>(
        &self,
        url: &str,
        data: &U,
    ) -> Result<T> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.delete(url).json(&data).send().await?;
        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }


    pub(crate) async fn put<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.put(url).send().await?;
        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }

    pub(crate) async fn put_with_data<T: DeserializeOwned, U: Serialize>(
        &self,
        url: &str,
        data: &U,
    ) -> Result<T> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.put(url).json(data).send().await?;
        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }

    pub(crate) async fn put_raw(&self, url: &str) -> Result<Response> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.put(url).send().await?;
        Ok(response)
    }

    pub(crate) async fn put_raw_with_data<U: Serialize>(&self, url: &str, data: &U) -> Result<Response> {
        let url = format!("https://127.0.0.1:{}{}", self.info().port.as_str(), url);
        let response = self.requester.put(url).json(data).send().await?;
        Ok(response)
    }


    pub(crate) fn base64_auth(&self) -> String {
        let auth = format!("riot:{}", &self.info().token);
        base64::encode(auth)
    }

    pub fn is_client_running(&self) -> bool {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();

        for (pid, process) in system.get_processes() {
            if ["LeagueClientUx.exe", "LeagueClientUx"].contains(&process.name()) {
                return true;
            }
        }

        return false;
    }

    pub fn try_find(&mut self) -> Option<&ClientInfo> {
        let info = match ClientInfo::new() {
            Ok(info) => info,
            Err(e) => return None
        };

        self.info = Some(info);
        let base64_auth = self.base64_auth();

        let mut headers = HeaderMap::new();
        headers.append("Authorization", format!("Basic {}", base64_auth).parse().unwrap());

        self.requester = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .default_headers(headers)
            .build()
            .unwrap();

        Some(self.info())
    }

    pub fn find(&mut self) -> Result<&ClientInfo> {
        let info = ClientInfo::new()?;
        self.info = Some(info);
        let base64_auth = self.base64_auth();

        let mut headers = HeaderMap::new();
        headers.append("Authorization", format!("Basic {}", base64_auth).parse()?);

        self.requester = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .default_headers(headers)
            .build()
            .unwrap();

        Ok(self.info())
    }

    pub fn info(&self) -> &ClientInfo {
        &self.info.as_ref().unwrap()
    }

    pub(crate) fn socket_url_auth(&self) -> String {
        let info = self.info();
        let base64_auth = self.base64_auth();
        format!("wss://riot:{}@127.0.0.1:{}", info.token, info.port)
    }

    pub(crate) fn socket_url(&self) -> String {
        let info = self.info();
        format!("wss://127.0.0.1:{}", info.port)
    }

    pub fn socket(&self) -> WebSocketBuilder {
        WebSocketBuilder { lcu: self }
    }

    pub fn champ_select(&self) -> ChampSelect {
        ChampSelect { lcu: &self }
    }

    pub fn summoner(&self) -> Summoner {
        Summoner { lcu: &self }
    }

    pub fn chat(&self) -> Chat {
        Chat { lcu: &self }
    }

    pub fn lobby(&self) -> Lobby {
        Lobby { lcu: &self }
    }

    pub fn matchmaking(&self) -> Matchmaking { Matchmaking { lcu: &self } }

    pub fn login(&self) -> Login { Login { lcu: &self } }

    pub fn ranked(&self) -> Ranked { Ranked { lcu: &self } }
}

// TODO: add the other attributes
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
            if ["LeagueClientUx.exe", "LeagueClientUx"].contains(&process.name()) {
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
            if var.starts_with("--remoting-auth-token=") {
                token = Some(var.split("--remoting-auth-token=").collect::<Vec<&str>>()[1])
            } else if var.starts_with("--app-port=") {
                port = Some(var.split("--app-port=").collect::<Vec<&str>>()[1])
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

    pub fn launch_client_headless(path: &str) -> Result<Self> {
        Command::new(path)
            .args(["--mode unattended", "--headless"])
            .output();

        Self::new()
    }
}

pub struct WebSocketBuilder<'a> {
    lcu: &'a LCU,
}

pub struct WebSocket {
    pub client: websocket::sync::Client<TlsStream<TcpStream>>,
    pub lcu: LCU,
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

pub struct TeamBuilder<'a> {
    lcu: &'a LCU,
}

pub struct Matchmaking<'a> {
    lcu: &'a LCU,
}

pub struct Login<'a> {
    lcu: &'a LCU,
}

pub struct Riot<'a> {
    lcu: &'a LCU,
}

pub struct Ranked<'a> {
    lcu: &'a LCU,
}

#[cfg(test)]
mod tests {
    use crate::{ClientInfo, LCU};
    use std::io::Write;
    use serde_json::{Error, Value};
    use websocket::OwnedMessage;
    use kassatypes::socket::{EventType, LeagueEvent, LeagueEventKind, QueueEvent};
    use std::str::FromStr;
    use kassatypes::socket::LeagueEventKind::Queue;

    #[test]
    fn create_from_path() {
        let info = ClientInfo::launch_client_headless("C:\\Riot Games\\League of Legends\\LeagueClient.exe").expect("wrong path");
        let mut client = LCU::new().with_info(info);
        println!("{:?}", client.info);
    }

    #[test]
    fn create_lcu() {
        let mut client = LCU::new();
        let info = client.find().unwrap();
        println!("{:?}", info)
    }

    #[test]
    fn current_summoner() {
        let mut client = LCU::new();
        let info = client.find().unwrap();
        let current_summoner = tokio_test::block_on(client.summoner().current());

        println!("{:?}", current_summoner);
    }

    #[test]
    fn me() {
        let mut client = LCU::new();
        let info = client.find().unwrap();
        let me = tokio_test::block_on(client.chat().me());
        println!("{:?}", me);
    }

    #[test]
    fn me_set_me() {
        use kassatypes::lcu::chat::{LoL, Me};
        let mut client = LCU::new();
        let info = client.find().unwrap();

        let me_info = Me {
            status_message: Some("MEOW MEOW MEOW MEOW MEOW MEOW MEOW MEOW OWO UWU OWO UWU OWO UWU OWO UWU OWO UWU 😹 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 MEOW MEOW MEOW MEOW MEOW MEOW MEOW MEOW OWO UWU OWO UWU OWO UWU OWO UWU OWO UWU 😹 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 MEOW MEOW MEOW MEOW MEOW MEOW MEOW MEOW OWO UWU OWO UWU OWO UWU OWO UWU OWO UWU 😹 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 MEOW MEOW MEOW MEOW MEOW MEOW MEOW MEOW OWO UWU OWO UWU OWO UWU OWO UWU OWO UWU 😹 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺 😺".to_string()),
            ..Default::default()
        };

        let me = tokio_test::block_on(client.chat().me_post(&me_info));
        println!("{:?}", me);
    }

    #[test]
    fn friends() {
        let mut client = LCU::new();
        let info = client.find().unwrap();
        let friends = tokio_test::block_on(client.chat().friends());
        println!("{:?}", friends);
    }

    #[test]
    fn websocket() {
        use std::fs::OpenOptions;

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("wss.log")
            .unwrap();

        let start = std::time::SystemTime::now();
        let since_the_epoch = start
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");

        file.write(format!("---log {}---\n", since_the_epoch.as_secs().to_string()).as_bytes());

        let mut client = LCU::new();
        let mut websocket = client.socket().build();
        websocket.start();
        for message in websocket.client.incoming_messages() {
            let message = message.unwrap();
            match message {
                OwnedMessage::Text(message) => {
                    if let Ok(mut val) = serde_json::from_str::<Value>(&message) {
                        let mut event = match serde_json::from_value::<kassatypes::socket::LeagueEvent>(val[2].clone()) {
                            Ok(event) => event,
                            Err(_) => LeagueEvent::default(),
                        };

                        if val[2].get("uri").is_some() && val[2]["uri"] == kassaroutes::matchmaking::SEARCH {
                            event.kind = Some(Queue(None));
                            match serde_json::from_value::<kassatypes::socket::QueueEvent>(val[2]["data"].clone()) {
                                Ok(kind) => event.kind = Some(Queue(Some(kind))),
                                Err(_) => {}
                            }
                            // println!("event: {:?}", event);
                        } else if val[2].get("uri").is_some() && val[2]["uri"].to_string().contains(kassaroutes::lobby::BASE) {
                            file.write(format!("{:?}\n", val).as_bytes());
                        } else {
                            // file.write(format!("{:?}\n", val).as_bytes());
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

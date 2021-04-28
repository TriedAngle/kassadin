use crate::RequestClient;

pub struct RiotAPI {
    config: RiotAPIConfig,
    #[cfg(feature = "api")]
    api_client: RequestClient,
    #[cfg(feature = "lcu")]
    lcu_client: LCUClient,
}

pub struct LCUClient {}

pub struct RiotAPIConfig {}

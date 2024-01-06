use crate::{channels::ChannelApi, search::SearchApi, videos::VideoApi};

use reqwest::Client;

pub struct YouTubeDataApiHandler {
    api_key: String,
    client: Client,
}

impl YouTubeDataApiHandler {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub fn channels(&self) -> ChannelApi {
        ChannelApi::new(self.api_key.clone(), self.client.clone())
    }

    pub fn search(&self) -> SearchApi {
        SearchApi::new(self.api_key.clone(), self.client.clone())
    }

    pub fn videos(&self) -> VideoApi {
        VideoApi::new(self.api_key.clone(), self.client.clone())
    }
}

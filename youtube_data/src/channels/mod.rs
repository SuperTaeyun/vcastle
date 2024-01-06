use std::collections::HashMap;

use reqwest::Client;
use serde::Serialize;

pub mod model;

pub struct ChannelApi {
    api_key: String,
    client: Client,
}

impl ChannelApi {
    pub fn new(api_key: String, client: Client) -> Self {
        Self { api_key, client }
    }

    pub fn list(&self, part: Vec<ChannelPart>) -> ChannelList {
        ChannelList::new(&self.api_key, &self.client, part)
    }
}

pub struct ChannelList<'a> {
    api_key: &'a str,

    client: &'a Client,

    part: Vec<ChannelPart>,

    for_username: Option<String>,

    id: Option<String>,

    managed_by_me: Option<bool>,

    mine: Option<bool>,

    max_results: Option<u32>,
}

impl<'a> ChannelList<'a> {
    pub fn new(api_key: &'a str, client: &'a Client, part: Vec<ChannelPart>) -> Self {
        let part = if part.is_empty() {
            vec![ChannelPart::Id]
        } else {
            part
        };
        Self {
            api_key,
            client,
            part,
            for_username: None,
            id: None,
            managed_by_me: None,
            mine: None,
            max_results: None,
        }
    }

    pub fn part(&mut self, part: Vec<ChannelPart>) -> &mut Self {
        self.part = part;
        self
    }

    pub fn for_username(&mut self, for_username: impl Into<String>) -> &mut Self {
        self.for_username = Some(for_username.into());
        self
    }

    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id = Some(id.into());
        self
    }

    pub fn managed_by_me(&mut self, managed_by_me: bool) -> &mut Self {
        self.managed_by_me = Some(managed_by_me);
        self
    }

    pub fn mine(&mut self, mine: bool) -> &mut Self {
        self.mine = Some(mine);
        self
    }

    pub fn max_results(&mut self, max_results: u32) -> &mut Self {
        self.max_results = Some(max_results);
        self
    }

    pub async fn request(&self) -> model::ChannelListResponse {
        // create query
        let mut query = HashMap::<&str, &str>::new();

        // key
        query.insert("key", self.api_key);

        // part
        let part = self
            .part
            .iter()
            .map(|part| part.to_string())
            .collect::<Vec<String>>()
            .join(",");
        query.insert("part", &part);

        // for_username

        // id
        if let Some(id) = &self.id {
            query.insert("id", id);
        }

        // managed_by_me

        // mine

        // max_results
        self.client
            .get("https://www.googleapis.com/youtube/v3/channels")
            .query(&query)
            .send()
            .await
            .unwrap()
            .json::<model::ChannelListResponse>()
            .await
            .unwrap()
    }
}

#[derive(Debug, Serialize)]
pub enum ChannelPart {
    AuditDetails,
    BrandingSettings,
    ContentDetails,
    ContentOwnerDetails,
    Id,
    Localizations,
    Snippet,
    Statistics,
    Status,
    TopicDetails,
}

impl ToString for ChannelPart {
    fn to_string(&self) -> String {
        match self {
            ChannelPart::AuditDetails => "auditDetails",
            ChannelPart::BrandingSettings => "brandingSettings",
            ChannelPart::ContentDetails => "contentDetails",
            ChannelPart::ContentOwnerDetails => "contentOwnerDetails",
            ChannelPart::Id => "id",
            ChannelPart::Localizations => "localizations",
            ChannelPart::Snippet => "snippet",
            ChannelPart::Statistics => "statistics",
            ChannelPart::Status => "status",
            ChannelPart::TopicDetails => "topicDetails",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{get_develop_key, YouTubeDataApiHandler};

    use std::vec;

    #[tokio::test]
    async fn test_get_list() {
        let api_key = get_develop_key();
        let handler = YouTubeDataApiHandler::new(api_key);

        let response = handler
            .channels()
            .list(vec![ChannelPart::Snippet])
            .id("UCa9Y57gfeY0Zro_noHRVrnw")
            .request()
            .await;
        println!("{:?}", response);
        assert_eq!("UCa9Y57gfeY0Zro_noHRVrnw", response.items[0].id)
    }
}

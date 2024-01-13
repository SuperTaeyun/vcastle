pub mod model;

use crate::handler::YouTube;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone)]
pub struct ChannelsService {
    youtube: Box<YouTube>,
}

impl ChannelsService {
    pub fn new(youtube: Box<YouTube>) -> Self {
        Self { youtube }
    }

    pub fn list(&self, part: Vec<ChannelListPart>) -> ChannelList {
        ChannelList::new(&self, part)
    }
}

pub struct ChannelList<'a> {
    service: &'a ChannelsService,
    part: Vec<ChannelListPart>,
    for_username: Option<String>,
    id: Option<String>,
    managed_by_me: Option<bool>,
    mine: Option<bool>,
    max_results: Option<u32>,
}

impl<'a> ChannelList<'a> {
    pub fn new(service: &'a ChannelsService, part: Vec<ChannelListPart>) -> Self {
        let part = if part.is_empty() {
            vec![ChannelListPart::Id]
        } else {
            part
        };
        Self {
            service,
            part,
            for_username: None,
            id: None,
            managed_by_me: None,
            mine: None,
            max_results: None,
        }
    }

    pub fn part(&mut self, part: Vec<ChannelListPart>) -> &mut Self {
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
        let youtube = &self.service.youtube;
        query.insert("key", &youtube.api_key);

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
        self.service
            .youtube
            .client
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
pub enum ChannelListPart {
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

impl Display for ChannelListPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ChannelListPart::AuditDetails => "auditDetails",
            ChannelListPart::BrandingSettings => "brandingSettings",
            ChannelListPart::ContentDetails => "contentDetails",
            ChannelListPart::ContentOwnerDetails => "contentOwnerDetails",
            ChannelListPart::Id => "id",
            ChannelListPart::Localizations => "localizations",
            ChannelListPart::Snippet => "snippet",
            ChannelListPart::Statistics => "statistics",
            ChannelListPart::Status => "status",
            ChannelListPart::TopicDetails => "topicDetails",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_develop_key, YouTube};

    use std::vec;

    #[tokio::test]
    async fn test_get_list() {
        let api_key = get_develop_key();
        let handler = YouTube::new(api_key, None);

        let response = handler
            .channels()
            .list(vec![ChannelListPart::Snippet])
            .id("UCa9Y57gfeY0Zro_noHRVrnw")
            .request()
            .await;
        println!("{:?}", response);
        assert_eq!("UCa9Y57gfeY0Zro_noHRVrnw", response.items[0].id)
    }
}

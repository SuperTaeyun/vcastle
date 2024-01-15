use crate::{
    DataApi, Error, ListApi, ListResponse, Localization, Result, Thumbnail, ThumbnailKind, YouTube,
};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

pub type ChannelListResponse = ListResponse<ChannelListResource>;

#[derive(Clone)]
pub(crate) struct ChannelsService {
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

struct ChannelList<'a> {
    service: &'a ChannelsService,

    // required parameters
    part: Vec<ChannelListPart>,

    // filters (spcify exactly one of the following parameters)
    for_username: Option<&'a str>,
    id: Option<&'a str>,
    managed_by_me: Option<bool>,
    mine: Option<bool>,

    // optional parameters
    hl: Option<&'a str>,
    max_results: Option<u32>,
    on_behalf_of_content_owner: Option<&'a str>,
    page_token: Option<&'a str>,
}

impl DataApi for ChannelList<'_> {
    fn api_path(&self) -> &str {
        "/channels"
    }
}

#[async_trait]
impl ListApi<ChannelListResponse> for ChannelList<'_> {
    async fn request(&self) -> Result<ChannelListResponse> {
        let youtube = &self.service.youtube;

        // createquery parameter map
        let mut query_parameters = HashMap::<String, String>::new();

        // key
        self.insert_query_parameter(&mut query_parameters, "key", Some(&youtube.api_key));

        // required parameters
        self.insert_query_parameters(&mut query_parameters, "part", Some(&self.part));

        // filters
        if vec![
            self.for_username.is_none(),
            self.id.is_none(),
            self.managed_by_me.is_none(),
            self.mine.is_none(),
        ]
        .into_iter()
        .all(|v| v)
        {
            return Err(Error::InvalidParameter);
        }
        if let Some(for_username) = self.for_username {
            if vec![
                self.id.is_some(),
                self.managed_by_me.is_some(),
                self.mine.is_some(),
            ]
            .into_iter()
            .any(|v| v)
            {
                return Err(Error::InvalidParameter);
            }
            self.insert_query_parameter(&mut query_parameters, "forUsername", Some(for_username));
        }
        if let Some(id) = self.id {
            if vec![
                self.for_username.is_some(),
                self.managed_by_me.is_some(),
                self.mine.is_some(),
            ]
            .into_iter()
            .any(|v| v)
            {
                return Err(Error::InvalidParameter);
            }
            self.insert_query_parameter(&mut query_parameters, "id", Some(id));
        }
        // TODO: check if the user is authenticated
        if let Some(_managed_by_me) = self.managed_by_me {
            return Err(Error::NotAuthorized);
            // if vec![
            //     self.for_username.is_some(),
            //     self.id.is_some(),
            //     self.mine.is_some(),
            // ]
            // .into_iter()
            // .any(|v| v)
            // {
            //     return Err(Error::InvalidParameter);
            // }
            // self.insert_query_parameter(&mut query_parameters, "managedByMe", Some(managed_by_me));
        }
        if let Some(_mine) = self.mine {
            return Err(Error::NotAuthorized);
            // if vec![
            //     self.for_username.is_some(),
            //     self.id.is_some(),
            //     self.managed_by_me.is_some(),
            // ]
            // .into_iter()
            // .any(|v| v)
            // {
            //     return Err(Error::InvalidParameter);
            // }
            // self.insert_query_parameter(&mut query_parameters, "mine", Some(mine));
        }

        // optional parameters
        self.insert_query_parameter(&mut query_parameters, "hl", self.hl);
        self.insert_query_parameter(&mut query_parameters, "maxResults", self.max_results);
        self.insert_query_parameter(&mut query_parameters, "pageToken", self.page_token);

        // TODO: check if the user is authenticated
        if let Some(on_behalf_of_content_owner) = self.on_behalf_of_content_owner {
            if !on_behalf_of_content_owner.is_empty() {
                return Err(Error::NotAuthorized);
            }
        }

        Ok(youtube
            .client
            .get(youtube.base_path.to_owned() + self.api_path())
            .query(&query_parameters)
            .send()
            .await
            .unwrap()
            .json::<ChannelListResponse>()
            .await
            .unwrap())
    }
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
            hl: None,
            max_results: None,
            on_behalf_of_content_owner: None,
            page_token: None,
        }
    }

    pub fn part(&mut self, part: Vec<ChannelListPart>) -> &mut Self {
        self.part = part;
        self
    }

    pub fn for_username(&mut self, for_username: &'a str) -> &mut Self {
        self.for_username = Some(for_username);
        self
    }

    pub fn id(&mut self, id: &'a str) -> &mut Self {
        self.id = Some(id);
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

    pub fn hl(&mut self, hl: &'a str) -> &mut Self {
        self.hl = Some(hl);
        self
    }

    pub fn max_results(&mut self, max_results: u32) -> &mut Self {
        let max_results = if max_results > 50 { 50 } else { max_results };
        self.max_results = Some(max_results);
        self
    }

    pub fn on_behalf_of_content_owner(&mut self, on_behalf_of_content_owner: &'a str) -> &mut Self {
        self.on_behalf_of_content_owner = Some(on_behalf_of_content_owner);
        self
    }

    pub fn page_token(&mut self, page_token: &'a str) -> &mut Self {
        self.page_token = Some(page_token);
        self
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelListResource {
    /// Identifies the API resource's type. The value will be `youtube#channel`.
    pub kind: String,

    /// The ETag of the response.
    pub etag: String,

    /// The ID that YouTube uses to uniquely identify the channel.
    pub id: String,

    pub snippet: Option<ChannelSnippet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelSnippet {
    /// The channel's title.
    pub title: String,

    /// The channel's description. The property's value has a maximum length of 1000 characters.
    pub description: String,

    /// The channel's custom URL. The [YouTube Help Center](https://support.google.com/youtube/answer/2657968)
    /// explains eligibility requirements for getting a custom URL as well as how to set up the URL.
    #[serde(alias = "customUrl")]
    pub custom_url: Option<String>,

    /// The date and time that the channel was created. The value is specified in [ISO 8601](https://www.w3.org/TR/NOTE-datetime)
    /// format.
    #[serde(alias = "publishedAt")]
    pub published_at: DateTime<Utc>,

    /// A map of thumbnail images associated with the search result. For each object in the map, the key is the name of the
    /// thumbnail image, and the value is an object that contains other information about the thumbnail.
    ///
    /// When displaying thumbnails in your application, make sure that your code uses the image URLs exactly as they are
    /// returned in API responses. For example, your application should not use the http domain instead of the https domain
    /// in a URL returned in an API response.
    ///
    /// Channel thumbnail URLs are available only in the https domain, which is how the URLs appear in API responses.
    /// You might see broken images in your application if it tries to load YouTube images from the http domain.
    /// Thumbnail images might be empty for newly created channels and might take up to one day to populate.
    pub thumbnails: HashMap<ThumbnailKind, Thumbnail>,

    /// The language of the text in the channel resource's `snippet.title` and `snippet.description` properties.
    #[serde(alias = "defaultLanguage")]
    pub default_language: Option<String>,

    /// The snippet.localized object contains a localized title and description for the channel or it contains
    /// the channel's title and description in the [default_language][default_language] for the channel's metadata.
    ///
    /// - Localized text is returned in the resource snippet if the [channels.list][channels.list] request used the hl parameter to
    /// specify a language for which localized text should be returned, the hl parameter value identifies a
    /// [YouTube application language][YouTube application language], and localized text is available in that language.
    /// - Metadata for the default language is returned if an hl parameter value is not specified or a value is
    /// specified but localized metadata is not available for the specified language.
    ///
    /// The property contains a read-only value. Use the [localizations][localizations] object to add, update, or delete localized metadata.
    ///
    /// [default_language]:https://developers.google.com/youtube/v3/docs/channels#snippet.defaultLanguage
    /// [channels.list]:https://developers.google.com/youtube/v3/docs/channels/list
    /// [localizations]:https://developers.google.com/youtube/v3/docs/channels#localizations
    /// [YouTube application language]:https://developers.google.com/youtube/v3/docs/i18nLanguages
    pub localized: Option<Localization>,

    /// The country with which the channel is associated. To set this property's value, update the value of the
    /// [brandingSettings.channel.country](https://developers.google.com/youtube/v3/docs/channels#brandingSettings.channel.country)
    /// property.
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelContentDetails {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelStatistics {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelTopicDetails {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelStatus {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelBrandingSettings {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelAuditDetails {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelContentOwnerDetails {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_develop_key;

    #[tokio::test]
    async fn test_get_list_by_id() {
        let youtube = YouTube::new(get_develop_key(), None);

        let response = youtube
            .channels()
            .list(vec![ChannelListPart::Snippet])
            .id("UCa9Y57gfeY0Zro_noHRVrnw")
            .request()
            .await
            .unwrap();
        assert_eq!("UCa9Y57gfeY0Zro_noHRVrnw", response.items[0].id);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_get_list_without_filters() {
        let youtube = YouTube::new(get_develop_key(), None);
        let _response = youtube
            .channels()
            .list(vec![ChannelListPart::Snippet])
            .request()
            .await
            .unwrap();
    }
}

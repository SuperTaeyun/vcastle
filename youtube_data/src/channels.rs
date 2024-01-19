use crate::{
    error::{Error, Result},
    ListApi, ListResponse, Localization, RequestBase, Thumbnail, ThumbnailKind, YouTube,
    YouTubeDataApi,
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

    pub fn list(&self, part: Vec<Part>) -> ChannelList {
        ChannelList::new(&self, part)
    }
}

struct ChannelList<'a> {
    service: &'a ChannelsService,

    // required parameters
    part: Vec<Part>,

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

impl RequestBase for ChannelList<'_> {
    fn api_path(&self) -> &str {
        "channels"
    }
}

#[async_trait]
impl YouTubeDataApi for ChannelList<'_> {}

#[async_trait]
impl ListApi<ChannelListResponse> for ChannelList<'_> {
    async fn request(&self) -> Result<ChannelListResponse> {
        let youtube = &self.service.youtube;

        // createquery parameter map
        let mut params = HashMap::<String, String>::new();

        // key
        self.insert_query_parameter(&mut params, "key", Some(&youtube.api_key));

        // required parameters
        self.insert_query_parameters(&mut params, "part", Some(&self.part));

        // filter
        let filters = vec![
            self.for_username.is_some(),
            self.id.is_some(),
            self.managed_by_me.is_some(),
            self.mine.is_some(),
        ]
        .into_iter()
        .filter(|&v| v);
        let count = filters.clone().count();

        // filter must be specified exactly one
        if count == 1 {
            if let Some(for_username) = self.for_username {
                self.insert_query_parameter(&mut params, "forUsername", Some(for_username));
            }
            if let Some(id) = self.id {
                self.insert_query_parameter(&mut params, "id", Some(id));
            }
            // TODO: check if the user is authenticated
            if let Some(_managed_by_me) = self.managed_by_me {
                return Err(Error::authorization_required(
                    "The request uses the `managed_by_me` parameter but is not properly authorized",
                ));

                // self.insert_query_parameter(
                //     &mut query_parameters,
                //     "managedByMe",
                //     Some(managed_by_me),
                // );
            }
            if let Some(_mine) = self.mine {
                return Err(Error::authorization_required(
                    "The request uses the `mine` parameter but is not properly authorized",
                ));
                // self.insert_query_parameter(&mut query_parameters, "mine", Some(mine));
            }
        } else {
            return if count > 1 {
                let imcompatible_params = filters
                    .into_iter()
                    .enumerate()
                    .filter(|(_, v)| *v)
                    .map(|(i, _)| match i {
                        0 => "for_username",
                        1 => "id",
                        2 => "managed_by_me",
                        3 => "mine",
                        _ => "",
                    })
                    .collect::<Vec<&str>>()
                    .join(", ");
                Err(Error::incompatible_parameters(format!(
                    "Incompatible parameters specified in the request: {}",
                    imcompatible_params,
                )))
            } else {
                Err(Error::missing_required_parameter(
                    "No filter selected. Expected one of: for_username, id, managed_by_me, mine",
                ))
            };
        }

        // optional parameters
        self.insert_query_parameter(&mut params, "hl", self.hl);
        self.insert_query_parameter(&mut params, "maxResults", self.max_results);
        self.insert_query_parameter(&mut params, "pageToken", self.page_token);

        // TODO: check if the user is authenticated
        if let Some(on_behalf_of_content_owner) = self.on_behalf_of_content_owner {
            if !on_behalf_of_content_owner.is_empty() {
                return Err(Error::authorization_required(
                    "The request uses the `on_behalf_of_content_owner` parameter but is not properly authorized",
                ));
                // self.insert_query_parameter(
                //     &mut query_parameters,
                //     "onBehalfOfContentOwner",
                //     Some(on_behalf_of_content_owner),
                // );
            }
        }

        let response = self
            .send(
                youtube
                    .client
                    .get(self.url(&youtube.base_path))
                    .query(&params),
            )
            .await?;
        Ok(response.json().await?)
    }
}

impl<'a> ChannelList<'a> {
    pub fn new(service: &'a ChannelsService, part: Vec<Part>) -> Self {
        let part = if part.is_empty() {
            vec![Part::Id]
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

    pub fn part(&mut self, part: Vec<Part>) -> &mut Self {
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
pub enum Part {
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

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Part::AuditDetails => "auditDetails",
            Part::BrandingSettings => "brandingSettings",
            Part::ContentDetails => "contentDetails",
            Part::ContentOwnerDetails => "contentOwnerDetails",
            Part::Id => "id",
            Part::Localizations => "localizations",
            Part::Snippet => "snippet",
            Part::Statistics => "statistics",
            Part::Status => "status",
            Part::TopicDetails => "topicDetails",
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
    use crate::get_youtube_without_user_agent;

    #[tokio::test]
    async fn test_get_by_id() {
        let response = get_youtube_without_user_agent()
            .channels()
            .list(vec![Part::Snippet])
            .id("UCa9Y57gfeY0Zro_noHRVrnw")
            .request()
            .await;
        assert_eq!(true, response.is_ok());
        assert_eq!("UCa9Y57gfeY0Zro_noHRVrnw", response.unwrap().items[0].id);
    }

    #[tokio::test]
    async fn test_request_with_invalid_id() {
        let invalid_id = get_youtube_without_user_agent()
            .channels()
            .list(vec![Part::Snippet])
            .id("UC_x5XG1OV2P6uZZ5FSM9Ttw日本語한국어English")
            .request()
            .await;
        assert_eq!(true, invalid_id.is_err());
        let err = invalid_id.unwrap_err();
        let assert_message = concat!("client error for url (\"/youtube/v3/channels?id=UC_x5XG1OV2P6uZZ5FSM9Ttw日本語한국어English&key=[API_KEY]&part=snippet\"): ",
        "400 Bad Request status: \"INVALID_ARGUMENT\" ",
        "message: \"Request contains an invalid argument.\" ", 
        "[message: \"Request contains an invalid argument.\", domain: \"global\", reason: \"badRequest\"]");
        assert_eq!(assert_message, format!("{}", err));
    }

    #[tokio::test]
    async fn test_request_without_filters() {
        let without_filters = get_youtube_without_user_agent()
            .channels()
            .list(vec![])
            .request()
            .await;
        assert_eq!(true, without_filters.is_err());
        let err = without_filters.unwrap_err();
        assert_eq!(
            "builder error: \"No filter selected. Expected one of: for_username, id, managed_by_me, mine\"",
            format!("{}", err)
        );
    }

    #[tokio::test]
    async fn test_request_multiple_filters() {
        let multiple_filters = get_youtube_without_user_agent()
            .channels()
            .list(vec![])
            .id("something")
            .for_username("something")
            .request()
            .await;
        assert_eq!(true, multiple_filters.is_err());
        let err = multiple_filters.unwrap_err();
        assert_eq!(
            "builder error: \"Incompatible parameters specified in the request: for_username, id\"",
            format!("{}", err)
        );
    }

    /// test use filters that require authentication wihtout authentication
    #[tokio::test]
    async fn test_request_without_auth() {
        let without_auth = get_youtube_without_user_agent()
            .channels()
            .list(vec![])
            .mine(true)
            .request()
            .await;
        assert_eq!(true, without_auth.is_err());
        let err = without_auth.unwrap_err();
        assert_eq!(
            "builder error: \"The request uses the `mine` parameter but is not properly authorized\"",
            format!("{}", err)
        );
    }
}

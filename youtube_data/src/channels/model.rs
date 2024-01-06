use crate::model::{Localization, Thumbnail, ThumbnailKind};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

use super::{Thumbnail, ThumbnailKind};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Properties that appear in a search result.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchListResponse {
    /// Identifies the API resource's type. The value will be `youtube#searchListResponse`.
    pub kind: String,

    /// The ETag of the response.
    pub etag: String,

    /// The token that chan be used as the value of pageToken parameter to retrieve the next page in the result set.
    #[serde(alias = "nextPageToken")]
    pub next_page_token: Option<String>,

    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(alias = "prevPageToken")]
    pub prev_page_token: Option<String>,

    /// The region code that was used for the search query. The property value is a two-letter ISO country code that
    /// identifies the region. The [i18nRegions](https://developers.google.com/youtube/v3/docs/i18nRegions/list).
    /// list method returns a list of supported regions. The default value is US. if a non-supported region is specified,
    /// YouTube might still select another region, rather than the default value, to handle the query.
    #[serde(alias = "regionCode")]
    pub region_code: String,

    #[serde(alias = "pageInfo")]
    pub page_info: PageInfo,

    /// A list of results that match the search criteria.
    pub items: Vec<SearchListResource>,
}

/// The `pageInfo` object encapsulates paging information for the result set.
#[derive(Debug, Serialize, Deserialize)]
pub struct PageInfo {
    /// The total number of results in the result set. Please note that the value is an approximation and may not
    /// represent an exact value. In addition, the maximum value is 1,000,000.
    #[serde(alias = "totalResults")]
    pub total_results: i32,

    /// The number of results included in the API response.
    #[serde(alias = "resultsPerPage")]
    pub results_per_page: i32,
}

/// Structure shows the format of a search result.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchListResource {
    /// Identifies the API resource's type. The value will be `youtube#searchResult`.
    pub kind: String,

    /// The ETag of the response.
    pub etag: String,

    pub id: ResourceId,

    pub snippet: Option<SearchListSnippet>,
}

/// The id object contains information that can be used to uniquely identify the resource that matches the search
/// request.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    /// The type of the API resource.
    pub kind: String,

    /// If the `id.type` property's value is `youtube#video`, then this property will be present and its value will
    /// contain the ID that YouTube uses to uniquely identify a video that matches the search query.
    #[serde(alias = "videoId")]
    pub video_id: Option<String>,

    /// If the id.type property's value is youtube#channel, then this property will be present and its value will
    /// contain the ID that YouTube uses to uniquely identify a channel that matches the search query.
    #[serde(alias = "channelId")]
    pub channel_id: Option<String>,

    /// If the id.type property's value is youtube#playlist, then this property will be present and its value will
    /// contain the ID that YouTube uses to uniquely identify a playlist that matches the search query.
    #[serde(alias = "playlistId")]
    pub playlist_id: Option<String>,
}

/// The snippet object contains basic details about a search result, such as its title or description.
/// For example, if the search result is a video, then the title will be the video's title and the description
/// will be the video's description.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchListSnippet {
    /// The creation date and time of the resource that the search result identifies. The value is specified in
    /// [ISO 8601](https://www.w3.org/TR/NOTE-datetime) format.
    #[serde(alias = "publishedAt")]
    pub published_at: String,

    /// The value that YouTube uses to uniquely identify the channel that published the resource that the search result identifies.
    #[serde(alias = "channelId")]
    pub channel_id: String,

    /// The title of the search result.
    #[serde(default)]
    pub title: Option<String>,

    /// A description of the search result.
    #[serde(default)]
    pub description: Option<String>,

    /// A map of thumbnail images associated with the search result. For each object in the map, the key is the name of the
    /// thumbnail image, and the value is an object that contains other information about the thumbnail.
    pub thumbnails: HashMap<ThumbnailKind, Thumbnail>,

    /// The title of the channel that published the resource that the search result identifies.
    #[serde(alias = "channelTitle")]
    pub channel_title: String,

    /// An indication of whether a video or channel resource has live broadcast content. Valid property values are upcoming, live, and none.
    ///
    /// For a video resource, a value of upcoming indicates that the video is a live broadcast that has not yet started,
    /// while a value of live indicates that the video is an active live broadcast. For a channel resource, a value of upcoming
    /// indicates that the channel has a scheduled broadcast that has not yet started, while a value of live indicates that the channel
    /// has an active live broadcast.
    #[serde(alias = "liveBroadcastContent")]
    pub live_broadcast_content: Option<String>,
}

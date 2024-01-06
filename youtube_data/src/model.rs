use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse<T> {
    /// Identifies the API resource's type.
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
    pub region_code: Option<String>,

    #[serde(alias = "pageInfo")]
    pub page_info: PageInfo,

    /// A list of results that match the criteria.
    pub items: Vec<T>,
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

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ThumbnailKind {
    /// The default thumbnail image. The default thumbnail for a video – or a resource that refers to a video,
    /// such as a playlist item or search result – is 120px wide and 90px tall. The default thumbnail for a channel is
    /// 88px wide and 88px tall.
    #[serde(alias = "default")]
    Default,

    /// A higher resolution version of the thumbnail image. For a video (or a resource that refers to a video),
    /// this image is 320px wide and 180px tall. For a channel, this image is 240px wide and 240px tall.
    #[serde(alias = "medium")]
    Medium,

    /// A high resolution version of the thumbnail image. For a video (or a resource that refers to a video),
    /// this image is 480px wide and 360px tall. For a channel, this image is 800px wide and 800px tall.
    #[serde(alias = "high")]
    High,

    /// An even higher resolution version of the thumbnail image than the high resolution image.
    /// This image is available for some videos and other resources that refer to videos, like playlist items
    /// or search results. This image is 640px wide and 480px tall.
    #[serde(alias = "standard")]
    Standard,

    /// The highest resolution version of the thumbnail image. This image size is available for some videos and
    /// other resources that refer to videos, like playlist items or search results. This image is 1280px wide and 720px tall.
    #[serde(alias = "maxres")]
    Maxres,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    /// The image's URL.
    pub url: String,

    /// The image's width.
    pub width: Option<u32>,

    /// The image's height.
    pub height: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Localization {
    /// The localized description.
    pub description: String,

    /// The localized title.
    pub title: String,
}

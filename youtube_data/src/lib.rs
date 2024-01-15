use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod channels;
pub mod error;
pub mod search;
pub mod videos;

use channels::ChannelsService;
use error::Result;
use search::SearchService;
use videos::VideosService;

#[derive(Clone)]
pub struct YouTube {
    /// The API key used to authenticate requests to the YouTube Data API.
    pub(crate) api_key: String,

    /// The HTTP client used to make requests to the YouTube Data API.
    pub(crate) client: Client,

    /// The base path for the YouTube Data API.
    pub(crate) base_path: String,

    /// (optioanl) The user agent used to make requests to the YouTube Data API.
    pub(crate) user_agent: Option<String>,

    // services
    channels: Option<ChannelsService>,
    search: Option<SearchService>,
    videos: Option<VideosService>,
}

impl YouTube {
    pub fn new(api_key: String, user_agent: Option<String>) -> YouTube {
        let mut youtube = YouTube {
            api_key,
            client: Client::new(),
            base_path: "https://www.googleapis.com/youtube/v3".to_string(),
            user_agent,
            channels: None,
            videos: None,
            search: None,
        };

        // Initialize services
        youtube.channels = Some(ChannelsService::new(Box::new(youtube.clone())));
        youtube.search = Some(SearchService::new(Box::new(youtube.clone())));
        youtube.videos = Some(VideosService::new(Box::new(youtube.clone())));

        youtube
    }

    pub fn channels(&self) -> &ChannelsService {
        self.channels.as_ref().unwrap()
    }

    pub fn search(&self) -> &SearchService {
        self.search.as_ref().unwrap()
    }

    pub fn videos(&self) -> &VideosService {
        self.videos.as_ref().unwrap()
    }
}

/// The base trait for all YouTube Data CRUD APIs.
pub trait DataApi {
    fn api_path(&self) -> &str;

    fn insert_query_parameter(
        &self,
        map: &mut HashMap<String, String>,
        key: impl Into<String>,
        value: Option<impl ToString>,
    ) {
        if let Some(value) = value {
            let value = value.to_string();
            if !value.is_empty() {
                map.insert(key.into(), value);
            }
        }
    }

    fn insert_query_parameters(
        &self,
        map: &mut HashMap<String, String>,
        key: impl Into<String>,
        value: Option<Vec<impl ToString>>,
    ) {
        if let Some(value) = value {
            let value = value
                .into_iter()
                .filter(|v| !v.to_string().is_empty())
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",");
            if !value.is_empty() {
                map.insert(key.into(), value);
            }
        }
    }

    fn insert_date_time_query_parameter(
        &self,
        map: &mut HashMap<String, String>,
        key: impl Into<String>,
        value: Option<DateTime<Utc>>,
    ) {
        if let Some(value) = value {
            let value = value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
            if !value.is_empty() {
                map.insert(key.into(), value);
            }
        }
    }
}

#[async_trait]
pub trait ListApi<T>: DataApi
where
    T: serde::Serialize,
{
    async fn request(&self) -> Result<T>;
}

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

#[cfg(test)]
pub(crate) fn get_develop_key() -> String {
    use dotenv::dotenv;

    dotenv().ok();
    std::env::var("TEST_API_KEY").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::TimeZone;

    struct Test {}

    impl DataApi for Test {
        fn api_path(&self) -> &str {
            ""
        }
    }

    #[test]
    fn test_insert_query_parameter() {
        let test = Test {};
        let mut map = HashMap::<String, String>::new();

        // string
        test.insert_query_parameter(&mut map, "key1", Some("value"));
        // none
        test.insert_query_parameter(&mut map, "key2", None::<String>);
        // empty string
        test.insert_query_parameter(&mut map, "key3", Some(""));
        // bool
        test.insert_query_parameter(&mut map, "key4", Some(true));

        assert_eq!(map.get("key1").unwrap(), "value");
        assert_eq!(map.contains_key("key2"), false);
        assert_eq!(map.contains_key("key3"), false);
        assert_eq!(map.get("key4").unwrap(), "true");
    }

    #[test]
    fn test_insert_date_time_query_parameter() {
        let test = Test {};
        let mut map = HashMap::<String, String>::new();

        test.insert_date_time_query_parameter(
            &mut map,
            "key1",
            Some(Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap()),
        );

        assert_eq!(map.get("key1").unwrap(), "2021-01-01T00:00:00Z");
    }

    #[test]
    fn test_insert_query_parameters() {
        let test = Test {};
        let mut map = HashMap::<String, String>::new();

        test.insert_query_parameters(&mut map, "key1", Some(vec!["value1", "value2"]));

        assert_eq!(map.get("key1").unwrap(), "value1,value2");
    }
}

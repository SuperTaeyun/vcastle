use crate::{
    DataApi, Error, ListApi, ListResponse, Localization, Result, Thumbnail, ThumbnailKind, YouTube,
};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type VideoListResponse = ListResponse<VideoListResource>;

#[derive(Clone)]
pub(crate) struct VideosService {
    youtube: Box<YouTube>,
}

impl VideosService {
    pub fn new(youtube: Box<YouTube>) -> Self {
        Self { youtube }
    }

    pub fn list(&self, part: Vec<Part>) -> VideoList {
        VideoList::new(&self, part)
    }
}

struct VideoList<'a> {
    service: &'a VideosService,

    // required parameters
    part: Vec<Part>,

    // filters (specify exactly one of the following parameters)
    chart: Option<Chart>,
    id: Option<Vec<&'a str>>,
    my_rating: Option<MyRating>,

    // optional parameters
    hl: Option<&'a str>,
    max_height: Option<u32>,
    max_results: Option<u32>,
    max_width: Option<u32>,
    on_behalf_of_content_owner: Option<&'a str>,
    page_token: Option<&'a str>,
    region_code: Option<&'a str>,
    video_category_id: Option<&'a str>,
}

impl DataApi for VideoList<'_> {
    fn api_path(&self) -> &str {
        "videos"
    }
}

#[async_trait]
impl ListApi<VideoListResponse> for VideoList<'_> {
    async fn request(&self) -> Result<VideoListResponse> {
        let youtube = &self.service.youtube;

        // createquery parameter map
        let mut params = HashMap::<String, String>::new();

        // key
        self.insert_query_parameter(&mut params, "key", Some(&youtube.api_key));

        // required parameters
        self.insert_query_parameters(&mut params, "part", Some(&self.part));

        // filter
        let filters = vec![
            self.chart.is_some(),
            self.id.is_some(),
            self.my_rating.is_some(),
        ]
        .into_iter()
        .filter(|&x| x)
        .count();
        // filter must be specified exactly one
        if filters == 1 {
            if let Some(chart) = &self.chart {
                self.insert_query_parameter(&mut params, "chart", Some(chart));
                self.insert_query_parameter(&mut params, "videoCategoryId", self.video_category_id);
            }
            if let Some(id) = &self.id {
                if id.is_empty() {
                    return Err(Error::InvalidParameter);
                }
                self.insert_query_parameters(&mut params, "id", Some(&id));
            }
            // TODO: check if the user is authenticated
            if let Some(_my_rating) = &self.my_rating {
                return Err(Error::NotAuthorized);
                // self.insert_query_parameter(&mut params, "myRating", Some(my_rating));
            }
        } else {
            return if filters > 1 {
                // specify exactly one of the following parameters: chart, id, myRating
                Err(Error::InvalidParameter)
            } else {
                // specify one of the following parameters: chart, id, myRating
                Err(Error::InvalidParameter)
            };
        }

        // optional parameters
        self.insert_query_parameter(&mut params, "hl", self.hl);
        self.insert_query_parameter(&mut params, "maxHeight", self.max_height);
        self.insert_query_parameter(&mut params, "maxResults", self.max_results);
        self.insert_query_parameter(&mut params, "maxWidth", self.max_width);
        if self.on_behalf_of_content_owner.is_some() {
            // This parameter is intended exclusively for YouTube content partners.
            return Err(Error::NotAuthorized);
        }
        self.insert_query_parameter(&mut params, "pageToken", self.page_token);
        self.insert_query_parameter(&mut params, "regionCode", self.region_code);

        Ok(youtube
            .client
            .get(self.url(&youtube.base_path))
            .query(&params)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap())
    }
}

impl<'a> VideoList<'a> {
    pub fn new(service: &'a VideosService, part: Vec<Part>) -> Self {
        let part = if part.is_empty() {
            vec![Part::Id]
        } else {
            part
        };
        Self {
            service,
            part,
            chart: None,
            id: None,
            my_rating: None,
            hl: None,
            max_height: None,
            max_results: None,
            max_width: None,
            on_behalf_of_content_owner: None,
            page_token: None,
            region_code: None,
            video_category_id: None,
        }
    }

    pub fn part(&mut self, part: Vec<Part>) -> &mut Self {
        self.part = part;
        self
    }

    pub fn chart(&mut self, chart: Chart) -> &mut Self {
        self.chart = Some(chart);
        self
    }

    pub fn id(&mut self, id: Vec<&'a str>) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn my_rating(&mut self, my_rating: MyRating) -> &mut Self {
        self.my_rating = Some(my_rating);
        self
    }

    pub fn hl(&mut self, hl: &'a str) -> &mut Self {
        self.hl = Some(hl);
        self
    }

    pub fn max_height(&mut self, max_height: u32) -> &mut Self {
        let max_height = if max_height > 4320 {
            4320
        } else if max_height < 72 {
            72
        } else {
            max_height
        };
        self.max_height = Some(max_height);
        self
    }

    pub fn max_results(&mut self, max_results: u32) -> &mut Self {
        let max_results = if max_results > 50 { 50 } else { max_results };
        self.max_results = Some(max_results);
        self
    }

    pub fn max_width(&mut self, max_width: u32) -> &mut Self {
        let max_width = if max_width > 8192 {
            8192
        } else if max_width < 72 {
            72
        } else {
            max_width
        };
        self.max_width = Some(max_width);
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

    pub fn region_code(&mut self, region_code: &'a str) -> &mut Self {
        self.region_code = Some(region_code);
        self
    }

    pub fn video_category_id(&mut self, video_category_id: &'a str) -> &mut Self {
        self.video_category_id = Some(video_category_id);
        self
    }
}

pub enum Part {
    ContentDetails,

    FileDetails,

    Id,

    LiveStreamingDetails,

    Localizations,

    Player,

    ProcessingDetails,

    RecordingDetails,

    Snippet,

    Statistics,

    Status,

    Suggestions,

    TopicDetails,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let part = match self {
            Part::ContentDetails => "contentDetails",
            Part::FileDetails => "fileDetails",
            Part::Id => "id",
            Part::LiveStreamingDetails => "liveStreamingDetails",
            Part::Localizations => "localizations",
            Part::Player => "player",
            Part::ProcessingDetails => "processingDetails",
            Part::RecordingDetails => "recordingDetails",
            Part::Snippet => "snippet",
            Part::Statistics => "statistics",
            Part::Status => "status",
            Part::Suggestions => "suggestions",
            Part::TopicDetails => "topicDetails",
        };
        write!(f, "{}", part)
    }
}

pub enum Chart {
    /// Returns the most popular videos for the specified content region and video category.
    MostPopular,
}

impl std::fmt::Display for Chart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chart = match self {
            Chart::MostPopular => "mostPopular",
        };
        write!(f, "{}", chart)
    }
}

pub enum MyRating {
    /// Returns only videos disliked by the authenticated user.
    Dislike,

    /// Returns only videos liked by the authenticated user.
    Like,
}

impl std::fmt::Display for MyRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let my_rating = match self {
            MyRating::Dislike => "dislike",
            MyRating::Like => "like",
        };
        write!(f, "{}", my_rating)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoListResource {
    /// Identifies the API resource's type. The value will be `youtube#video`.
    pub kind: String,

    /// The ETag of the response.
    pub etag: String,

    /// The ID that YouTube uses to uniquely identify the video.
    pub id: String,

    pub snippet: Option<VideoSnippet>,

    #[serde(alias = "liveStreamingDetails")]
    pub live_streaming_details: Option<VideoLiveStreamingDetails>,
}

/// The snippet object contains basic details about the video, such as its title, description, and category.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoSnippet {
    /// The date and time that the video was published. Note that this time might be different than the time that
    /// the video was uploaded. For example, if a video is uploaded as a private video and then made public at a
    /// later time, this property will specify the time that the video was made public.
    ///
    /// There are a couple of special cases:
    ///
    /// * If a video is uploaded as a private video and the video metadata is retrieved by the channel owner,
    /// then the property value specifies the date and time that the video was uploaded.
    /// * If a video is uploaded as an unlisted video, the property value also specifies the date and time that
    /// the video was uploaded. In this case, anyone who knows the video's unique video ID can retrieve the video metadata.
    ///
    /// The value is specified in [ISO 8601](https://www.w3.org/TR/NOTE-datetime) format.
    #[serde(alias = "publishedAt")]
    pub published_at: DateTime<Utc>,

    /// The ID that YouTube uses to uniquely identify the channel that the video was uploaded to.
    #[serde(alias = "channelId")]
    pub channel_id: String,

    /// The video's title. The property value has a maximum length of 100 characters and may contain all valid
    /// UTF-8 characters except `<` and `>`. You must set a value for this property if you call the videos.update
    /// method and are updating the [snippet][VideoSnippet] part of a video resource.
    #[serde(default)]
    pub title: String,

    /// The video's description. The property value has a maximum length of 5000 bytes and may contain all
    /// valid UTF-8 characters except `<` and `>`.
    #[serde(default)]
    pub description: String,

    /// A map of thumbnail images associated with the video. For each object in the map, the key is the name
    /// of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    pub thumbnails: HashMap<ThumbnailKind, Thumbnail>,

    /// Channel title for the channel that the video belongs to.
    #[serde(alias = "channelTitle")]
    pub channel_title: String,

    /// A list of keyword tags associated with the video. Tags may contain spaces. The property value has a maximum
    /// length of 500 characters. Note the following rules regarding the way the character limit is calculated:
    ///
    /// * The property value is a list, and commas between items in the list count toward the limit.
    /// * If a tag contains a space, the API server handles the tag value as though it were wrapped in quotation marks,
    /// and the quotation marks count toward the character limit. So, for the purposes of character limits,
    /// the tag Foo-Baz contains seven characters, but the tag Foo Baz contains nine characters.
    pub tags: Option<Vec<String>>,

    /// The YouTube video category associated with the video. You must set a value for this property if you call
    /// the `videos.update` method and are updating the [snippet][VideoSnippet] part of a video resource.
    #[serde(alias = "categoryId")]
    pub category_id: Option<String>,

    /// Indicates if the video is an upcoming/active live broadcast. Or it's "none" if the video is not an
    /// upcoming/active live broadcast.
    ///
    /// Valid values for this property are:
    ///
    /// * live
    /// * none
    /// * upcoming
    #[serde(alias = "liveBroadcastContent")]
    pub live_broadcast_content: Option<String>,

    /// The language of the text in the channel resource's `snippet.title` and `snippet.description` properties.
    #[serde(alias = "defaultLanguage")]
    pub default_language: Option<String>,

    /// The snippet.localized object contains either a localized title and description for the video or the title in
    /// the `default_language` for the video's metadata.
    ///
    /// * Localized text is returned in the resource snippet if the videos.list request used the hl parameter to specify
    /// a language for which localized text should be returned and localized text is available in that language.
    /// * Metadata for the default language is returned if an hl parameter value is not specified or a value is specified
    /// but localized metadata is not available for the specified language.
    pub localized: Option<Localization>,

    /// The property contains a read-only value. Use the `localizations` object to add, update, or delete localized titles.
    #[serde(alias = "defaultAudioLanguage")]
    pub default_audio_language: Option<String>,
}

/// The contentDetails object contains information about the video content, including the length of the video and
/// an indication of whether captions are available for the video.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoContentDetail {}

/// The status object contains information about the video's uploading, processing, and privacy statuses.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoStatus {
    /// The status of the uploaded video.
    ///
    /// Valid values for this property are:
    ///
    /// * deleted
    /// * failed
    /// * processed
    /// * rejected
    /// * uploaded
    #[serde(alias = "uploadStatus")]
    pub upload_status: String,

    /// This value explains why a video failed to upload. This property is only present if the uploadStatus
    /// property indicates that the upload failed.
    ///
    /// Valid values for this property are:
    ///
    /// * codec
    /// * conversion
    /// * emptyFile
    /// * invalidFile
    /// * tooSmall
    /// * uploadAborted
    #[serde(alias = "failureReason")]
    pub failure_reason: Option<String>,

    /// This value explains why YouTube rejected an uploaded video. This property is only present if the uploadStatus
    /// property indicates that the upload was rejected.
    ///
    /// Valid values for this property are:
    ///
    /// * claim
    /// * copyright
    /// * duplicate
    /// * inappropriate
    /// * legal
    /// * length
    /// * termsOfUse
    /// * trademark
    /// * uploaderAccountClosed
    /// * uploaderAccountSuspended
    #[serde(alias = "rejectionReason")]
    pub rejection_reason: Option<String>,

    /// The video's privacy status.
    ///
    /// Valid values for this property are:
    ///
    /// * private
    /// * public
    /// * unlisted
    #[serde(alias = "privacyStatus")]
    pub privacy_status: Option<String>,

    /// The date and time when the video is scheduled to publish. It can be set only if the privacy status of the video
    /// is private. The value is specified in ISO 8601 format. Note the following two additional points about this
    /// property's behavior:
    ///
    /// * If you set this property's value when calling the videos.update method, you must also set the status.privacyStatus
    /// property value to private even if the video is already private.
    /// * If your request schedules a video to be published at some time in the past, the video will be published right away.
    /// As such, the effect of setting the status. `publishAt` property to a past date and time is the same as of changing
    /// the video's `privacyStatus` from private to public.
    #[serde(alias = "publishAt")]
    pub publish_at: Option<DateTime<Utc>>,

    /// The video's license.
    ///
    /// Valid values for this property are:
    ///
    /// * creativeCommon
    /// * youtube
    pub license: Option<String>,

    /// This value indicates whether the video can be embedded on another website.
    pub embeddable: bool,

    /// This value indicates whether the extended video statistics on the video's watch page are publicly viewable.
    /// By default, those statistics are viewable, and statistics like a video's view count and ratings will still be
    /// publicly visible even if this property's value is set to false.
    #[serde(alias = "publicStatsViewable")]
    pub public_stats_viewable: bool,

    /// This value indicates whether the video is designated as child-directed, and it contains the current "made for kids"
    /// status of the video. For example, the status might be determined based on the value of the selfDeclaredMadeForKids property.
    /// See the YouTube Help Center for more information about setting the audience for your channel, videos, or broadcasts.
    #[serde(alias = "madeForKids")]
    pub made_for_kids: bool,

    /// In a videos.insert or videos.update request, this property allows the channel owner to designate the video as being child-directed.
    /// In a videos.list request, the property value is only returned if the channel owner authorized the API request.
    #[serde(alias = "selfDeclaredMadeForKids")]
    pub self_declared_made_for_kids: Option<bool>,
}

/// The statistics object contains statistics about the video.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoStatistics {
    /// The number of times the video has been viewed.
    #[serde(alias = "viewCount")]
    pub view_count: Option<u64>,

    /// The number of users who have indicated that they liked the video.
    #[serde(alias = "likeCount")]
    pub like_count: Option<u64>,

    /// The number of users who have indicated that they disliked the video.
    ///
    /// Note: The statistics.dislikeCount property was made private as of December 13, 2021. This means that the property is included
    /// in an API response only if the API request was authenticated by the video owner. See the revision history for more information.
    #[serde(alias = "dislikeCount")]
    pub dislike_count: Option<u64>,

    /// Note: This property has been deprecated. The deprecation is effective as of August 28, 2015. The property's value is now always set to 0.
    #[serde(alias = "favoriteCount")]
    pub favorite_count: u64,

    /// The number of comments for the video.
    #[serde(alias = "commentCount")]
    pub comment_count: Option<u64>,
}

/// The player object contains information that you would use to play the video in an embedded player.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoPlayer {
    #[serde(alias = "embedHtml")]
    pub embed_html: Option<String>,

    #[serde(alias = "embedHeight")]
    pub embed_height: Option<i64>,

    #[serde(alias = "embedWidth")]
    pub embed_width: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoTopicDetails {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoRecordingDetails {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoFileDetails {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoProcessingDetails {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoSuggestions {}

/// The object contains metadata about a live video broadcast. The object will only be present in a
/// video resource if the video is an upcoming, live, or completed live broadcast.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoLiveStreamingDetails {
    /// The time that the broadcast actually started. The value is specified in ISO 8601 format.
    /// This value will not be available until the broadcast begins.
    #[serde(alias = "actualStartTime")]
    pub actual_start_time: Option<DateTime<Utc>>,

    /// The time that the broadcast actually ended. The value is specified in ISO 8601 format.
    /// This value will not be available until the broadcast is over.
    #[serde(alias = "actualEndTime")]
    pub actual_end_time: Option<DateTime<Utc>>,

    /// The time that the broadcast is scheduled to begin. The value is specified in ISO 8601 format.
    #[serde(alias = "scheduledStartTime")]
    pub scheduled_start_time: DateTime<Utc>,

    /// The time that the broadcast is scheduled to end. The value is specified in ISO 8601 format.
    /// If the value is empty or the property is not present, then the broadcast is scheduled to continue indefinitely.
    #[serde(alias = "scheduledEndTime")]
    pub scheduled_end_time: Option<DateTime<Utc>>,

    /// The number of viewers currently watching the broadcast. The property and its value will be present if the broadcast
    /// has current viewers and the broadcast owner has not hidden the view count for the video. Note that YouTube stops tracking
    /// the number of concurrent viewers for a broadcast when the broadcast ends. So, this property would not identify
    /// the number of viewers watching an archived video of a live broadcast that already ended.
    ///
    /// The concurrent viewer counts that the YouTube Data API returns might differ from the processed, despammed concurrent viewer
    /// counts available through YouTube Analytics. Learn more about live streaming metrics in the YouTube Help Center.
    #[serde(alias = "concurrentViewers")]
    pub concurrent_viewers: Option<u64>,

    /// The ID of the currently active live chat attached to this video. This field is filled only if the video is a currently
    /// live broadcast that has live chat. Once the broadcast transitions to complete this field will be removed and the
    /// live chat closed down. For persistent broadcasts that live chat id will no longer be tied to this video
    /// but rather to the new video being displayed at the persistent page.
    #[serde(alias = "activeLiveChatId")]
    pub active_live_chat_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_develop_key;

    #[tokio::test]
    #[should_panic]
    async fn test_request_without_filter() {
        let youtube = YouTube::new(get_develop_key(), None);

        let _ = youtube
            .videos()
            .list(vec![Part::Snippet])
            .request()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_request_multiple_filters() {
        let youtube = YouTube::new(get_develop_key(), None);

        let _ = youtube
            .videos()
            .list(vec![Part::Snippet])
            .chart(Chart::MostPopular)
            .id(vec!["Ks-_Mh1QhMc", "c0KYU2j0TM4", "eIho2S0ZahI"])
            .request()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_by_id() {
        let youtube = YouTube::new(get_develop_key(), None);

        let response = youtube
            .videos()
            .list(vec![Part::Snippet])
            .id(vec!["Ks-_Mh1QhMc"])
            .request()
            .await
            .unwrap();
        println!("{:#?}", response);

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].id, "Ks-_Mh1QhMc");
    }

    #[tokio::test]
    async fn test_get_by_multiple_ids() {
        let youtube = YouTube::new(get_develop_key(), None);

        let response = youtube
            .videos()
            .list(vec![Part::Snippet])
            .id(vec!["Ks-_Mh1QhMc", "c0KYU2j0TM4", "eIho2S0ZahI"])
            .request()
            .await
            .unwrap();
        println!("{:#?}", response);

        assert_eq!(response.items.len(), 3);
        assert_eq!(response.items[0].id, "Ks-_Mh1QhMc");
        assert_eq!(response.items[1].id, "c0KYU2j0TM4");
        assert_eq!(response.items[2].id, "eIho2S0ZahI");
    }

    #[tokio::test]
    async fn test_get_most_popular_videos() {
        let youtube = YouTube::new(get_develop_key(), None);

        let response = youtube
            .videos()
            .list(vec![Part::Snippet])
            .chart(Chart::MostPopular)
            .max_results(10)
            .request()
            .await
            .unwrap();
        println!("{:#?}", response);
        assert_eq!(response.items.len(), 10);
    }

    /// this test is ignored because live videos might be not "upcoming" or "active" at the time of testing
    #[ignore]
    #[tokio::test]
    async fn test_get_upcoming_live_videos() {
        let youtube = YouTube::new(get_develop_key(), None);

        let response = youtube
            .videos()
            .list(vec![Part::Snippet, Part::LiveStreamingDetails])
            .id(vec!["wPXfKeWU2YE"])
            .request()
            .await
            .unwrap();
        println!("{:#?}", response);

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].id, "wPXfKeWU2YE");
    }
}

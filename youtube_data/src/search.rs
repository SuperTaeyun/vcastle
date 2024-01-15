use crate::{DataApi, ListApi, ListResponse, Result, Thumbnail, ThumbnailKind, YouTube};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type SearchListResponse = ListResponse<SearchListResource>;

#[derive(Clone)]
pub(crate) struct SearchService {
    youtube: Box<YouTube>,
}

impl SearchService {
    pub fn new(youtube: Box<YouTube>) -> Self {
        Self { youtube }
    }

    pub fn list(&self, part: Vec<Part>) -> SearchList {
        SearchList::new(self, part)
    }
}

/// Parameters for the `list` method of the `search` api. details:
/// [link](https://developers.google.com/youtube/v3/docs/search/list)
struct SearchList<'a> {
    service: &'a SearchService,

    // required parameters
    part: Vec<Part>,

    // filters specify 0 or 1 of the following parameters
    for_content_owner: Option<bool>,
    for_developer: Option<bool>,
    for_mine: Option<bool>,

    // optional parameters
    channel_id: Option<&'a str>,
    channel_type: Option<ChannelType>,
    event_type: Option<EventType>,
    location: Option<&'a str>,
    location_radius: Option<&'a str>,
    max_results: Option<u32>,
    on_behalf_of_content_owner: Option<&'a str>,
    order: Option<Order>,
    page_token: Option<&'a str>,
    published_after: Option<DateTime<Utc>>,
    published_before: Option<DateTime<Utc>>,
    q: Option<&'a str>,
    region_code: Option<&'a str>,
    relevance_language: Option<&'a str>,
    safe_search: Option<SafeSearch>,
    topic_id: Option<&'a str>,
    /// The actual parameter name is `type`, but `type` is a keyword in Rust.
    resource_type: Option<Vec<ResourceType>>,
    video_caption: Option<VideoCaption>,
    video_category_id: Option<&'a str>,
    video_definition: Option<VideoDefinition>,
    video_dimension: Option<VideoDimension>,
    video_duration: Option<VideoDuration>,
    video_embeddable: Option<VideoEmbeddable>,
    video_license: Option<VideoLicense>,
    video_paid_product_placement: Option<VideoPaidProductPlacement>,
    video_syndicated: Option<VideoSyndicated>,
    video_type: Option<VideoType>,
}

impl DataApi for SearchList<'_> {
    fn api_path(&self) -> &str {
        "/search"
    }
}

#[async_trait]
impl ListApi<SearchListResponse> for SearchList<'_> {
    async fn request(&self) -> Result<SearchListResponse> {
        todo!()
    }
}

impl<'a> SearchList<'a> {
    pub fn new(service: &'a SearchService, part: Vec<Part>) -> Self {
        let part = if part.is_empty() {
            vec![Part::Snippet]
        } else {
            part
        };
        Self {
            service,
            part,
            for_content_owner: None,
            for_developer: None,
            for_mine: None,
            channel_id: None,
            channel_type: None,
            event_type: None,
            location: None,
            location_radius: None,
            max_results: None,
            on_behalf_of_content_owner: None,
            order: None,
            page_token: None,
            published_after: None,
            published_before: None,
            q: None,
            region_code: None,
            relevance_language: None,
            safe_search: None,
            topic_id: None,
            resource_type: None,
            video_caption: None,
            video_category_id: None,
            video_definition: None,
            video_dimension: None,
            video_duration: None,
            video_embeddable: None,
            video_license: None,
            video_paid_product_placement: None,
            video_syndicated: None,
            video_type: None,
        }
    }

    pub fn part(&mut self, part: Vec<Part>) -> &mut Self {
        self.part = part;
        self
    }

    pub fn for_content_owner(&mut self, for_content_owner: bool) -> &mut Self {
        self.for_content_owner = Some(for_content_owner);
        self
    }

    pub fn for_developer(&mut self, for_developer: bool) -> &mut Self {
        self.for_developer = Some(for_developer);
        self
    }

    pub fn for_mine(&mut self, for_mine: bool) -> &mut Self {
        self.for_mine = Some(for_mine);
        self
    }

    pub fn channel_id(&mut self, channel_id: &'a str) -> &mut Self {
        self.channel_id = Some(channel_id);
        self
    }

    pub fn channel_type(&mut self, channel_type: ChannelType) -> &mut Self {
        self.channel_type = Some(channel_type);
        self
    }

    pub fn event_type(&mut self, event_type: EventType) -> &mut Self {
        self.event_type = Some(event_type);
        self
    }

    pub fn location(&mut self, location: &'a str) -> &mut Self {
        self.location = Some(location);
        self
    }

    pub fn location_radius(&mut self, location_radius: &'a str) -> &mut Self {
        self.location_radius = Some(location_radius);
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

    pub fn order(&mut self, order: Order) -> &mut Self {
        self.order = Some(order);
        self
    }

    pub fn page_token(&mut self, page_token: &'a str) -> &mut Self {
        self.page_token = Some(page_token);
        self
    }

    pub fn published_after(&mut self, published_after: DateTime<Utc>) -> &mut Self {
        self.published_after = Some(published_after);
        self
    }

    pub fn published_before(&mut self, published_before: DateTime<Utc>) -> &mut Self {
        self.published_before = Some(published_before);
        self
    }

    pub fn q(&mut self, q: &'a str) -> &mut Self {
        self.q = Some(q);
        self
    }

    pub fn region_code(&mut self, region_code: &'a str) -> &mut Self {
        self.region_code = Some(region_code);
        self
    }

    pub fn relevance_language(&mut self, relevance_language: &'a str) -> &mut Self {
        self.relevance_language = Some(relevance_language);
        self
    }

    pub fn safe_search(&mut self, safe_search: SafeSearch) -> &mut Self {
        self.safe_search = Some(safe_search);
        self
    }

    pub fn topic_id(&mut self, topic_id: &'a str) -> &mut Self {
        self.topic_id = Some(topic_id);
        self
    }

    pub fn resource_type(&mut self, resource_type: Vec<ResourceType>) -> &mut Self {
        self.resource_type = Some(resource_type);
        self
    }

    pub fn video_caption(&mut self, video_caption: VideoCaption) -> &mut Self {
        self.video_caption = Some(video_caption);
        self
    }

    pub fn video_category_id(&mut self, video_category_id: &'a str) -> &mut Self {
        self.video_category_id = Some(video_category_id);
        self
    }

    pub fn video_definition(&mut self, video_definition: VideoDefinition) -> &mut Self {
        self.video_definition = Some(video_definition);
        self
    }

    pub fn video_dimension(&mut self, video_dimension: VideoDimension) -> &mut Self {
        self.video_dimension = Some(video_dimension);
        self
    }

    pub fn video_duration(&mut self, video_duration: VideoDuration) -> &mut Self {
        self.video_duration = Some(video_duration);
        self
    }

    pub fn video_embeddable(&mut self, video_embeddable: VideoEmbeddable) -> &mut Self {
        self.video_embeddable = Some(video_embeddable);
        self
    }

    pub fn video_license(&mut self, video_license: VideoLicense) -> &mut Self {
        self.video_license = Some(video_license);
        self
    }

    pub fn video_paid_product_placement(
        &mut self,
        video_paid_product_placement: VideoPaidProductPlacement,
    ) -> &mut Self {
        self.video_paid_product_placement = Some(video_paid_product_placement);
        self
    }

    pub fn video_syndicated(&mut self, video_syndicated: VideoSyndicated) -> &mut Self {
        self.video_syndicated = Some(video_syndicated);
        self
    }

    pub fn video_type(&mut self, video_type: VideoType) -> &mut Self {
        self.video_type = Some(video_type);
        self
    }
}

pub enum Part {
    Id,
    Snippet,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Part::Id => "id",
            Part::Snippet => "snippet",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

/// The channelType parameter lets you restrict a search to a particular type of channel.
pub enum ChannelType {
    /// Return all channels.
    Any,

    /// Only retrieve shows.
    Show,
}

impl std::fmt::Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ChannelType::Any => "any",
            ChannelType::Show => "show",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

/// The eventType parameter restricts a search to broadcast events. If you specify a value for this parameter,
/// you must also set the [type](ResourceType) parameter's value to video.
pub enum EventType {
    /// Only include completed broadcasts.
    Completed,

    /// Only include active broadcasts.
    Live,

    /// Only include upcoming broadcasts.
    Upcoming,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EventType::Completed => "completed",
            EventType::Live => "live",
            EventType::Upcoming => "upcoming",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub enum Order {
    /// Resources are sorted in reverse chronological order based on the date they were created.
    Date,

    /// Resources are sorted from highest to lowest rating.
    Rating,

    /// Resources are sorted based on their relevance to the search query. This is the default value for this parameter.
    Relevance,

    /// Resources are sorted alphabetically by title.
    Title,

    /// Channels are sorted in descending order of their number of uploaded videos.
    VideoCount,

    /// Resources are sorted from highest to lowest number of views. For live broadcasts, videos are sorted by number
    /// of concurrent viewers while the broadcasts are ongoing.
    ViewCount,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Order::Date => "date",
            Order::Rating => "rating",
            Order::Relevance => "relevance",
            Order::Title => "title",
            Order::VideoCount => "videoCount",
            Order::ViewCount => "viewCount",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub enum SafeSearch {
    /// YouTube will filter some content from search results and, at the least, will filter content that is restricted
    /// in your locale. Based on their content, search results could be removed from search results or demoted in
    /// search results. This is the default parameter value.
    Moderate,

    /// YouTube will not filter the search result set.
    None,

    /// YouTube will try to exclude all restricted content from the search result set. Based on their content,
    /// search results could be removed from search results or demoted in search results.
    Strict,
}

impl std::fmt::Display for SafeSearch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SafeSearch::Moderate => "moderate",
            SafeSearch::None => "none",
            SafeSearch::Strict => "strict",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

/// The type parameter restricts a search query to only retrieve a particular type of resource. The value is a
/// comma-separated list of resource types. The default value is `video,channel,playlist`.
pub enum ResourceType {
    Channel,

    Playlist,

    Video,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ResourceType::Channel => "channel",
            ResourceType::Playlist => "playlist",
            ResourceType::Video => "video",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub enum VideoCaption {
    /// Do not filter results based on caption availability.
    Any,

    /// Only include videos that have captions.
    ClosedCaption,

    /// Only include videos that do not have captions.
    None,
}

impl std::fmt::Display for VideoCaption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            VideoCaption::Any => "any",
            VideoCaption::ClosedCaption => "closedCaption",
            VideoCaption::None => "none",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub enum VideoDefinition {
    /// Return all videos, regardless of their resolution.
    Any,

    /// Only retrieve HD videos.
    High,

    /// Only retrieve videos in standard definition.
    Standard,
}

impl std::fmt::Display for VideoDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            VideoDefinition::Any => "any",
            VideoDefinition::High => "high",
            VideoDefinition::Standard => "standard",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub enum VideoDimension {
    /// Include both 3D and non-3D videos in returned results. This is the default value.
    Any,

    /// `2d`. Restrict search results to only include 3D videos.
    TwoDimensional,

    /// `3d`. Restrict search results to only include 3D videos.
    ThreeDimensional,
}

impl std::fmt::Display for VideoDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            VideoDimension::Any => "any",
            VideoDimension::TwoDimensional => "2d",
            VideoDimension::ThreeDimensional => "3d",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub enum VideoDuration {
    /// Do not filter video search results based on their duration. This is the default value.
    Any,

    /// Only include videos that are less than four minutes long in the result set.
    Short,

    /// Only include videos that are between four and 20 minutes long in the result set.
    Medium,

    /// Only include videos longer than 20 minutes in the result set.
    Long,
}

impl std::fmt::Display for VideoDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            VideoDuration::Any => "any",
            VideoDuration::Short => "short",
            VideoDuration::Medium => "medium",
            VideoDuration::Long => "long",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub enum VideoEmbeddable {
    /// Return all videos, embeddable or not.
    Any,

    /// Only retrieve embeddable videos.
    True,
}

impl std::fmt::Display for VideoEmbeddable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            VideoEmbeddable::Any => "any",
            VideoEmbeddable::True => "true",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub enum VideoLicense {
    /// Return all videos, regardless of which license they have, that match the query parameters.
    Any,

    /// Only return videos that have a Creative Commons license. Users can reuse videos with this license in other videos
    /// that they create. [Learn more](https://support.google.com/youtube/answer/2797468).
    CreativeCommon,

    /// Only return videos that have the standard YouTube license.
    Youtube,
}

impl std::fmt::Display for VideoLicense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            VideoLicense::Any => "any",
            VideoLicense::CreativeCommon => "creativeCommon",
            VideoLicense::Youtube => "youtube",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub enum VideoPaidProductPlacement {
    /// Return all videos, regardless of their paid product placement status.
    Any,

    /// Only retrieve videos that contain paid product placements.
    True,
}

pub enum VideoSyndicated {
    /// Return all syndicated videos.
    Any,

    /// Only retrieve syndicated videos.
    True,
}

impl std::fmt::Display for VideoSyndicated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            VideoSyndicated::Any => "any",
            VideoSyndicated::True => "true",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub enum VideoType {
    /// Return all videos.
    Any,

    /// Only retrieve episodes of shows.
    Episode,

    /// Only retrieve movies.
    Movie,
}

impl std::fmt::Display for VideoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            VideoType::Any => "any",
            VideoType::Episode => "episode",
            VideoType::Movie => "movie",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

/// Structure shows the format of a search result.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchListResource {
    /// Identifies the API resource's type. The value will be `youtube#searchResult`.
    pub kind: String,

    /// The ETag of the response.
    pub etag: String,

    pub id: ResourceId,

    pub snippet: Option<SearchSnippet>,
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
pub struct SearchSnippet {
    /// The creation date and time of the resource that the search result identifies. The value is specified in
    /// [ISO 8601](https://www.w3.org/TR/NOTE-datetime) format.
    #[serde(alias = "publishedAt")]
    pub published_at: DateTime<Utc>,

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

#[cfg(test)]
mod tests {}

use crate::{channels::ChannelsService, search::SearchService, videos::VideosService};

use reqwest::Client;

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

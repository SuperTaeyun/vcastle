use http::StatusCode;
use std::{error::Error as StdError, fmt};
use thiserror::Error;

/// A `Result` alias where the `Err` case is `youtube_data::Error`.
pub type Result<T> = std::result::Result<T, Error>;

pub type BoxError = Box<dyn StdError + Send + Sync>;

#[derive(Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorRepr);

impl From<reqwest::Error> for Error {
    #[cfg(not(debug_assertions))]
    fn from(value: reqwest::Error) -> Self {
        Error::new(ErrorKind::ReqwestError, Some(value.without_url()))
    }

    #[cfg(debug_assertions)]
    fn from(value: reqwest::Error) -> Self {
        Error::new(ErrorKind::ReqwestError, Some(value))
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("youtube_data::Error");

        // kind
        builder.field("kind", &self.0.kind);

        // source
        if let Some(source) = &self.0.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl Error {
    pub(crate) fn new<E>(kind: ErrorKind, source: Option<E>) -> Error
    where
        E: Into<BoxError>,
    {
        Error {
            0: ErrorRepr {
                kind,
                source: source.map(Into::into),
            },
        }
    }

    pub(crate) fn invalid_parameter(message: impl Into<String>) -> Error {
        Error::new(
            ErrorKind::BuilderError(BuilderErrorKind::InvalidParameter {
                message: message.into(),
            }),
            None::<Error>,
        )
    }

    pub(crate) fn incompatible_parameters(message: impl Into<String>) -> Error {
        Error::new(
            ErrorKind::BuilderError(BuilderErrorKind::IncompatibleParameters {
                message: message.into(),
            }),
            None::<Error>,
        )
    }

    pub(crate) fn missing_required_parameter(message: impl Into<String>) -> Error {
        Error::new(
            ErrorKind::BuilderError(BuilderErrorKind::MissingRequiredParameter {
                message: message.into(),
            }),
            None::<Error>,
        )
    }

    pub(crate) fn authorization_required(message: impl Into<String>) -> Error {
        Error::new(
            ErrorKind::BuilderError(BuilderErrorKind::AuthorizationRequired {
                message: message.into(),
            }),
            None::<Error>,
        )
    }

    pub(crate) fn client_error(source: YouTubeError) -> Error {
        Error::new(ErrorKind::ClientError, Some(source))
    }
}

#[derive(Debug, Error)]
struct ErrorRepr {
    kind: ErrorKind,
    source: Option<BoxError>,
}

impl fmt::Display for ErrorRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::BuilderError(kind) => {
                write!(f, "builder error: ")?;
                match kind {
                    BuilderErrorKind::InvalidParameter { message } => {
                        write!(f, "\"{}\"", message)?;
                    }
                    BuilderErrorKind::IncompatibleParameters { message } => {
                        write!(f, "\"{}\"", message)?;
                    }
                    BuilderErrorKind::MissingRequiredParameter { message } => {
                        write!(f, "\"{}\"", message)?;
                    }
                    BuilderErrorKind::AuthorizationRequired { message } => {
                        write!(f, "\"{}\"", message)?;
                    }
                }
            }
            ErrorKind::ClientError => {
                f.write_str("client error")?;
            }
            ErrorKind::ServerError => {
                f.write_str("server error")?;
            }
            ErrorKind::ReqwestError => {
                f.write_str("reqwest error")?;
            }
        }

        if let Some(e) = &self.source {
            write!(f, ": {}", e)?;
        }

        Ok(())
    }
}

/// Represents the kind of an error.
#[derive(Debug)]
pub(crate) enum ErrorKind {
    /// An error occurred while building the request.
    BuilderError(BuilderErrorKind),

    /// An client error occurred in the YouTube Data API.
    ClientError,

    /// An server error occurred in the YouTube Data API.
    ServerError,

    /// An error occurred in reqwest lib.
    ReqwestError,
}

/// Represents the error that occurred before the request was sent (request build process)
#[derive(Debug)]
pub(crate) enum BuilderErrorKind {
    /// The request specifies an invalid parameter value.
    InvalidParameter { message: String },

    /// The request specifies two or more parameters that cannot be used in the same request.
    IncompatibleParameters { message: String },

    /// The request is missing a required parameter.
    MissingRequiredParameter { message: String },

    /// The request uses parameters that require authentication but is not properly authorized.
    AuthorizationRequired { message: String },
}

/// Represents an error returned by the YouTube Data API.
#[derive(Error, serde::Deserialize)]
#[error(transparent)]
pub(crate) struct YouTubeError {
    error: YouTubeErrorRepr,
}

impl fmt::Debug for YouTubeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("youtube_data::YouTubeError");
        builder.field("error", &self.error);

        builder.finish()
    }
}

impl YouTubeError {
    pub(crate) fn code(&self) -> StatusCode {
        self.error.code
    }

    pub(crate) fn message(&self) -> &str {
        &self.error.message
    }

    pub(crate) fn errors(&self) -> &[YouTubeErrorDetail] {
        &self.error.errors
    }

    pub(crate) fn status(&self) -> Option<&str> {
        self.error.status.as_deref()
    }
}

#[derive(Debug, Error, serde::Deserialize)]
#[serde(tag = "error")]
struct YouTubeErrorRepr {
    #[serde(deserialize_with = "http_serde::status_code::deserialize")]
    code: StatusCode,
    message: String,
    errors: Vec<YouTubeErrorDetail>,
    status: Option<String>,
}

impl fmt::Display for YouTubeErrorRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.code)?;
        if let Some(status) = &self.status {
            write!(f, " status: \"{}\"", status)?;
        }
        write!(f, " message: \"{}\"", &self.message)?;
        f.write_str(" [")?;
        let details = &mut self.errors.iter();
        while let Some(detail) = details.next() {
            write!(f, "{}", detail)?;
            if details.next().is_some() {
                f.write_str(", ")?;
            }
        }
        f.write_str("]")?;

        Ok(())
    }
}

#[derive(Debug, Error, serde::Deserialize)]
struct YouTubeErrorDetail {
    message: String,
    domain: String,
    reason: String,
    location: Option<String>,
    #[serde(alias = "locationType")]
    location_type: Option<String>,
}

impl fmt::Display for YouTubeErrorDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "message: \"{}\"", &self.message)?;
        write!(f, ", domain: \"{}\"", &self.domain)?;
        write!(f, ", reason: \"{}\"", &self.reason)?;
        if let Some(location) = &self.location {
            write!(f, ", location: \"{}\"", location)?;
        }
        if let Some(location_type) = &self.location_type {
            write!(f, ", location_type: \"{}\"", location_type)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const YOUTUBE_ERROR_JSON: &str = r#"
    {
        "error":
        {
            "errors": [
                    {
                        "domain": "youtube.parameter",
                        "reason": "missingRequiredParameter",
                        "message": "No filter selected. Expected one of: for_username, id, managed_by_me, mine"
                    }
                ],
            "code": 400,
            "message": "No filter selected. Expected one of: for_username, id, managed_by_me, mine"
        }
    }"#;

    #[test]
    fn test_deserialize_youtube_error() {
        let error: YouTubeError = serde_json::from_str(YOUTUBE_ERROR_JSON).unwrap();
        assert_eq!(error.code(), StatusCode::BAD_REQUEST);

        println!("{:?}", error);
    }

    #[test]
    fn test_display_error() {
        let missing_param = Error::missing_required_parameter(
            "No filter selected. Expected one of: for_username, id, managed_by_me, mine",
        );
        assert_eq!(
            format!("{}", missing_param),
            "builder error: \"No filter selected. Expected one of: for_username, id, managed_by_me, mine\""
        );

        let client_error = Error::client_error(serde_json::from_str(YOUTUBE_ERROR_JSON).unwrap());
        let assert_message = concat!("client error: 400 Bad Request message: \"No filter selected.", 
        " Expected one of: for_username, id, managed_by_me, mine\"",
        " [message: \"No filter selected. Expected one of: for_username, id, managed_by_me, mine\",",
        " domain: \"youtube.parameter\", reason: \"missingRequiredParameter\"]"
        );
        assert_eq!(format!("{}", client_error), assert_message);
    }
}

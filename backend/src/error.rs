/*  #region      ------------------------------- [ IMPORTS ] ------------------------------------------  */

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

/*  #endregion   ------------------------------- [ IMPORTS ] ------------------------------------------  */

/*  #region      ------------------------------- [ Server Errors ] ------------------------------------------  */

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    // -- Auth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequest,

    // -- Model Errors
    TodoDeleteFailIdNotFound { id: u64 },
    TodoGetFailIdNotFound { id: u64 },
    TodoUpdateFailIdNotFound { id: u64 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // Create Placeholder response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert Error into Response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL), //FIXME: Replace Latter
            // -- Auth.
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequest => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            // -- Model.
            Self::TodoDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

/*  #endregion   ------------------------------- [ Server Errors ] ------------------------------------------  */

/*  #region      ------------------------------- [ Client Errors ] ------------------------------------------  */

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

/*  #endregion   ------------------------------- [ Client Errors ] ------------------------------------------  */

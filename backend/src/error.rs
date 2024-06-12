/*  #region      ------------------------------- [ IMPORTS ] ------------------------------------------  */

use axum::{http::StatusCode, response::{IntoResponse, Response}};

/*  #endregion   ------------------------------- [ IMPORTS ] ------------------------------------------  */

/*  #region      ------------------------------- [ Errors ] ------------------------------------------  */

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
 pub enum Error {
    LoginFail,
    // -- Model Errors
    TodoDeleteFailIdNotFound {id: u64},
    TodoGetFailIdNotFound {id: u64},
    TodoUpdateFailIdNotFound {id: u64},
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

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

/*  #endregion   ------------------------------- [ ERRORS ] ------------------------------------------  */

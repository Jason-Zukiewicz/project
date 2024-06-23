use std::time::{SystemTime, UNIX_EPOCH};

use crate::{ctx::Ctx, Error, Result};
use axum::http::{Method, Uri};
use chrono::{DateTime, Local, TimeZone, Utc};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&Error>,
    client_error: Option<crate::error::ClientError>,
) -> Result<()> {
    let timestamp = Local::now();

    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    // Create RequestLogLin
    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),

        // -- User & Ctx
        user_id: ctx.map(|c| c.user_id()),

        // -- http request attributes
        req_path: uri.to_string(),
        req_method: req_method.to_string(),

        // -- Errors attributes
        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data,
    };

    // FIXME: Production: send to cloud-watch log service
    println!("  ->> log_request: \n {}", json!(log_line));
    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String,

    // -- User & Ctx
    user_id: Option<u64>,

    // -- http request attributes
    req_path: String,
    req_method: String,

    // -- Errors attributes
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}

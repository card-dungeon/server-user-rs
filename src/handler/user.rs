use axum::{response::{IntoResponse, Redirect}, Json, http::StatusCode, extract::{State, Path, Query}};
use mongodb::{bson::{doc, to_document}, Database, Collection, options::FindOptions, options::UpdateOptions};
use mongodm::operator::*;
use tracing::info;
use serde_json::json;
use tokio_stream::StreamExt;
use serde::{Deserialize};

use std::collections::HashMap;

use crate::model::user::User;

// const KAKAO_API_KEY: String = std::env::var("KAKAO_API_KEY").unwrap();

#[derive(Debug, Deserialize)]
pub struct Querys {
    pub code: String,
}

pub async fn signin() -> impl IntoResponse {
    
}

pub async fn kakao_redirect(
    query: Option<Query<Querys>>,
) -> impl IntoResponse {
    let Query(query) = query.unwrap();

    (StatusCode::OK, Json({}))
}
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
    Redirect::permanent(&format!("https://kauth.kakao.com/oauth/authorize?client_id={}&redirect_uri={}&response_type=code", std::env::var("KAKAO_API_KEY").expect("no env"), "http://localhost:3000/user/redirect"))
}

pub async fn kakao_redirect(
    query: Option<Query<Querys>>,
) -> impl IntoResponse {
    let Query(query) = query.unwrap();

    (StatusCode::OK, Json({}))
}
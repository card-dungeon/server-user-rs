use serde::{Serialize, Deserialize};
use mongodb::bson;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub nickname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    // pub password: String,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub permalink_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub user: User,
    pub genre: Option<String>,
    pub permalink_url: String,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct PagedResponse {
    pub collection: Vec<Song>,
    pub next_href: Option<String>,
}

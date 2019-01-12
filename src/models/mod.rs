use super::schema::systems;
use serde_derive::*;

#[derive(Queryable, Serialize)]
pub struct Systems {
    pub id: i32,
    pub sysnr: String,
    pub descr: String,
}
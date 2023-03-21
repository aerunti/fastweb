pub mod user;
use sqlx::{Pool,Postgres};
use tera::Tera;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub tera: Tera,
}
pub mod data;
pub mod domain;
pub mod service;
pub mod web;

pub use domain::clip::field::ShortCode;
pub use domain::clip::{Clip, ClipError};
pub use data::DataError;
pub use domain::time::Time;
pub use service::ServiceError;
use crate::data::AppDatabase;
use crate::web::renderer::Renderer;
use rocket::fs::FileServer;
use rocket::{Build, Rocket};

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<AppDatabase>(config.database)
        .manage::<Renderer>(config.renderer)
        .mount("/", web::http::routes()) // set up root route
        .mount("/static", FileServer::from("static"))
        .register("/", web::http::catcher::catchers())

}

pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub database: AppDatabase
}
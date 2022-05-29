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
use domain::maintenance::Maintenance;
use crate::web::hitcounter::HitCounter;

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<AppDatabase>(config.database)
        .manage::<Renderer>(config.renderer)
        .manage::<HitCounter>(config.hit_counter)
        .manage::<Maintenance>(config.maintenance)
        .mount("/", web::http::routes()) // set up root route
        .mount("/api/clip", web::api::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", web::http::catcher::catchers())
        .register("/api/clip", web::api::catcher::catchers())

}

pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub database: AppDatabase,
    pub hit_counter: HitCounter,
    pub maintenance: Maintenance
}
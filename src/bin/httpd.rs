use std::env::VarError;
use clipstash::data::AppDatabase;
use clipstash::web::{renderer::Renderer};
use dotenv::dotenv;
use std::path::PathBuf;
use rocket::tokio;
use structopt::StructOpt;
use clipstash::domain::maintenance::Maintenance;
use clipstash::web::hitcounter::HitCounter;

/// The binary that gets the webserver running.

#[derive(StructOpt, Debug)]
#[structopt(name = "httpd")]
struct Opt {
    #[structopt(default_value = "localhost:5432")]
    connection_string: String,
    #[structopt(short, long, parse(from_os_str), default_value = "templates/")]
    template_directory: PathBuf
}

fn main() {
    dotenv().ok();

    let DB_URL = std::env::var("DATABASE_URL")
        .ok()
        .unwrap_or("localhost:5432".to_owned());

    let opt = Opt {
        connection_string: DB_URL,
        template_directory: PathBuf::from("templates/")
    };

    let rt = tokio::runtime::Runtime::new()
        .expect("failed to spawn tokio runtime");

    let handle = rt.handle().clone();
    let renderer = Renderer::new(opt.template_directory.clone());

    let database = rt.block_on(async move {
        AppDatabase::new(&opt.connection_string).await
    });

    let hit_counter = HitCounter::new(database.get_pool().clone(), handle.clone());
    let maintenance = Maintenance::spawn(database.get_pool().clone(), handle.clone());

    let config = clipstash::RocketConfig {
        renderer,
        database,
        hit_counter,
        maintenance
    };

    rt.block_on(async move {
        clipstash::rocket(config)
            .launch()
            .await
            .expect("failed to launch rocket server")

    });
}
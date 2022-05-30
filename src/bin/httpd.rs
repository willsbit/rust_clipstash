use clipstash::data::AppDatabase;
use clipstash::web::{renderer::Renderer};
use dotenv::dotenv;
use std::path::PathBuf;
use rocket::tokio;
use structopt::StructOpt;
use clipstash::domain::maintenance::Maintenance;
use clipstash::web::hitcounter::HitCounter;
use clipstash::data::secret;
use clipstash::data::secret::DATABASE_URL;

const DB_URL: &str = DATABASE_URL;

#[derive(StructOpt, Debug)]
#[structopt(name = "httpd")]
struct Opt {
    #[structopt(default_value = DB_URL)]
    connection_string: String,
    #[structopt(short, long, parse(from_os_str), default_value = "templates/")]
    template_directory: PathBuf
}

fn main() {
    dotenv().ok();
    let opt = Opt::from_args();

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
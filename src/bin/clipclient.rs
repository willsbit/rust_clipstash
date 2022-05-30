use clipstash::domain::clip::field::{Content, Expires, Password, ShortCode, Title};
use clipstash::service::ask::{GetClip, NewClip, UpdateClip};
use clipstash::web::api::{ApiKey, API_KEY_HEADER};
use clipstash::Clip;
use std::error::Error;
use structopt::StructOpt;

/// ClipStash API client, which allows a user to read, write and modify clips from a command
/// line interface. To use, run [`cargo run --bin clipclient`]. One can also use the API through
/// requests which do not come from this client, but from other platforms such as Insomnia and
/// Postman.

#[derive(StructOpt, Debug)]
enum Command {
    Get {
        shortcode: ShortCode,
        #[structopt(short, long, help = "password")]
        password: Option<String>
    },
    New {
        #[structopt(help = "content")]
        clip: String,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
        #[structopt(short, long, help = "expiration date")]
        expires: Option<Expires>,
        #[structopt(short, long, help = "title")]
        title: Option<Title>
    },
    Update {
        shortcode: ShortCode,
        #[structopt(help = "content")]
        clip: String,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
        #[structopt(short, long, help = "expiration date")]
        expires: Option<Expires>,
        #[structopt(short, long, help = "title")]
        title: Option<Title>
    }
}


#[derive(StructOpt, Debug)]
#[structopt(name = "clipclient", about = "ClipStash API client")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(default_value = "http://127.0.0.1:8000", env = "CLIPSTASH_ADDR")]
    addr: String,

    #[structopt(long)]
    api_key:ApiKey
}


fn run(opt:Opt) -> Result<(), Box<dyn Error>> {
    match opt.command {
        Command::Get { shortcode, password } => {
            let req = GetClip {
                password: Password::new(password.unwrap_or_default())?,
                shortcode
            };
            let clip = get_clip(opt.addr.as_str(), req, opt.api_key)?;
            println!("{:#?}", clip);
            Ok(())
        },
        Command::New { clip, password, expires, title} => {
            let req = NewClip {
                content: Content::new(clip.as_str())?,
                title: title.unwrap_or_default(),
                expires: expires.unwrap_or_default(),
                password: password.unwrap_or_default()
            };

            let clip = new_clip(opt.addr.as_str(), req, opt.api_key)?;
            println!("{:#?}", clip);
            Ok(())
        },
        Command::Update { clip, password, expires, title, shortcode} => {
            let password = password.unwrap_or_default();
            let svc_get_req = GetClip {
                password: password.clone(),
                shortcode: shortcode.clone()
            };
            let original_clip = get_clip(opt.addr.as_str(), svc_get_req, opt.api_key.clone())?;
            let svc_update_req = UpdateClip {
                content: Content::new(clip.as_str())?,
                expires: expires.unwrap_or(original_clip.expires),
                title: title.unwrap_or(original_clip.title),
                password,
                shortcode
            };

            let clip = update_clip(opt.addr.as_str(), svc_update_req, opt.api_key);
            println!("{:#?}", clip);
            Ok(())
        }
    }
}


fn get_clip(addr: &str, ask_svc: GetClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip/{}", addr, ask_svc.shortcode.into_inner());
    let mut request = client.get(addr);
    request = match ask_svc.password.into_inner() {
        Some(password) => request.header(reqwest::header::COOKIE,
                                         format!("password={}", password)),
        None => request
    };

    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.send()?.json()?)
}

fn new_clip(addr: &str, ask_svc: NewClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip", addr);
    let mut request = client.post(addr);
    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.json(&ask_svc).send()?.json()?)
}

fn update_clip(addr: &str, ask_svc: UpdateClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip", addr);
    let mut request = client.put(addr);
    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.json(&ask_svc).send()?.json()?)
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("An error occurred: {}", e)
    }
}
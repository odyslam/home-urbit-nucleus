use actix_web::{middleware, web, App, HttpServer};
use api::*;
use std::{net::SocketAddrV4, str::FromStr};
use tracing::{debug, info};

use clap::Parser;
use cli::Opts;

mod api;
mod cli;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts = Opts::parse();
    match opts.clone() {
        Opts::StartServer { bind, logs, .. } => {
            std::env::set_var("RUST_LOG", logs.unwrap_or_else(|| "warn".to_string()));
            tracing_subscriber::fmt::init();
            debug!("{:?}", opts);
            info!("Hello Martian, welcome to Nucleus!");
            info!(
                r#"
         __
 _(\    |@@|
(__/\__ \--/ __
   \___|----|  |   __
     \ }}{{ /\ )_ / _\
       /\__/\ \__O (__
      (--/\--)    \__/
      _)(  )(_
     `---''---`
     "#
            );
            start_api(bind).await
        }
    }
}

async fn start_api(bind_or_none: Option<SocketAddrV4>) -> std::io::Result<()> {
    let bind = bind_or_none.unwrap_or_else(|| "127.0.0.1:6969".parse::<SocketAddrV4>().unwrap());
    info!("Binding API server to {}", bind);
    info!(
        r#"
--------------------
| Active Endpoints: |
--------------------
{}/                   : User Dashboard
{}/ui/dashboard       : Device's screen
{}/api/v1/upload_key  : Endpoint to upload an Urbit ship's key
{}/api/v1/upload_pier : Endpoint to upload an entire Urbit ship's pier
{}/api/v1/status      : Endpoint that returns the status of the Nucleus and Home-Urbit
"#,
        bind, bind, bind, bind, bind
    );
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            // Normalize path: /ui to /ui/ and /ui/// to /ui/
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::MergeOnly,
            ))
            .route("/", web::get().to(homepage_redirect))
            .service(
                actix_files::Files::new("/ui", "./src/ui")
                    .show_files_listing()
                    .index_file("index.html"),
            )
            .service(
                web::scope("/api/v1")
                    .route("/upload_key", web::post().to(upload_key))
                    .route("/upload_pier", web::post().to(upload_pier))
                    .route("/status", web::get().to(status)),
            )
    })
    .bind(bind)?
    .run()
    .await
}

#[derive(Debug, Clone)]
pub enum NucleusMode {
    Docker,
    Process,
}

impl FromStr for NucleusMode {
    type Err = String;

    fn from_str(input: &str) -> Result<NucleusMode, Self::Err> {
        match input {
            "docker" => Ok(NucleusMode::Docker),
            "process" => Ok(NucleusMode::Process),
            _ => Err(format!("'{}' is not a valid value for NucleusMode", input)),
        }
    }
}

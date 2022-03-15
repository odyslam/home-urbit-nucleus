use actix_files;
use actix_web::{middleware, web, App, HttpServer};
use api::*;
use std::{net::SocketAddrV4, str::FromStr};
use tracing::{debug, info, Level};
use tracing_subscriber;

use clap::{IntoApp, Parser};
use clap_complete::generate;
use cli::Opts;

mod api;
mod cli;
mod config;
mod docker_driver;
mod urbit_driver;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts = Opts::parse();
    match opts.clone() {
        Opts::StartServer {
            bind,
            config_dir,
            docker_endpoint,
            mode,
            backup_dir,
            backup_elapsed,
            logs,
        } => {
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
       \ }{ /\ )_ / _\
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

async fn start_api(bindOrNone: Option<SocketAddrV4>) -> std::io::Result<()> {
    let bind = bindOrNone.unwrap_or_else(|| "127.0.0.1:6969".parse::<SocketAddrV4>().unwrap());
    info!("Binding API server to {}", bind);
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

struct Config {}

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

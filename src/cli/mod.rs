use crate::NucleusMode;
use clap::{Parser, Subcommand};
use std::{net::SocketAddrV4, path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
#[clap(name = "nucleus", version = VERSION_MESSAGE, about = "The core of Home-Urbit")]
pub enum Opts {
    #[clap(name = "start", about = "start nucleus and it's API server")]
    StartServer {
        #[clap(
            name = "bind",
            short,
            long,
            help = "IP:PORT to bind the API to.",
            env = "NUCLEUS_BIND"
        )]
        bind: Option<SocketAddrV4>,
        #[clap(
            name = "config-dir",
            short,
            long,
            help = "Relative path to the configuration file",
            env = "NUCLEUS_CONFIG_DIR"
        )]
        config_dir: Option<PathBuf>,
        #[clap(
            name = "docker-driver",
            short,
            long,
            help = "IP:PORT or UNIX socket for the Docker engine",
            env = "NUCLEUS_DOCKER_DRIVER"
        )]
        docker_endpoint: Option<String>,
        #[clap(
            name = "mode",
            short,
            long,
            help = "How Nucleus should run Urbit, 'docker' or 'process'",
            env = "NUCLEUS_MODE"
        )]
        mode: Option<NucleusMode>,
        #[clap(
            name = "backup-dir",
            long,
            help = "Relative path to the backup directory",
            env = "NUCLEUS_BACKUP_DIR"
        )]
        backup_dir: Option<PathBuf>,
        #[clap(
            name = "backup-interval",
            long,
            help = "Every how many hours a backup of Urbit should be saved. Accepts 1 to 24",
            env = "NUCLEUS_BACKUP_INTERVAL"
        )]
        backup_elapsed: Option<i64>,
    },
}

/// The version message for the current program, like
/// `nucleus 0.1.0 (f01b232bc 2022-01-22T23:28:39.493201+00:00)`
pub(crate) const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA_SHORT"),
    " ",
    env!("VERGEN_BUILD_TIMESTAMP"),
    ")"
);

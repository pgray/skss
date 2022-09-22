use clap::{arg_enum, Parser, Subcommand};
use log::{debug, error, info};
use std::time::Duration;
use tokio::{task, time};

mod util;
use util::{get_keys, update, write_tmp};

/// simple service to sync user accounts from github
#[derive(Parser, Debug)]
#[clap(author,version,about,long_about = None)]
struct Cli {
    /// comma delimited list of user to sync from github
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Cron {
        #[clap(short, long, value_parser, value_delimiter = ',')]
        users: Vec<String>,
        #[clap(long, default_value = "github")]
        host: KeyHost,
        #[clap(long, default_value = "/var/tmp/gss")]
        tmpdir: String,
    },
    Server {
        #[clap(long, value_parser, default_value = "24")]
        hours: i32,
        #[clap(short, long, value_parser, value_delimiter = ',')]
        users: Vec<String>,
        #[clap(long, default_value = "github")]
        host: KeyHost,
        #[clap(long, default_value = "/var/tmp/gss")]
        tmpdir: String,
    },
}

arg_enum! {
#[derive(Debug,Clone)]
pub enum KeyHost {
    Gitlab,
    Github,
    Sourcehut,
}
}

struct Cron {
    users: Vec<String>,
    host: KeyHost,
    tmpdir: String,
}

use std::fs::metadata;
use std::os::unix::fs::PermissionsExt;
impl Cron {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        for i in self.users.clone().into_iter() {
            info!("found user {}", i);
            let resp = get_keys(&self.host, &i).await?;
            info!("got keys: {}", resp.clone().to_string());
            let err = write_tmp(i.clone(), resp.to_string(), self.tmpdir.clone()).await;
            match err {
                Ok(_) => println!("ok"),
                Err(e) => error!("{}", e),
            }
            if update(&i, self.tmpdir.clone()).await? {
                let user_ssh_path = format!("/home/{}/.ssh", i.clone());
                if !std::path::Path::new(&user_ssh_path).is_dir() {
                    use std::fs::create_dir_all;
                    create_dir_all(&user_ssh_path)?;
                }
                metadata(user_ssh_path)?.permissions().set_mode(0o700);
                let auth_path = format!("/home/{}/.ssh/authorized_keys", i);
                std::fs::copy(
                    format!("{}/{}.keys", self.tmpdir.clone(), i),
                    auth_path.clone(),
                )?;
                let mut authfile = metadata(auth_path)?.permissions();
                authfile.set_mode(0o600);
            }
        }
        Ok(())
    }
}
struct Server {
    period_hours: i32,
    users: Vec<String>,
    host: KeyHost,
    tmpdir: String,
}

impl Server {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        debug!(
            "hours: {} users: {} and url: {} (tmpdir: {}",
            self.period_hours,
            self.users.len(),
            self.host,
            self.tmpdir,
        );
        let u = self.users.clone();
        let h = self.period_hours.clone() as u64;
        let ho = self.host.clone();
        let worker = task::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(60 * 60 * h));
            loop {
                interval.tick().await;
                for i in 0..u.len() {
                    debug!("{}", u[i]);
                    let keys = get_keys(&ho, &u[i]).await;
                    match keys {
                        Ok(k) => debug!("{}", &k),
                        Err(_) => error!("fuk"),
                    }
                }
            }
        });
        worker.await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Cli::parse();
    match args.command {
        Commands::Cron {
            users,
            host,
            tmpdir,
        } => {
            Cron {
                users,
                host,
                tmpdir,
            }
            .run()
            .await?
        }
        Commands::Server {
            users,
            hours,
            host,
            tmpdir,
        } => {
            Server {
                users,
                period_hours: hours,
                host,
                tmpdir,
            }
            .run()
            .await?
        }
    }
    Ok(())
}

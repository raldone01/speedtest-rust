#![forbid(unsafe_code)]

use clap::{Arg, ArgAction, Command};
use log::error;
use crate::http::http_server::HttpServer;

mod http;
mod results;
mod database;
mod ip;
mod config;

fn main() -> std::io::Result<()> {
    //parse args
    let args = Command::new("librespeed-rs")
        .version("1.2.0")
        .about("Rust backend for LibreSpeed")
        .arg(Arg::new("server_config_path").short('c').long("config"))
        .arg(Arg::new("update-ipdb")
            .long("update-ipdb")
            .help("Update IPInfo country asn database")
            .action(ArgAction::SetTrue))
        .get_matches();

    if args.get_flag("update-ipdb") {
        ip::updater::update_ipdb("https://raw.githubusercontent.com/librespeed/speedtest/master/backend/country_asn.mmdb","country_asn.mmdb");
        return Ok(())
    }

    //get config path
    let config_path = args.get_one::<String>("server_config_path");

    //init configs & statics
    if let Err(e) = config::init_configs(config_path) {
        error!("{}",e.to_string());
        std::process::exit(1)
    }

    //init database
    let database = database::init();
    match database {
        Ok(mut database) => {
            let runtime = config::init_runtime();
            match runtime {
                Ok(runtime) => {
                    runtime.block_on(async  {
                        let http_server = HttpServer::init().await;
                        match http_server {
                            Ok(mut http_server) => {
                                http_server.listen(&mut database).await;
                            }
                            Err(e) => {
                                error!("{}",e.to_string());
                                std::process::exit(1)
                            }
                        }
                    });
                }
                Err(e) => {
                    error!("{}",e.to_string());
                    std::process::exit(1)
                }
            }
        }
        Err(e) => {
            error!("{}",e.to_string());
            std::process::exit(1)
        }
    }
    Ok(())
}
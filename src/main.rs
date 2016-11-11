extern crate app_therapy;
extern crate docopt;
extern crate sodiumoxide;
extern crate rustc_serialize;

use app_therapy::config::*;
use app_therapy::client;
use app_therapy::crypto;
use app_therapy::server;

use docopt::Docopt;
use sodiumoxide::crypto::box_::{PUBLICKEYBYTES, SECRETKEYBYTES};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

const USAGE: &'static str = "
App Therapy.

Usage:
  app_therapy (-h | --help)
  app_therapy --version
  app_therapy --gen-keys
  app_therapy --agent [--config=<config_file>]
  app_therapy <component> [<action>] --app=<application>

Options:
  -h, --help              Show this screen
  --version               Show version
  --agent                 Launch in agent mode (Listen for incomming commands)
  --gen-keys              Generate public and private keys
  --config=<config_file>  Specify the configuration file to be used
  --app=<application>     Specify the application context to use

Subcommands:
  dbms                    Query the dbms via the specified <agent>
  cache                   Perform actions on the caching layer
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_agent: bool,
    flag_config: Vec<String>,
    flag_gen_keys: bool,
    flag_version: bool,
    arg_application: Option<String>,
    arg_command: Option<String>,
    arg_config: Option<String>,
    arg_component: Option<String>,
    arg_action: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                      .and_then(|d| d.decode())
                      .unwrap_or_else(|e| e.exit());

    let config = match load_config() {
        Some(config) => config,
        None => panic!("Failed to parse config file!"),
    };

    //println!("{:?}", args);

    match &args.flag_gen_keys {
        &true => crypto::generate_keys(),
        &false => match &args.flag_agent {
            &true => as_agent(args, config),
            &false => as_client(args, config),
        }
    }
}

fn as_agent(args: Args, config: Config) {
    let listener = TcpListener::bind(config.agent_address.as_str()).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    server::process_request(stream)
                });
            },
            Err(e) => {
                println!("There was a problem with the connection {}", e.description());
            }
        }
    }
}

fn as_client(args: Args, config: Config) {
    let mut stream = client::connect(&config.agent_address);

    let mut pub_key_file = match File::open(&config.crypto.pub_key_file) {
        Ok(file) => file,
        Err(reason) => panic!("Failed to open public key file {}: {}", &config.crypto.pub_key_file, reason.description()),
    };
    let mut priv_key_file = match File::open(&config.crypto.priv_key_file) {
        Ok(file) => file,
        Err(reason) => panic!("Failed to open private key file {}: {}", &config.crypto.priv_key_file, reason.description()),
    };

    let mut pub_key_buf = vec![0; PUBLICKEYBYTES];
    let mut priv_key_buf = vec![0; SECRETKEYBYTES];

    let pk = match pub_key_file.read(&mut pub_key_buf) {
        Ok(_) => match crypto::to_pub(&pub_key_buf) {
            Some(key) => key,
            None => panic!("Unable to convert public key!"),
        },
        Err(reason) => panic!("Can't read public key data"),
    };

    let sk = match priv_key_file.read(&mut priv_key_buf) {
        Ok(_) => match crypto::to_priv(&priv_key_buf) {
            Some(key) => key,
            None => panic!("Unable to convert private key!"),
        },
        Err(reason) => panic!("Can't read private key data"),
    };

    // Figure out what our op is
    let mut task = match args.arg_component {
        Some(component) => component,
        None => panic!("No component specified!"),
    };

    // Figure out what our command is, if there is one
    let command = match args.arg_command {
        Some(command) => command,
        None => match args.arg_action {
            Some(action) => action,
            None => String::from(""),
        }
    };

    task.push_str(":");
    task.push_str(&command);

    let message = crypto::new_box(task.as_bytes(), &pk, &sk);
}

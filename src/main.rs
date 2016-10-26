extern crate app_therapy;
extern crate rustc_serialize;
extern crate docopt;

use app_therapy::config::*;
use app_therapy::client;
use app_therapy::crypto;
use app_therapy::server;

use docopt::Docopt;
use std::error::Error;
use std::net::{TcpListener, TcpStream};
use std::thread;

const USAGE: &'static str = "
App Therapy.

Usage:
  app_therapy (-h | --help)
  app_therapy --version
  app_therapy --agent [--config=<config_file>]
  app_therapy exec <command> --app=<application> [--config=<config_file>]
  app_therapy <component> [<action>] --app=<application>

Options:
  -h, --help              Show this screen
  --version               Show version
  --agent                 Launch in agent mode (Listen for incomming commands)
  --config=<config_file>  Specify the configuration file to be used
  --app=<application>     Specify the application context to use

Subcommands:
  exec                    Run the given command via the specified <agent>
  logs                    Display logs via the specified <agent>
  dbms                    Query the dbms via the specified <agent>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_agent: bool,
    flag_config: Vec<String>,
    flag_version: bool,
    cmd_exec: bool,
    arg_application: Option<Vec<String>>,
    arg_command: Option<Vec<String>>,
    arg_config: Option<Vec<String>>,
    arg_component: Option<Vec<String>>,
    arg_action: Option<Vec<String>>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                      .and_then(|d| d.decode())
                      .unwrap_or_else(|e| e.exit());

    let config = match load_config() {
        Some(config) => config,
        None => panic!("Failed to parse config file!"),
    };

    println!("{:?}", args);

    match &args.flag_agent {
        True => as_agent(args, config),
        False => as_client(args, config),
    }

    println!("User: {}\nPassword: {}", config.user.login, config.user.password);
}

fn as_agent(args: Args, config: Config) {
    let listener = TcpListener::bind(&config.agent_address).unwrap();

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

    let pk = match crypto::to_pub(&config.crypto.pub_key_file) {
        Some(key) => key,
        None => panic!("Failed to retrieve public key"),
    };

    let sk = match crypto::to_priv(&config.crypto.priv_key_file) {
        Some(key) => key,
        None => panic!("Failed to retrieve secret key"),
    };

    // Figure out what our op is
    let op = match args.cmd_exec {
        True => vec!(String::from("exec")),
        False => match args.arg_component {
            Some(component) => component,
            None => panic!("No component specified!"),
        }
    };

    // Figure out what our command is, if there is one
    let command = match args.arg_command {
        Some(command) => command,
        None => match args.arg_action {
            Some(action) => action,
            None => vec!(String::from("")),
        }
    };

    let message = crypto::new_box(&args.arg_component, &pk, &sk);
}

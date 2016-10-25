extern crate app_therapy;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use app_therapy::config::*;
use std:net::{TcpListener, TcpStream};

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

    match &args.flag_agent {
        True => as_agent(args),
        False => as_client(args),
    }

    let config = match config::load_config() {
        Some(config) => config,
        None => panic!("Failed to parse config file!"),
    };

    println!("User: {}\nPassword: {}", config.user.login, config.user.password);
}

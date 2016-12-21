extern crate app_therapy;
extern crate byteorder;
extern crate docopt;
extern crate sodiumoxide;
extern crate rustc_serialize;

use app_therapy::config::*;
use app_therapy::client;
use app_therapy::crypto;
use app_therapy::crypto::{FileBacked};
use app_therapy::messaging::{ Message, SEPARATOR };
use app_therapy::server;

use byteorder::{NetworkEndian, WriteBytesExt};
use docopt::Docopt;
use sodiumoxide::crypto::box_::{PublicKey, SecretKey, NONCEBYTES};
use std::error::Error;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener};
use std::process::exit;
use std::sync::Arc;
use std::thread;

const USAGE: &'static str = "
App Therapy.

Usage:
  app_therapy (-h | --help)
  app_therapy --version
  app_therapy --gen-keys
  app_therapy --agent [--config=<config_file>]
  app_therapy <component> <action> [--app=<application>] [--config=<config_file>]

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
    flag_config: Option<String>,
    flag_gen_keys: bool,
    flag_version: bool,
    arg_application: Option<String>,
    arg_command: Option<String>,
    arg_component: Option<String>,
    arg_action: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                      .and_then(|d| d.decode())
                      .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("Version 0.1.0");
        exit(0);
    }

    // Set default config file name if none is given
    let config_file = match args.flag_config {
        Some(ref file) => file.clone(),
        None => "app_therapy.json".to_string(),
    };

    match &args.flag_gen_keys {
        &true => crypto::generate_keys(),
        &false => match &args.flag_agent {
            &true => as_agent(args, AgentConfig::read(&config_file).unwrap()),
            &false => as_client(args, ClientConfig::read(&config_file).unwrap()),
        }
    }
}

fn as_agent(args: Args, config: AgentConfig) {
    // Currently the keys structure is immutable.  Ideally this would move out into
    // some form of actor type structure that could respond to requests for keys as well
    // as instructions to reload the key list without restarting the app.
    let key_map: HashMap<String, SecretKey> = server::load_keys(&config.users);
    let keys = Arc::new(key_map);

    let listener = TcpListener::bind(config.listen.as_str()).unwrap();

    for stream in listener.incoming() {
        let local_keys = keys.clone();
        let agent_key = match PublicKey::read_from(&config.public_key) {
            Some(key) => key,
            None => panic!("Couldn't read our own secret key")
        };

        match stream {
            Ok(mut stream) => {
                thread::spawn(move|| {

                    server::process_request(&mut stream, agent_key, &local_keys)
                });
            },
            Err(e) => {
                println!("There was a problem with the connection {}", e.description());
            }
        }
    }
}

fn as_client(args: Args, config: ClientConfig) {
    let pk = match PublicKey::read_from(&config.crypto.agent_key_file) {
        Some(key) => key,
        None => panic!("Unable to convert public key!"),
    };

    let sk = match SecretKey::read_from(&config.crypto.priv_key_file) {
        Some(key) => key,
        None => panic!("Unable to convert private key!"),
    };

    println!("Public key: {:?}", &pk);
    //println!("{:?}", args);

    let application = match args.arg_application {
        Some(app) => app,
        None => "".to_string(),
    };

    let message = Message::new(vec![&args.arg_component.unwrap(), &args.arg_action.unwrap(), &application]);

    let (nonce, boxed_message) = crypto::new_box(message.to_payload().as_bytes(), &pk, &sk);

    println!("Nonce: {:?}", &nonce);
    println!("Boxed: {:?}", &boxed_message);

    let mut response = vec![0; 1000];
    let message_size: u64 = (&config.user.user_name.len() + SEPARATOR.len() + NONCEBYTES + &boxed_message.len()) as u64;

    let mut stream = client::connect(&config.agent_address);
    let _ = stream.write_u64::<NetworkEndian>(message_size);
    let _ = stream.write(config.user.user_name.as_bytes());
    let _ = stream.write(SEPARATOR.as_bytes());
    let _ = stream.write(&nonce.0);
    let _ = stream.write(&boxed_message);

    let _ = stream.read_to_end(&mut response);
}

use clap::Parser;
use lettre::{Message, SmtpTransport, Transport};
use serde_derive::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize, Debug)]
struct Configuration {
    gmail: Gmail,
}

#[derive(Deserialize, Debug)]
struct Gmail {
    credentials: Credentials,
    relay_server: String,
}

#[derive(Deserialize, Debug)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CommandArguments {
    // who to send the email to
    #[arg(short = 't', long = "to")]
    to: String,

    // subject
    #[arg(short = 's', long = "subject")]
    subject: String,

    // email body
    #[arg(short = 'b', long = "body")]
    body: String,
}

fn read_config(filepath: String) -> Configuration {
    let c = fs::read_to_string(filepath).unwrap();
    let cfg: Configuration = toml::from_str(c.as_str()).unwrap();

    cfg
}

fn main() {
    let cfg_path = "./config/credentials.toml";
    let cfg = read_config(cfg_path.to_string());

    let args = CommandArguments::parse();

    let email = Message::builder()
        .from(cfg.gmail.credentials.username.parse().unwrap())
        .reply_to(args.to.parse().unwrap())
        .to(args.to.parse().unwrap())
        .subject(args.subject)
        .body(String::from(args.body))
        .unwrap();

    let creds = lettre::transport::smtp::authentication::Credentials::new(
        cfg.gmail.credentials.username,
        cfg.gmail.credentials.password,
    );

    let mailer = SmtpTransport::relay(cfg.gmail.relay_server.as_str())
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(err) => panic!("Couldn't send the email...\n{:?}", err),
    }
}

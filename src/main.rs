#[macro_use]
extern crate log;
#[macro_use]
extern crate rustful;
extern crate env_logger;
extern crate rust_captcha;

use rustful::{Server, TreeRouter};
use std::error::Error;
use std::env;

use rust_captcha::requesthandler::{RequestHandler, CaptchaMethod};

fn precondition_checks() -> bool {
    match env::var("REDIS_HOST") {
        Err(_) => {
            error!("Environment variable REDIS_HOST not set.");
            false
        },
        Ok(_)  => true
    }
}

fn main() {
    env_logger::init();
    let port: u16 = env::var_os("PORT").unwrap().into_string().unwrap().parse::<u16>().unwrap();

    if !precondition_checks() {
        error!("Failed to start server.");
        return;
    }

    info!("Starting service on port {} ...", port);

    let ret = Server {
        handlers: insert_routes! {
            TreeRouter::new() => {
                "/new/:difficulty/:max_tries/:ttl" => Post: RequestHandler::new(CaptchaMethod::New),
                "/solution/:id/:solution"          => Post: RequestHandler::new(CaptchaMethod::Solution)
            }
        },
        host: port.into(),
        ..Server::default()
    }.run();

    match ret {
        Ok(_)  => { },
        Err(e) => error!("Could not start server: {}", e.description())
    }
}

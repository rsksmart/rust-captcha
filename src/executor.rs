extern crate rand;

use self::rand::os::OsRng;
use self::rand::Rng;

use super::config::Config;
use super::persistence::persist;
use super::captcha::{Captcha, CaptchaCreation, CaptchaToJson};

pub enum CaptchaError {
    DatabaseUnavailable,
    CaptchaNotFound,
}

pub struct CaptchaResult {
    pub captcha: CaptchaCreation,
    pub session: String
}

/*
pub fn get_captcha(conf: &Config) -> Result<CaptchaCreation, CaptchaError> {


}
*/

/// Creates a new CAPTCHA and persists it in a database.
pub fn create_and_persist_captcha(conf: &Config) -> Option<CaptchaResult> {

    // TODO: how expensive is it to create a new PRNG for every request?
    let mut rng = match OsRng::new() {
        Err(_) => {
            error!(target: "executor::create_and_persist_captcha()", "Could not create RNG.");
            return None;
        },
        Ok(r) => { r }
    };

    let session = new_session(&mut rng);
    let solution = new_solution(&mut rng, conf.min_letters, conf.max_letters, &conf.characters);

    // TODO create the image

    let c = Captcha {
        tries: 0,
        max_tries: conf.max_tries,
        solved: false,
        session: session,
        solution: solution,
    };

    info!(target: "executor::create_and_persist_captcha()", "Created new CAPTCHA: {}", c.to_json());

    match persist(&c) {
        true => {
            let session = c.session.clone();
            let cc = CaptchaCreation::new(c);
            Some(CaptchaResult {
                captcha: cc,
                session: session,
            })
        }
        false => None
    }
}

// ----------------------------------------------------------------------------

static SESSION_CHARS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyz";

pub fn validate_session(id: &String) -> bool {
    id.chars().count() == 20 && id.chars().all(|x| SESSION_CHARS.contains(x))
}

/// Creates a new random session id.
fn new_session(rng: &mut OsRng) -> String {

    let c = SESSION_CHARS.chars().collect::<Vec<_>>();
    let mut s = String::new();
    for _ in 0..20 {
        s.push(*rng.choose(&c).unwrap());
    }
    s
}

/// Creates a new CAPTCHA text from a set of characters.
fn new_solution(rng: &mut OsRng, minlen: u32, maxlen: u32, chars: &String) -> String {

    let l = rng.next_u32() % (maxlen - minlen + 1) + minlen;
    let c = chars.chars().collect::<Vec<_>>();

    let mut s = String::new();
    for _ in 0..l {
        s.push(*rng.choose(&c).unwrap());
    }
    s
}

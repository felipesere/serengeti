#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use rocket::State;
use base64::encode;
use serde_derive::Deserialize;

use shared::{VoterRegistryKeys, VoterPublicKeys, RegisteredVoters};

use rocket::{Request, Data};
use rocket::data::Outcome;
use rocket::http::{Status, ContentType};
use rocket::data::FromDataSimple;
use serde_json;
use std::io::Read;
use rocket::outcome::Outcome::Failure;
use rocket::outcome::Outcome::Success;

#[post("/voter_list", data = "<key>")]
fn register_new_voter(key: SomePublicKey) -> &'static str {
    println!("{:?}", key);

    "well done"
}

#[derive(Debug, Deserialize)]
pub struct SomePublicKey {
    value: String,
}

impl FromDataSimple for SomePublicKey {
    type Error = String;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, String> {

        let mut buffer = String::new();
        if let Err(e) = data.open().take(256).read_to_string(&mut buffer) {
           return Failure((Status::InternalServerError, format!("{:?}", e)))
        }

        match serde_json::from_str::<SomePublicKey>(&buffer) {
            Ok(key) => Success(key),
            Err(e) => Failure((Status::InternalServerError, format!("{:?}", e)))
        }
    }
}

#[get("/voter_list")]
fn index(registry_keys: State<VoterRegistryKeys>) -> Json<RegisteredVoters> {

    let keys = vec!["ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINALgioqnUNxPH6VsSlvfibXdXICc1C8u4tJE7aOu9Og"];

    let public_keys: Vec<_> = keys.iter().map(|k| VoterPublicKeys::from(k)).collect();
    let signed_keys = registry_keys.sign(&public_keys[..]);

    let signature = encode(signed_keys.as_ref());

    Json(RegisteredVoters { signature, public_keys })
}

fn main() {
    rocket::ignite()
        .manage(VoterRegistryKeys::new())
        .mount("/", routes![index, register_new_voter])
        .launch();
}

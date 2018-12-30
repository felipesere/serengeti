#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use rocket::State;
use base64::encode;

use shared::{VoterRegistryKeys, VoterPublicKeys, RegisteredVoters, generate_initial_keypair};

#[post("/voter_list")]
fn register_new_voter() -> &'static str {
    "well done"
}

#[get("/voter_list")]
fn index(registry_keys: State<VoterRegistryKeys>) -> Json<RegisteredVoters> {
    let keys = vec!["ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINALgioqnUNxPH6VsSlvfibXdXICc1C8u4tJE7aOu9Og"];

    let public_keys = keys.iter().map(|k| VoterPublicKeys::from(k)).collect::<Vec<_>>();
    let signed_keys = registry_keys.sign(&public_keys[..]);

    let signature = encode(signed_keys.as_ref());

    Json(RegisteredVoters {
        signature: signature,
        public_keys: public_keys
    })

}

fn main() {
    rocket::ignite()
        .manage(generate_initial_keypair())
        .mount("/", routes![index, register_new_voter])
        .launch();
}

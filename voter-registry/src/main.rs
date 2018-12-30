#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use serde_derive::{Serialize};

use rocket_contrib::json::Json;
use rocket::State;
use ring::signature::{Ed25519KeyPair, Signature};
use ring::rand::SystemRandom;
use untrusted::Input;
use base64::encode;

#[derive(Serialize)]
#[serde(transparent)]
struct VoterPublicKeys {
    value: String
}

#[derive(Serialize)]
struct RegisteredVoters {
    signature: String,
    public_keys: Vec<VoterPublicKeys>,
}

#[post("/voter_list")]
fn register_new_voter() -> &'static str {
    "well done"
}

#[get("/voter_list")]
fn index(registry_keys: State<VoterRegistryKeys>) -> Json<RegisteredVoters> {
    let keys = vec!["ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINALgioqnUNxPH6VsSlvfibXdXICc1C8u4tJE7aOu9Og"];

    let public_keys = keys.iter().map(|k| VoterPublicKeys {value: (*k).to_owned()}).collect::<Vec<_>>();
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

struct VoterRegistryKeys {
    ed25519: Ed25519KeyPair
}

impl VoterRegistryKeys {
    fn sign<'a>(&self, keys: &[VoterPublicKeys]) -> Signature {
        let mut msg = String::new();
        for key in keys {
            msg.push_str(key.value.as_ref())
        }

        self.ed25519.sign(msg.as_ref())
    }
}

fn generate_initial_keypair() -> VoterRegistryKeys {
    let random = SystemRandom::new();

    let pkcs = Ed25519KeyPair::generate_pkcs8(&random).unwrap();

    VoterRegistryKeys { ed25519: Ed25519KeyPair::from_pkcs8(Input::from(&pkcs)).unwrap()}
}

#[cfg(test)]
mod tests {
    use ring::signature::Ed25519KeyPair;
    use ring::rand::SystemRandom;
    use untrusted::Input;

    #[test]
    fn generates_a_keypair() {
        let random = SystemRandom::new();

        let pkcs = Ed25519KeyPair::generate_pkcs8(&random).unwrap();

        let pair = Ed25519KeyPair::from_pkcs8(Input::from(&pkcs)).unwrap();
    }

}

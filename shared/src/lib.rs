use serde_derive::{Serialize};

use ring::signature::{Ed25519KeyPair, Signature};
use ring::rand::SystemRandom;
use untrusted::Input;
use base64::encode;

#[derive(Serialize)]
#[serde(transparent)]
pub struct VoterPublicKeys {
    value: String
}

impl <T: AsRef<str>> From<T> for VoterPublicKeys {
    fn from(key: T) -> VoterPublicKeys {
        VoterPublicKeys {value: key.as_ref().to_owned()}
    }
}

#[derive(Serialize)]
pub struct RegisteredVoters {
    pub signature: String,
    pub public_keys: Vec<VoterPublicKeys>,
}


pub struct VoterRegistryKeys {
    ed25519: Ed25519KeyPair
}

impl VoterRegistryKeys {
    pub fn sign(&self, keys: &[VoterPublicKeys]) -> Signature {
        let mut msg = String::new();
        for key in keys {
            msg.push_str(key.value.as_ref())
        }

        self.ed25519.sign(msg.as_ref())
    }

    pub fn new() -> Self {
        let random = SystemRandom::new();

        let pkcs = Ed25519KeyPair::generate_pkcs8(&random).unwrap();

        VoterRegistryKeys { ed25519: Ed25519KeyPair::from_pkcs8(Input::from(&pkcs)).unwrap()}
    }
}


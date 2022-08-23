use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use primitive_types::U512;
use serde::{Deserialize, Serialize};

use crate::elliptic_curve::elliptic_curve::EllipticCurvePoint;
use crate::elliptic_curve::encryption::Encryption;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

impl ConfigFile {
    pub fn from_config(config: Config) -> Self {
        let private_key = {
            if let Some(private_key) = config.private_key {
                let s = private_key.to_string();
                let encode = base64::encode(&s);
                Some(encode)
            } else {
                None
            }
        };

        let public_key = {
            if let Some(public_key) = config.public_key {
                let s = serde_json::to_string(&public_key).unwrap();
                let encode = base64::encode(&s);
                Some(encode)
            } else {
                None
            }
        };

        Self {
            private_key,
            public_key,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub private_key: Option<U512>,
    pub public_key: Option<EllipticCurvePoint>,
}

impl Config {
    pub fn read_or_create() -> Self {
        let encryption = Encryption::secp256k1();
        if Path::new("gpsl_conf").exists() {
            Config::from_file("gpsl_conf")
        } else {
            let private_key = Encryption::get_private_key();
            let config = Config {
                private_key: Some(private_key),
                public_key: Some(encryption.get_public_key(private_key)),
            };

            let mut file = File::create("gpsl_conf").unwrap();
            let config_file = ConfigFile::from_config(config.clone());
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            e.write_all(toml::to_string(&config_file).unwrap().as_bytes())
                .unwrap();
            file.write_all(&e.finish().unwrap()).unwrap();

            config
        }
    }

    fn read_file(file: &str) -> String {
        let mut file = fs::File::open(file).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        let mut d = ZlibDecoder::new(&contents[..]);
        let mut s = String::new();
        d.read_to_string(&mut s).unwrap();
        s
    }

    pub fn from_file(file: &str) -> Self {
        let file = Config::read_file(file);
        let config: ConfigFile = toml::from_str(&file).unwrap();

        let private_key = {
            if let Some(private_key) = config.private_key {
                let decoded = base64::decode(&private_key).unwrap();
                let s = std::str::from_utf8(&decoded).unwrap();
                Some(U512::from_str_radix(s, 10).unwrap())
            } else {
                None
            }
        };

        let public_key = {
            if let Some(public_key) = config.public_key {
                let decoded = base64::decode(&public_key).unwrap();
                let s = std::str::from_utf8(&decoded).unwrap();
                let r = EllipticCurvePoint::from_str(s).unwrap();
                Some(r)
            } else {
                None
            }
        };

        Config {
            private_key,
            public_key,
        }
    }
}

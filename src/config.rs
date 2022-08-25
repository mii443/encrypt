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
    pub private_key2: Option<String>,
    pub public_key: Option<String>,
    pub public_key2: Option<String>,
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

        let private_key2 = {
            if let Some(private_key2) = config.private_key2 {
                let s = private_key2.to_string();
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

        let public_key2 = {
            if let Some(public_key2) = config.public_key2 {
                let s = serde_json::to_string(&public_key2).unwrap();
                let encode = base64::encode(&s);
                Some(encode)
            } else {
                None
            }
        };

        Self {
            private_key,
            private_key2,
            public_key,
            public_key2,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub private_key: Option<U512>,
    pub private_key2: Option<U512>,
    pub public_key: Option<EllipticCurvePoint>,
    pub public_key2: Option<EllipticCurvePoint>,
}

impl Config {
    pub fn read_or_create(encryption: Option<Encryption>) -> Self {
        let encryption = encryption.unwrap_or(Encryption::secp256k1());
        if Path::new("gpsl_conf").exists() {
            Config::from_file("gpsl_conf")
        } else {
            let private_key = Encryption::get_private_key();
            let private_key2 = Encryption::get_private_key();
            let config = Config {
                private_key: Some(private_key),
                private_key2: Some(private_key2),
                public_key: Some(encryption.get_public_key(private_key)),
                public_key2: Some(encryption.get_public_key(private_key2)),
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

        let private_key2 = {
            if let Some(private_key2) = config.private_key2 {
                let decoded = base64::decode(&private_key2).unwrap();
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

        let public_key2 = {
            if let Some(public_key2) = config.public_key2 {
                let decoded = base64::decode(&public_key2).unwrap();
                let s = std::str::from_utf8(&decoded).unwrap();
                let r = EllipticCurvePoint::from_str(s).unwrap();
                Some(r)
            } else {
                None
            }
        };

        Config {
            private_key,
            private_key2,
            public_key,
            public_key2,
        }
    }
}

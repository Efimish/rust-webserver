//! # RSA keys loading and generation
//! Reading or generating RSA key pair for encryption and decryption.
//! Keys are saved in `keys` directory in the root of the project.
//! In case they can not be found, new key pair is generated.
//! These keys are then only passed to [Tokens][super::tokens] module.

use std::path::PathBuf;
use anyhow::Context;
use rsa::{
    RsaPrivateKey,
    RsaPublicKey,
    pkcs8::{
        EncodePrivateKey,
        EncodePublicKey,
        DecodePrivateKey,
        DecodePublicKey,
        LineEnding
    }
};
use once_cell::sync::Lazy;

pub(super) static KEY_PAIR: Lazy<RsaKeyPair> = Lazy::new(|| RsaKeyPair::get("keys").unwrap());

#[derive(Clone)]
pub struct RsaKeyPair {
    pub private: RsaPrivateKey,
    pub public: RsaPublicKey
}

impl RsaKeyPair {
    fn get(dir_path: &str) -> anyhow::Result<Self> {
        let current_dir: PathBuf = std::env::current_dir()
            .context("failed to access current directory")?;
        let keys_dir: PathBuf = current_dir.join(dir_path);

        if !keys_dir
        .try_exists()
        .context("failed to check if keys directory exists")? {
            log::warn!("Keys directory not found, creating new directory");
            std::fs::create_dir(&keys_dir)
            .context("failed to create keys directory")?;
        }

        let private_key_file: PathBuf = keys_dir.join("private.key");
        let public_key_file: PathBuf = keys_dir.join("public.key");

        if !private_key_file
            .try_exists()
            .context("failed to check if private key file exists")?
        || !public_key_file
            .try_exists()
            .context("failed to check if public key file exists")? {
            
            log::warn!("Keys not found, generating new key pair");
    
            let mut rng = rand::thread_rng();
            let bits = 2048;
            let private_key = RsaPrivateKey::new(&mut rng, bits)
                .context("failed to create private key")?;
            let public_key = RsaPublicKey::from(&private_key);
    
            private_key.write_pkcs8_pem_file(&private_key_file, LineEnding::default())
                .context("failed to write private key to file")?;
            public_key.write_public_key_pem_file(&public_key_file, LineEnding::default())
                .context("failed to write public key to file")?;
        }
        let private_key = RsaPrivateKey::read_pkcs8_pem_file(&private_key_file)
            .context("failed to read private key")?;
        let public_key = RsaPublicKey::read_public_key_pem_file(&public_key_file)
            .context("failed to read public key")?;
    
        Ok(Self{
            private: private_key,
            public: public_key
        })
    }
}
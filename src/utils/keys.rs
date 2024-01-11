use std::path::PathBuf;
use anyhow::{Context, ensure};
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey, LineEnding}};

#[derive(Clone)]
pub struct RsaKeyPair {
    pub private: RsaPrivateKey,
    pub public: RsaPublicKey
}

impl RsaKeyPair {
    pub fn read_or_generate(dir_path: &PathBuf) -> anyhow::Result<RsaKeyPair> {
        ensure!(
            dir_path
            .try_exists()
            .context("Error checking if keys directory exists")?,
            "keys directory does not exist"
        );
        let private_key_file: PathBuf = dir_path.join("private.key");
        let public_key_file: PathBuf = dir_path.join("public.key");

        if !private_key_file
            .try_exists()
            .context("Error checking if private key file directory exists")?
        || !public_key_file
            .try_exists()
            .context("Error checking if public key file directory exists")? {
            
            println!("Keys not found, generating key pair");
    
            let mut rng = rand::thread_rng();
            let bits = 2048;
            let private_key = RsaPrivateKey::new(&mut rng, bits)
                .context("Error creating private key")?;
            let public_key = RsaPublicKey::from(&private_key);
    
            private_key.write_pkcs8_pem_file(&private_key_file, LineEnding::default())
                .context("Error writing private key")?;
            public_key.write_public_key_pem_file(&public_key_file, LineEnding::default())
                .context("Error writing public key")?;
        }
        let private_key = RsaPrivateKey::read_pkcs8_pem_file(&private_key_file)
            .context("Error reading private key")?;
        let public_key = RsaPublicKey::read_public_key_pem_file(&public_key_file)
            .context("Error reading public key")?;
    
        Ok(RsaKeyPair{
            private: private_key,
            public: public_key
        })
    }
}
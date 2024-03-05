use ecdsa::signature::digest::Digest;
use k256::ecdsa::signature::RandomizedSigner;
use k256::ecdsa::Signature;
use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        SigningKey, VerifyingKey,
    },
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    PublicKey, SecretKey,
};
use rand_core::OsRng;

pub struct Utils {}

impl Utils {
    pub fn verify_signature(
        data: &str,
        signature: &Signature,
        verifying_key: &VerifyingKey,
    ) -> bool {
        verifying_key.verify(data.as_bytes(), signature).is_ok()
    }

    pub fn sign_data(data: &str, signing_key: &SigningKey) -> Signature {
        signing_key.sign_with_rng(&mut OsRng, data.as_bytes())
    }

    pub fn get_verifying_key(key: &str) -> Result<VerifyingKey, Box<dyn std::error::Error>> {
        let key = base64::decode(key).unwrap();
        let public_key: PublicKey = PublicKey::from_public_key_der(key.as_slice()).unwrap();
        let verifying_key: VerifyingKey = VerifyingKey::from(public_key);
        Ok(verifying_key)
    }

    pub fn get_signing_key(key: &str) -> Result<SigningKey, Box<dyn std::error::Error>> {
        let key = base64::decode(key).unwrap();
        let secret_key: SecretKey = SecretKey::from_pkcs8_der(key.as_slice()).unwrap();
        let signing_key: SigningKey = SigningKey::from(secret_key);
        Ok(signing_key)
    }

    pub fn encode_signature(signature: &Signature) -> String {
        hex::encode(signature.to_der()).to_string()
    }

    pub fn decode_signature(signature: &str) -> Result<Signature, Box<dyn std::error::Error>> {
        let signature_bytes = hex::decode(signature)?;
        Ok(Signature::from_der(&signature_bytes)?)
    }

    pub fn hash_data(data: &str) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let private = "MIGEAgEAMBAGByqGSM49AgEGBSuBBAAKBG0wawIBAQQgYp6GnxdjxLvnucsaaTZ+J+FqtCdjbEaQsEqxk3KHJ3yhRANCAAR6X+Ws+hYmkOMIZTq/HMVBRbMcT1lADpd4z5c3MG6LzyuMDBMGOZ4C3gceN6I0/kzgQ/DWEZcNY4s6/WgLxUD1";
        let public = "MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEel/lrPoWJpDjCGU6vxzFQUWzHE9ZQA6XeM+XNzBui88rjAwTBjmeAt4HHjeiNP5M4EPw1hGXDWOLOv1oC8VA9Q==";

        let signing_key = Utils::get_signing_key(private).unwrap();
        let verifying_key = Utils::get_verifying_key(public).unwrap();
        let data = "Hello, world!";
        let signature = Utils::sign_data(data, &signing_key);
        assert!(Utils::verify_signature(data, &signature, &verifying_key));

        let encoded_signature = Utils::encode_signature(&signature);
        let decoded_signature = Utils::decode_signature(&encoded_signature).unwrap();
        assert_eq!(signature, decoded_signature);
    }
}

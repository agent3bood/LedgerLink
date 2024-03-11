use k256::ecdsa::{Signature, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use utils::Utils;

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub nonce: u64,
    pub(crate) amount: u64,
    pub sender: String,
    sender_key: VerifyingKey,
    pub receiver: String,
    receiver_key: VerifyingKey,
    pub(crate) hash: String,
    signature: Option<(String, Signature)>,
}

impl Transaction {
    pub fn new(
        nonce: u64,
        amount: u64,
        sender: &str,
        receiver: &str,
        signature: Option<&str>,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        let hash = Transaction::calculate_hash(nonce, sender, receiver, amount);
        Ok(Transaction {
            nonce,
            amount,
            sender: sender.to_string(),
            sender_key: Utils::get_verifying_key(sender)?,
            receiver: receiver.to_string(),
            receiver_key: Utils::get_verifying_key(receiver)?,
            hash,
            signature: match signature {
                None => None,
                Some(signature) => {
                    Some((signature.to_string(), Utils::decode_signature(signature)?))
                }
            },
        })
    }

    pub fn sign(&mut self, signing_key: &SigningKey) -> Result<(), Box<dyn std::error::Error>> {
        if self.signature.is_some() {
            return Err("Transaction already signed".into());
        }
        let sig = Utils::sign_data(&self.hash, signing_key);
        self.signature = Some((Utils::encode_signature(&sig), sig));
        Ok(())
    }

    /// verify the transaction hash and signature
    pub fn verify(&self) -> bool {
        self.verify_hash() && self.verify_signature()
    }

    fn verify_hash(&self) -> bool {
        self.hash
            == Transaction::calculate_hash(self.nonce, &self.sender, &self.receiver, self.amount)
    }

    fn verify_signature(&self) -> bool {
        match self.signature {
            None => false,
            Some(ref signature) => {
                Utils::verify_signature(&self.hash, &signature.1, &self.sender_key)
            }
        }
    }

    fn calculate_hash(nonce: u64, sender: &str, receiver: &str, amount: u64) -> String {
        Utils::hash_data(&format!("{}{}{}{}", nonce, sender, receiver, amount))
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sender_key = "MIGEAgEAMBAGByqGSM49AgEGBSuBBAAKBG0wawIBAQQgGYFjSRDEGRmqvaJreuMY22pZz3TojuOm2dEmxhtbPTyhRANCAARSnspJBeKF9TrV5WmDTsJXb/wtihZ4YyXRmGASMIbzdYuW+B5vh1B/dRvZ15Ne8ehUQ/xH023fVx1STJzkSeoS";
        let sender = "MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEUp7KSQXihfU61eVpg07CV2/8LYoWeGMl0ZhgEjCG83WLlvgeb4dQf3Ub2deTXvHoVEP8R9Nt31cdUkyc5EnqEg==";
        let signing_key = Utils::get_signing_key(sender_key).unwrap();

        let mut transaction = Transaction::new(0, 100, sender, sender, None).unwrap();
        transaction.sign(&signing_key);
        assert_eq!(transaction.verify(), true);
    }
}

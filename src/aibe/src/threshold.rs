use crate::errors::IbeError;
use rand::Rng;
use rand::SeedableRng;
use bn::arith::U256;
use borsh::maybestd::collections::HashMap;
use crate::traits::ToBytes;
use crate::utils::{u64_to_scalar, hash_to_g2, scalar_to_bigint, bigint_to_scalar, hash_to_scalar};
use crate::rand_utils::{RandUtilsRng};
use crate::define::{MODULUS};
use crate::errors::{ThresholdError, require};
use crate::polynomial::{Polynomial};
use num_bigint::{BigInt, Sign};
use crate::rand_utils::Sample;
use borsh::maybestd::string::String;
use borsh::maybestd::vec::Vec;
use borsh::maybestd::string::ToString;
use rand_chacha::ChaCha20Rng;

pub use bn::{G1, G2, Gt, Fr, Group, pairing};

pub type Scalar = Fr;
pub type SecretKey = Scalar;
pub type PublicKey = G1;

pub struct Keypair {
    pub sk: SecretKey,
    pub pk: PublicKey,
}

impl Keypair {
    pub fn random() -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(0); 
        let sk = Scalar::random(&mut rng);
        let pk = G1::one() * sk;
        Self {
            sk, 
            pk
        }
    }

    pub fn from_sk(sk: SecretKey) -> Self {
        Self {
            sk: sk.clone(),
            pk: G1::one() * sk
        }
    }
}

pub struct Threshold {
    pub threshold: u64
}

impl Threshold {
    pub fn new(threshold: usize) -> Self {
        Self {
            threshold: threshold as u64
        }
    }

    pub fn key_gen(&mut self, ids: &[&str]) -> Result<(Keypair, HashMap<String, Keypair>), ThresholdError> {
        let kp = Keypair::random();
        let (kps, _) = self.key_split_with_poly(&kp.sk, ids)?;
        Ok((kp, kps))
    }

    pub fn key_gen_with_poly(&mut self, ids: &[&str]) -> Result<(Keypair, HashMap<String, Keypair>, Polynomial<BigInt>), ThresholdError> {
        let kp = Keypair::random();
        let (kps, poly) = self.key_split_with_poly(&kp.sk, ids)?;
        Ok((kp, kps, poly))
    }

    pub fn key_split_with_poly(&mut self, sk: &SecretKey, ids: &[&str]) -> Result<(HashMap<String, Keypair>, Polynomial<BigInt>), ThresholdError> {
        let mut rng = RandUtilsRng::new();

        let mut coeffs: Vec<BigInt> = rng.sample_vec(self.threshold as usize, &MODULUS);
        coeffs[0] = scalar_to_bigint(sk); 
        let poly = Polynomial::new(coeffs);

        let mut kps: HashMap<String, Keypair> = HashMap::new();
        for i in 0..ids.len() {
            let hash_id = hash_to_scalar(ids[i].as_bytes());

            let sk_share = bigint_to_scalar(&poly.eval(&scalar_to_bigint(&hash_id)));
            kps.insert(ids[i].to_string(), Keypair::from_sk(sk_share));
        }
        Ok((kps, poly))
    }

    pub fn recover(&self, ids: &[&str], sks: &[SecretKey]) -> Result<SecretKey, ThresholdError> {
        require(ids.len() == sks.len(), "Different length");
        let n = ids.len();
        let mut sk = Scalar::zero();
        for i in 0..n {
            let xi = hash_to_scalar(ids[i].as_bytes());
            let mut lagrange_coeff = Scalar::one();
            for j in 0..n {
                if i != j {
                    let xj = hash_to_scalar(ids[j].as_bytes());
                    lagrange_coeff = lagrange_coeff * ((xj - xi).inverse().unwrap() * xj);
                }
            }
            sk = sk + lagrange_coeff * sks[i];
        }
        Ok(sk)
    }

    // /// Split the key in a deterministic way.  
    // ///
    // /// **NEVER** use this in production! 
    // pub fn deterministic_key_split(&mut self, sk: &SecretKey, ids: &[&str]) -> Result<HashMap<String, Keypair>, DvfError> {
    //     let seed: [u8; 32] = [0; 32];
    //     let mut rng = RandUtilsRng::from_seed(&seed);

    //     let mut coeffs: Vec<BigInt> = rng.sample_vec(self.threshold(), &MODULUS);
    //     coeffs[0] = BigInt::from_bytes_be(Sign::Plus, &sk.serialize().as_bytes()); 
    //     let poly = Polynomial::new(coeffs);

    //     let mut kps: HashMap<u64, Keypair> = HashMap::new();
    //     for i in 0..ids.len() {
    //         if ids[i] == 0 {
    //             return Err(DvfError::KeyGenError(format!("Invalid id {}", ids[i])));
    //         }

    //         let (_, mut sk_share) = poly.eval(&(&ids[i]).to_bigint().unwrap()).reduce(&MODULUS).to_bytes_be();
    //         if sk_share.len() < SECRET_KEY_BYTES_LEN {
    //             (0..SECRET_KEY_BYTES_LEN-sk_share.len()).for_each(|_| sk_share.insert(0, 0));
    //         }
    //         let sk_share = SecretKey::deserialize(&sk_share[..]).unwrap();
    //         kps.insert(ids[i], Keypair::from_components(sk_share.public_key(), sk_share));
    //     }
    //     Ok(kps)
    // }

    // /// Compute the key share in a deterministic way.  
    // ///
    // /// **NEVER** use this in production! 
    // pub fn deterministic_key_share(&mut self, sk: &SecretKey, id: u64) -> Keypair {
    //     if id == 0 {
    //         panic!("Invalid id")
    //     }
        
    //     let seed: [u8; 32] = [0; 32];
    //     let mut rng = RandUtilsRng::from_seed(&seed);

    //     let mut coeffs: Vec<BigInt> = rng.sample_vec(self.threshold(), &MODULUS);
    //     coeffs[0] = BigInt::from_bytes_be(Sign::Plus, &sk.serialize().as_bytes()); 
    //     let poly = Polynomial::new(coeffs);

    //     let (_, mut sk_share) = poly.eval(&(id).to_bigint().unwrap()).reduce(&MODULUS).to_bytes_be();
    //     if sk_share.len() < SECRET_KEY_BYTES_LEN {
    //         (0..SECRET_KEY_BYTES_LEN-sk_share.len()).for_each(|_| sk_share.insert(0, 0));
    //     }
    //     let sk_share = SecretKey::deserialize(&sk_share[..]).unwrap();
    //     Keypair::from_components(sk_share.public_key(), sk_share)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_threshold() {
        let t = 2;
        let n = 3;
        let mut m_threshold = Threshold::new(t);
        let ids = vec!["gmail", "twitter", "telegram"];
        let (kp, kps) = m_threshold.key_gen(&ids).unwrap();

        let ids_ = vec!["gmail", "twitter"];
        let sks: Vec<SecretKey> = ids_.iter().map(|id| kps.get(&id.to_string()).unwrap().sk).collect();
        let sk_ = m_threshold.recover(&ids_, &sks).unwrap();

        println!("sk: {:?}", kp.sk);
        println!("sk_: {:?}", sk_);

        assert_eq!(kp.sk, sk_);
    }
}
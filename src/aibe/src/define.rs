use bn::{Fr};
use lazy_static::lazy_static;
use num_bigint::{Sign, BigInt};

lazy_static! {
    static ref BUF: [u8; 32] = {
        let mut m = [0; 32];
        let _ = Fr::modulus().to_big_endian(&mut m);
        m
    };
    pub static ref MODULUS: BigInt = BigInt::from_bytes_be(Sign::Plus, &BUF[..]);
}


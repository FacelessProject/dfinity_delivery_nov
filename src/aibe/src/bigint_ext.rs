use num_bigint::{BigInt, Sign};

pub trait Ring {
    fn reduce(&self, m: &BigInt) -> Self;
}

impl Ring for BigInt {
    fn reduce(&self, m: & BigInt) -> BigInt {
        let mut result = self.clone();
        result %= m;
        if result.sign() == Sign::Minus {
            result += m;
        }
        result
    }
}

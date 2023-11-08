use borsh::maybestd::string::String;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IbeError {
    GtInverseError,
    OutOfBoundError,
}

pub enum ZkError {
    ProofError,
    VerificationError,
}

#[derive(Debug)]
pub enum ThresholdError {
    KeyGenError(String),
}

pub fn require(status: bool, msg: &'static str) {
    if !status {
        panic!("{}", msg);
    }
}
use base58::{FromBase58, ToBase58};
use hex::{FromHex, ToHex};
use sha2::{Digest, Sha256};
use std::convert::TryFrom;
use std::fmt;
use std::iter;
use std::str::FromStr; // .parse

use crate::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Address([u8; 21]);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        b58encode_check(&self.0).fmt(f)
    }
}

impl TryFrom<&[u8]> for Address {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 21 {
            Err(Error::InvalidAddress)
        } else {
            let mut raw = [0u8; 21];
            raw[..21].copy_from_slice(value);
            Ok(Address(raw))
        }
    }
}

impl TryFrom<Vec<u8>> for Address {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(&value[..])
    }
}

impl FromHex for Address {
    type Error = Error;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        Address::try_from(hex.as_ref())
    }
}

impl ToHex for Address {
    fn encode_hex<T: iter::FromIterator<char>>(&self) -> T {
        self.0.encode_hex()
    }

    fn encode_hex_upper<T: iter::FromIterator<char>>(&self) -> T {
        self.0.encode_hex_upper()
    }
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if s.len() == 34 && s.as_bytes()[0] == b'T' {
            b58decode_check(s).and_then(Address::try_from)
        } else if s.len() == 42 && s.starts_with("41") {
            Vec::from_hex(s)
                .map_err(|_| Error::InvalidAddress)
                .and_then(Address::try_from)
        } else if s.len() == 44 && (s.starts_with("0x") || s.starts_with("0X")) {
            Vec::from_hex(&s.as_bytes()[2..])
                .map_err(|_| Error::InvalidAddress)
                .and_then(Address::try_from)
        } else {
            Err(Error::InvalidAddress)
        }
    }
}

fn b58encode_check<T: AsRef<[u8]>>(raw: T) -> String {
    let mut hasher = Sha256::new();
    hasher.input(raw.as_ref());
    let digest1 = hasher.result();

    let mut hasher = Sha256::new();
    hasher.input(&digest1);
    let digest = hasher.result();

    let mut raw = raw.as_ref().to_owned();
    raw.extend(&digest[..4]);
    raw.to_base58()
}

// FIXME: better isolated to a crate
fn b58decode_check(s: &str) -> Result<Vec<u8>, Error> {
    let mut result = s.from_base58().map_err(|_| Error::InvalidAddress)?;

    let check = result.split_off(result.len() - 4);

    let mut hasher = Sha256::new();
    hasher.input(&result);
    let digest1 = hasher.result();

    let mut hasher = Sha256::new();
    hasher.input(&digest1);
    let digest = hasher.result();

    if check != &digest[..4] {
        Err(Error::InvalidChecksum)
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address() {
        let addr = Address([
            65, 150, 163, 186, 206, 90, 218, 207, 99, 126, 183, 204, 121, 213, 120, 127, 66, 71, 218, 75, 190,
        ]);

        assert_eq!("TPhiVyQZ5xyvVK2KS2LTke8YvXJU5wxnbN", format!("{:}", addr));
        assert_eq!(addr, "TPhiVyQZ5xyvVK2KS2LTke8YvXJU5wxnbN".parse().expect("parse error"));
        assert_eq!(
            addr,
            "4196a3bace5adacf637eb7cc79d5787f4247da4bbe"
                .parse()
                .expect("parse error")
        );

        assert_eq!(addr.encode_hex::<String>(), "4196a3bace5adacf637eb7cc79d5787f4247da4bbe")
    }
}

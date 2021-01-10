//! Rewrites https://github.com/suyash/ulid from C++ to Rust
//!
//! This exposes a single interface for Ulid creation
//!
//! ```ignore
//! Ulid::new(u64, Fn() -> u8)
//! ```
//!
//! Takes the last 48 bits of the passed timestamp and calls the passed closure
//! 10 times for a random value.
//!
//! In place of explicit MarshalBinary and UnmarshalBinary, implements
//! `Into<[u8; 16]>`, `Into<&[u8]>`, `Into<Vec<u8>>`, `From<[u8; 16]>` and `TryFrom<&[u8]>`
//!
//! Along with `marshal` that returns 26 UTF-8 words, `TryInto<String>`, `TryInto<&str>`
//! and `ToString` are also implemented.
//!
//! Along with `unmarshal` that works with `AsRef<[u8]>`, `TryFrom<String>` and `TryFrom<&str>`
//! are also implemented.
//!
//! Most benchmarks line up with similar performance from C++, with some showing
//! improvements. Benchmarks are run on GitHub actions using criterion.

#![deny(missing_docs)]

use std::convert::TryFrom;
use std::convert::TryInto;

use thiserror::Error;

#[cfg(test)]
mod tests;

/// Crockford's base32
static ENCODING: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// stores decimal encodings for characters.
static DECODING: &[u8; 256] = &[
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    /* 0     1     2     3     4     5     6     7  */
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    /* 8     9                                      */
    0x08, 0x09, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    /*    10(A) 11(B) 12(C) 13(D) 14(E) 15(F) 16(G) */
    0xFF, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
    /*17(H)     18(J) 19(K)       20(M) 21(N)       */
    0x11, 0xFF, 0x12, 0x13, 0xFF, 0x14, 0x15, 0xFF,
    /*22(P)23(Q)24(R) 25(S) 26(T)       27(V) 28(W) */
    0x16, 0x17, 0x18, 0x19, 0x1A, 0xFF, 0x1B, 0x1C,
    /*29(X)30(Y)31(Z)                               */
    0x1D, 0x1E, 0x1F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];

/// Ulid
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Ulid([u8; 16]);

impl Ulid {
    /// creates new Ulid from a timestamp and a custom rng
    pub fn new<F>(timestamp: u64, rng: F) -> Ulid
    where
        F: Fn() -> u8,
    {
        let mut ans = Ulid([0; 16]);
        ans.encode_time(timestamp);
        ans.encode_entropy(rng);
        ans
    }

    /// encodes time in the first 6 words
    pub fn encode_time(&mut self, timestamp: u64) {
        self.0[0] = (timestamp >> 40) as u8;
        self.0[1] = (timestamp >> 32) as u8;
        self.0[2] = (timestamp >> 24) as u8;
        self.0[3] = (timestamp >> 16) as u8;
        self.0[4] = (timestamp >> 8) as u8;
        self.0[5] = timestamp as u8;
    }

    /// encodes entropy in the last 10 words
    pub fn encode_entropy<F>(&mut self, rng: F)
    where
        F: Fn() -> u8,
    {
        self.0[6] = rng();
        self.0[7] = rng();
        self.0[8] = rng();
        self.0[9] = rng();
        self.0[10] = rng();
        self.0[11] = rng();
        self.0[12] = rng();
        self.0[13] = rng();
        self.0[14] = rng();
        self.0[15] = rng();
    }

    /// create a string representation of the stored ULID
    ///
    /// https://github.com/suyash/val/blob/master/ulid_uint128.hh#L253
    pub fn marshal(&self) -> [u8; 26] {
        let mut ans = [0; 26];
        let val = self.0;

        // timestamp
        ans[0] = ENCODING[(((val[0] & 224) >> 5) as usize) as usize];
        ans[1] = ENCODING[(val[0] & 31) as usize];
        ans[2] = ENCODING[((val[1] & 248) >> 3) as usize];
        ans[3] = ENCODING[(((val[1] & 7) << 2) | ((val[2] & 192) >> 6)) as usize];
        ans[4] = ENCODING[((val[2] & 62) >> 1) as usize];
        ans[5] = ENCODING[(((val[2] & 1) << 4) | ((val[3] & 240) >> 4)) as usize];
        ans[6] = ENCODING[(((val[3] & 15) << 1) | ((val[4] & 128) >> 7)) as usize];
        ans[7] = ENCODING[((val[4] & 124) >> 2) as usize];
        ans[8] = ENCODING[(((val[4] & 3) << 3) | ((val[5] & 224) >> 5)) as usize];
        ans[9] = ENCODING[(val[5] & 31) as usize];

        // entropy
        ans[10] = ENCODING[((val[6] & 248) >> 3) as usize];
        ans[11] = ENCODING[(((val[6] & 7) << 2) | ((val[7] & 192) >> 6)) as usize];
        ans[12] = ENCODING[((val[7] & 62) >> 1) as usize];
        ans[13] = ENCODING[(((val[7] & 1) << 4) | ((val[8] & 240) >> 4)) as usize];
        ans[14] = ENCODING[(((val[8] & 15) << 1) | ((val[9] & 128) >> 7)) as usize];
        ans[15] = ENCODING[((val[9] & 124) >> 2) as usize];
        ans[16] = ENCODING[(((val[9] & 3) << 3) | ((val[10] & 224) >> 5)) as usize];
        ans[17] = ENCODING[(val[10] & 31) as usize];
        ans[18] = ENCODING[((val[11] & 248) >> 3) as usize];
        ans[19] = ENCODING[(((val[11] & 7) << 2) | ((val[12] & 192) >> 6)) as usize];
        ans[20] = ENCODING[((val[12] & 62) >> 1) as usize];
        ans[21] = ENCODING[(((val[12] & 1) << 4) | ((val[13] & 240) >> 4)) as usize];
        ans[22] = ENCODING[(((val[13] & 15) << 1) | ((val[14] & 128) >> 7)) as usize];
        ans[23] = ENCODING[((val[14] & 124) >> 2) as usize];
        ans[24] = ENCODING[(((val[14] & 3) << 3) | ((val[15] & 224) >> 5)) as usize];
        ans[25] = ENCODING[(val[15] & 31) as usize];

        ans
    }

    /// unmarshals a string-like into a ULID
    pub fn unmarshal<S>(s: S) -> Result<Ulid, UlidError>
    where
        S: AsRef<[u8]>,
    {
        let s = s.as_ref();

        if s.len() != 26 {
            return Err(UlidError::InvalidLength);
        }

        let mut val = [0; 16];

        // timestamp
        val[0] = (Self::unmarshal_word(s[0])? << 5) | Self::unmarshal_word(s[1])?;
        val[1] = (Self::unmarshal_word(s[2])? << 3) | (Self::unmarshal_word(s[3])? >> 2);
        val[2] = (Self::unmarshal_word(s[3])? << 6)
            | (Self::unmarshal_word(s[4])? << 1)
            | (Self::unmarshal_word(s[5])? >> 4);
        val[3] = (Self::unmarshal_word(s[5])? << 4) | (Self::unmarshal_word(s[6])? >> 1);
        val[4] = (Self::unmarshal_word(s[6])? << 7)
            | (Self::unmarshal_word(s[7])? << 2)
            | (Self::unmarshal_word(s[8])? >> 3);
        val[5] = (Self::unmarshal_word(s[8])? << 5) | Self::unmarshal_word(s[9])?;

        // entropy
        val[6] = (Self::unmarshal_word(s[10])? << 3) | (Self::unmarshal_word(s[11])? >> 2);
        val[7] = (Self::unmarshal_word(s[11])? << 6)
            | (Self::unmarshal_word(s[12])? << 1)
            | (Self::unmarshal_word(s[13])? >> 4);
        val[8] = (Self::unmarshal_word(s[13])? << 4) | (Self::unmarshal_word(s[14])? >> 1);
        val[9] = (Self::unmarshal_word(s[14])? << 7)
            | (Self::unmarshal_word(s[15])? << 2)
            | (Self::unmarshal_word(s[16])? >> 3);
        val[10] = (Self::unmarshal_word(s[16])? << 5) | Self::unmarshal_word(s[17])?;
        val[11] = (Self::unmarshal_word(s[18])? << 3) | (Self::unmarshal_word(s[19])? >> 2);
        val[12] = (Self::unmarshal_word(s[19])? << 6)
            | (Self::unmarshal_word(s[20])? << 1)
            | (Self::unmarshal_word(s[21])? >> 4);
        val[13] = (Self::unmarshal_word(s[21])? << 4) | (Self::unmarshal_word(s[22])? >> 1);
        val[14] = (Self::unmarshal_word(s[22])? << 7)
            | (Self::unmarshal_word(s[23])? << 2)
            | (Self::unmarshal_word(s[24])? >> 3);
        val[15] = (Self::unmarshal_word(s[24])? << 5) | Self::unmarshal_word(s[25])?;

        Ok(Ulid(val))
    }

    fn unmarshal_word(x: u8) -> Result<u8, UlidError> {
        if DECODING[x as usize] == 0xFF {
            Err(UlidError::InvalidCharacter)
        } else {
            Ok(DECODING[x as usize])
        }
    }

    /// return the timestamp associated with the Ulid
    pub fn timestamp(&self) -> u64 {
        let ans: u64 = 0;
        let ans = (ans << 8) | self.0[0] as u64;
        let ans = (ans << 8) | self.0[1] as u64;
        let ans = (ans << 8) | self.0[2] as u64;
        let ans = (ans << 8) | self.0[3] as u64;
        let ans = (ans << 8) | self.0[4] as u64;
        (ans << 8) | self.0[5] as u64
    }
}

impl From<[u8; 16]> for Ulid {
    fn from(s: [u8; 16]) -> Self {
        Ulid(s)
    }
}

impl TryFrom<&[u8]> for Ulid {
    type Error = std::array::TryFromSliceError;

    fn try_from(f: &[u8]) -> Result<Self, Self::Error> {
        Ok(Ulid(f.try_into()?))
    }
}

impl TryFrom<String> for Ulid {
    type Error = UlidError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ulid::unmarshal(s)
    }
}

impl TryFrom<&str> for Ulid {
    type Error = UlidError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ulid::unmarshal(s)
    }
}

impl Into<[u8; 16]> for Ulid {
    fn into(self) -> [u8; 16] {
        self.0
    }
}

impl<'a> Into<&'a [u8]> for &'a Ulid {
    fn into(self) -> &'a [u8] {
        &self.0
    }
}

impl Into<Vec<u8>> for Ulid {
    fn into(self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl TryInto<String> for Ulid {
    type Error = std::string::FromUtf8Error;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(String::from_utf8(self.marshal().to_vec())?)
    }
}

impl ToString for Ulid {
    fn to_string(&self) -> String {
        String::from_utf8(self.marshal().to_vec()).unwrap()
    }
}

/// errors
#[derive(Error, Debug)]
pub enum UlidError {
    /// parsing error
    #[error("invalid length for unmarshal")]
    InvalidLength,

    /// parsing error
    #[error("invalid character encountered while parsing")]
    InvalidCharacter,
}

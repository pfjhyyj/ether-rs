use digest::{crypto_common::BlockSizeUser, Digest, Mac};
use hmac::{Hmac, SimpleHmac};
use md5::Md5;
use rand::distributions::{Alphanumeric, DistString};
use sha1::Sha1;
use sha2::Sha256;

pub fn md5(b: &[u8]) -> String {
    let mut h = Md5::new();
    h.update(b);
    const_hex::encode(h.finalize())
}

pub fn sha1(b: &[u8]) -> String {
    let mut h = Sha1::new();
    h.update(b);
    const_hex::encode(h.finalize())
}

pub fn sha256(b: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(b);
    const_hex::encode(h.finalize())
}

pub fn hash<D: Digest>(b: &[u8]) -> String {
    let mut h = D::new();
    h.update(b);
    const_hex::encode(h.finalize())
}

pub fn hmac_sha1(key: &[u8], b: &[u8]) -> String {
    let mut h = Hmac::<Sha1>::new_from_slice(key).unwrap();
    h.update(b);
    const_hex::encode(h.finalize().into_bytes())
}

pub fn hmac_sha256(key: &[u8], b: &[u8]) -> String {
    let mut h = Hmac::<Sha256>::new_from_slice(key).unwrap();
    h.update(b);
    const_hex::encode(h.finalize().into_bytes())
}

pub fn hmac<D: Digest + BlockSizeUser>(key: &[u8], b: &[u8]) -> String {
    let mut h = SimpleHmac::<D>::new_from_slice(key).unwrap();
    h.update(b);
    const_hex::encode(h.finalize().into_bytes())
}

pub fn nonce(size: usize) -> String {
    let mut rng = rand::thread_rng();
    Alphanumeric.sample_string(&mut rng, size)
}

pub fn bcrypt(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}

pub fn bcrypt_verify(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap()
}

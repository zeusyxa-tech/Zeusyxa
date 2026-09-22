//! # Dilithium3 — Pure Rust wrapper (dung dilithium-rs)
//!
//! Thay the pqcrypto-dilithium (C code / cc crate) bang dilithium-rs (pure Rust).
//!
//! ## Thay doi API:
//! - `sign_message` bay gio can `&DilithiumKeyPair` (ca pk + sk)
//!   Vi dilithium-rs can full keypair de sign
//!
//! ## Giu nguyen:
//! - Kich thuoc: PK=1952, SK=4032, Sig=3309
//! - Type: DilithiumPublicKey, DilithiumSignature, DilithiumSecretKey
//! - Substrate traits: Verify, IdentifyAccount

use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;

use codec::{Decode, Encode, MaxEncodedLen, DecodeWithMemTracking};
use scale_info::TypeInfo;
use sp_runtime::traits::{IdentifyAccount, Verify, Lazy};

use dilithium::{DilithiumKeyPair as DKP, DilithiumSignature as DSig, ML_DSA_65};

pub const DILITHIUM_PUBLIC_KEY_LEN: usize = 1952;
pub const DILITHIUM_SECRET_KEY_LEN: usize = 4032;
pub const DILITHIUM_SIGNATURE_LEN: usize = 3309;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Encode, Decode, DecodeWithMemTracking, TypeInfo, MaxEncodedLen)]
pub struct DilithiumPublicKey(pub [u8; DILITHIUM_PUBLIC_KEY_LEN]);

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, DecodeWithMemTracking, TypeInfo, MaxEncodedLen)]
pub struct DilithiumSignature(pub [u8; DILITHIUM_SIGNATURE_LEN]);

impl serde::Serialize for DilithiumPublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for DilithiumPublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        if bytes.len() != DILITHIUM_PUBLIC_KEY_LEN {
            return Err(serde::de::Error::custom("invalid public key length"));
        }
        let mut arr = [0u8; DILITHIUM_PUBLIC_KEY_LEN];
        arr.copy_from_slice(&bytes);
        Ok(DilithiumPublicKey(arr))
    }
}

#[derive(Clone, Debug)]
pub struct DilithiumSecretKey(pub [u8; DILITHIUM_SECRET_KEY_LEN]);

#[derive(Clone, Debug)]
pub struct DilithiumKeyPair {
    pub public: DilithiumPublicKey,
    pub secret: DilithiumSecretKey,
}

impl core::fmt::Display for DilithiumPublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Default for DilithiumPublicKey { fn default() -> Self { Self([0u8; DILITHIUM_PUBLIC_KEY_LEN]) } }
impl Default for DilithiumSignature { fn default() -> Self { Self([0u8; DILITHIUM_SIGNATURE_LEN]) } }
impl Default for DilithiumSecretKey { fn default() -> Self { Self([0u8; DILITHIUM_SECRET_KEY_LEN]) } }

impl Verify for DilithiumSignature {
    type Signer = DilithiumPublicKey;
    fn verify<L: Lazy<[u8]>>(&self, mut msg: L, signer: &Self::Signer) -> bool {
        let sig = DSig::from_slice(&self.0);
        DKP::verify(&signer.0, &sig, msg.get(), &[], ML_DSA_65)
    }
}

impl IdentifyAccount for DilithiumPublicKey {
    type AccountId = Self;
    fn into_account(self) -> Self::AccountId { self }
}

pub fn generate_keypair() -> DilithiumKeyPair {
    #[cfg(feature = "std")]
    let seed = {
        let mut s = [0u8; 32];
        getrandom::getrandom(&mut s).unwrap_or_default();
        s
    };
    #[cfg(not(feature = "std"))]
    let seed = [0u8; 32];
    let kp = DKP::generate_deterministic(ML_DSA_65, &seed);
    kp_to_quanta(&kp)
}

pub fn generate_keypair_deterministic(seed: &[u8; 32]) -> DilithiumKeyPair {
    let kp = DKP::generate_deterministic(ML_DSA_65, seed);
    kp_to_quanta(&kp)
}

fn kp_to_quanta(kp: &DKP) -> DilithiumKeyPair {
    let pk_raw = kp.public_key();
    let sk_raw = kp.private_key();
    let mut pk = [0u8; DILITHIUM_PUBLIC_KEY_LEN];
    let mut sk = [0u8; DILITHIUM_SECRET_KEY_LEN];
    pk.copy_from_slice(pk_raw);
    sk.copy_from_slice(sk_raw);
    DilithiumKeyPair { public: DilithiumPublicKey(pk), secret: DilithiumSecretKey(sk) }
}

pub fn sign_message(message: &[u8], kp: &DilithiumKeyPair) -> DilithiumSignature {
    let inner = DKP::from_keys(&kp.secret.0, &kp.public.0, ML_DSA_65)
        .expect("sign_message: invalid keypair");
    #[cfg(feature = "std")]
    let sig = inner.sign(message, &[]).expect("sign_message: signing failed");
    #[cfg(not(feature = "std"))]
    let sig = inner.sign_deterministic(message, &[], &[0u8; 32])
        .expect("sign_message: deterministic signing failed");
    let mut out = [0u8; DILITHIUM_SIGNATURE_LEN];
    out.copy_from_slice(sig.as_bytes());
    DilithiumSignature(out)
}

pub fn verify_dilithium_signature(
    message: &[u8],
    signature: &[u8; DILITHIUM_SIGNATURE_LEN],
    public_key: &[u8; DILITHIUM_PUBLIC_KEY_LEN],
) -> bool {
    let sig = DSig::from_slice(signature.as_ref());
    DKP::verify(public_key, &sig, message, &[], ML_DSA_65)
}

pub fn dilithium_to_address(pk: &DilithiumPublicKey) -> String {
    let hex: String = pk.0.iter().map(|b| format!("{:02x}", b)).collect();
    let end = if hex.len() > 40 { 40 } else { hex.len() };
    format!("QR{}", &hex[..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair() {
        let kp = generate_keypair_deterministic(&[0x42u8; 32]);
        assert_eq!(kp.public.0.len(), 1952);
        assert_eq!(kp.secret.0.len(), 4032);
        assert_ne!(kp.public.0, [0u8; 1952]);
    }

    #[test]
    fn test_sign_verify() {
        let kp = generate_keypair_deterministic(&[0x42u8; 32]);
        let msg = b"hello quanta l1";
        let sig = sign_message(msg, &kp);
        assert!(verify_dilithium_signature(msg, &sig.0, &kp.public.0));
    }

    #[test]
    fn test_wrong_sig_fails() {
        let kp = generate_keypair_deterministic(&[0x42u8; 32]);
        assert!(!verify_dilithium_signature(b"msg", &[0u8; 3309], &kp.public.0));
    }

    #[test]
    fn test_wrong_msg_fails() {
        let kp = generate_keypair_deterministic(&[0x42u8; 32]);
        let sig = sign_message(b"real msg", &kp);
        assert!(!verify_dilithium_signature(b"fake msg", &sig.0, &kp.public.0));
    }

    #[test]
    fn test_address() {
        let kp = generate_keypair_deterministic(&[0x42u8; 32]);
        let a = dilithium_to_address(&kp.public);
        assert!(a.starts_with("QR"));
        assert_eq!(a.len(), 42);
    }

    #[test]
    fn test_verify_trait() {
        let kp = generate_keypair_deterministic(&[0x42u8; 32]);
        let msg = b"test";
        let sig = sign_message(msg, &kp);
        assert!(sig.verify(&msg[..], &kp.public));
    }

    #[test]
    fn test_deterministic() {
        let a = generate_keypair_deterministic(&[0x42u8; 32]);
        let b = generate_keypair_deterministic(&[0x42u8; 32]);
        assert_eq!(a.public.0, b.public.0);
    }

    #[test]
    fn test_roundtrip() {
        let kp = generate_keypair_deterministic(&[0x42u8; 32]);
        let msg = b"QUANTA test";
        let sig = sign_message(msg, &kp);
        assert!(sig.verify(&msg[..], &kp.public));
    }

    #[test]
    fn test_multi_msgs() {
        let kp = generate_keypair_deterministic(&[0x42u8; 32]);
        let msgs: [&[u8]; 3] = [b"a", b"hello world", b""];
        for m in &msgs {
            let sig = sign_message(m, &kp);
            assert!(verify_dilithium_signature(m, &sig.0, &kp.public.0));
        }
    }

    #[test]
    fn test_keypair_uniqueness() {
        // This test ensures real entropy is used (not stub returning zeros).
        // With the old global getrandom stub, both keypairs would be identical
        // because both would get all-zero seeds.
        let a = generate_keypair();
        let b = generate_keypair();
        assert_ne!(a.public.0, b.public.0, "generate_keypair() must produce distinct public keys with real entropy");
        assert_ne!(a.secret.0, b.secret.0, "generate_keypair() must produce distinct secret keys with real entropy");
    }
}

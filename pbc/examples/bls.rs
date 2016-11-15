// examples/bls.rs

extern crate pbc;

use pbc::random::Stream;
use pbc::bls::BLS;

/// PBC with Rustic API.
///
/// Conversion from crypto-notation:
/// a = a_public_k
/// A = a_private_k
/// b = b_public_k
/// B = b_private_k
/// P = generator_point
/// S = a_shared_s = b_shared_s
/// m = message
/// M = H(m) = a_m_digest = b_m_digest
fn main() {

    let bls = BLS::new();

    //NOTE: generator point has to be known by both parties but it is not fixed
    //NOTE: http://crypto.stackexchange.com/questions/40566/bls-signature-choice-of-generator
    let generator_point = bls.generator();

    // Alice's public/private keypair
    // let a_private_k : Scalar = bls.private_key();
    // let a_public_k: Point  = a * generator_point;

    // Bob's public/private keypair
    // let b_private_k : Scalar = bls.scalar();
    // let b_public_k: Point  = b * generator_point;

    // Assume Alice and Bob have securely obtained each other's public keys.

    // Alice computes their shared secret using Bob's public key.
    // let a_shared_s: Point = a * b_public_k;

    // Bob computes their shared secret using Alice's public key.
    // let b_shared_s: Point = b * a_public_k;

    // They had better be the same!
    // assert_eq!(a_shared_s, b_shared_s);

    // Now Alice wants to send an authenticated message m to Bob

    // let message = "Hello, it's Alice.. pbc is cool!";

    // NOTE: hash function is crypto-secure and the algo is public
    // NOTE: `m_digest` is a Point
    // let a_m_digest: Point = bls.hash(message);

    // Alice's signature is a single group element (Point of the curve)
    // let a_sig_m: Point = a * m_digest;

    // Assume Bob got Alice's message and signature

    // let b_m_digest = bls.hash(message);
    // assert_eq!(a_m_digest, b_m_digest);

    // assert!(bls.verify(generator, a_private_k, b_m_digest, b_shared_s));
}

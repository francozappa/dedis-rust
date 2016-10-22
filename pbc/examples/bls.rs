// examples/bls.rs

extern crate pbc;

use pbc::random::Stream;
use pbc::bls::BLS;

// // NOTE: `nil` substrituted with `GP`
// // NOTE: methods names are snake_cased (Rust convention)
#[allow(non_snake_case)]
fn main() {

    // // Crypto setup
    let suite = BLS::new();
    let GP = suite.generator();

    // FIXME: who decide random Scalar range?
    // // Alice's public/private keypair
    // let a = suite.scalar().pick(Stream::new());
    // let A = suite.point().mul(GP, a);

    // // Bob's public/private keypair
    // let b = suite.scalar().pick(Stream::new());
    // let B = suite.point().mul(GP, b);

    // // Assume Alice and Bob have securely obtained each other's public keys.

    // // Alice computes their shared secret using Bob's public key.
    // let SA = suite.point().mul(B, a);

    // // Bob computes their shared secret using Alice's public key.
    // let SB = suite.point().mul(A, b);

    // // They had better be the same!
    // assert_eq!(SA, SB);
    // println!("Shared secret: {:?}", SA);

    // // Now Alice wants to send an authenticated message m to Bob

    // let m = "Hello, it's Alice.. pbc is cool!";
    // // Hash is crypto-secure and the algo is public
    // let AM = suite.hash(m);
    // // Alice's signature is a single group element (Point of the curve)
    // let AS = suite.point().mul(AM, a);

    // // Assume Bob got Alice's message m and signature ASig

    // let BM = suite.hash(m);
    // let BVerify = suite.verify(GP, A, BM, AS);
}

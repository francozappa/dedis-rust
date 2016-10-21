// examples/bls.rs

extern crate pbc;

use pbc::random;
use pbc::bls;
use pbc::bls::BLSScheme;

// // NOTE: `nil` substrituted with `GP`
// // NOTE: methods names are snake_cased (Rust convention)
fn main() {

    println!("ciao");

    // // Crypto setup
    // let suite = bls::bls();
    // let GP = suite.generator();

    // // Alice's public/private keypair
    // let a = suite.scalar().pick();
    // let A = suite.point().mul(GP, a);

    // // Bob's public/private keypair
    // let b = suite.scalar().pick();
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

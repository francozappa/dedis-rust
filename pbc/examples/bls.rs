// examples/bls.rs

extern crate pbc;

// NOTE: `nil` substrituted with `P`
fn main() {

    // Crypto setup
    let suite = pbc::BLSShortSignature();
    let P = suite.P();

    // Alice's public/private keypair
    let a = suite.Scalar().Pick(random.Stream); // Alice's private key
    let A = suite.Point().Mul(P, a);          // Alice's public key

    // Bob's public/private keypair
    let b = suite.Scalar().Pick(random.Stream); // Alice's private key
    let B = suite.Point().Mul(P, b);          // Alice's public key

    // Assume Alice and Bob have securely obtained each other's public keys.

    // Alice computes their shared secret using Bob's public key.
    let SA = suite.Point().Mul(B, a);

    // Bob computes their shared secret using Alice's public key.
    let SB = suite.Point().Mul(A, b);

    // They had better be the same!
    assert_eq!(SA, SB);
    println!("Shared secret: {:?}", SA)

    // Now Alice wants to send an authenticated message m to Bob

    let m = "Hello, it's Alice.. pbc is cool!"
    let AM = suite.Hash(m)                       // Hash is crypto-secure and public
    let AS = suite.Point().Mul(M, a)          // Alice's signature is a single group element

    // Assume Bob got Alice's message m and signature ASig

    let BM = suite.Hash(m)
    let BVerify = suite.Verify(P, A, BM, AS)

}

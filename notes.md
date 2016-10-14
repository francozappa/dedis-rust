# NOTES #

## TODISCUSS ##

* API:
    * provide general purpose crypto framework (like rust-crypto)
    * or provide directly crypto-systems (like sodiumoxide)
    * links
        * https://github.com/rust-lang/rust/issues/14655

* Security guarantees:
    * implementation
        * no timing attack
        * no branch prediction attack
    * curve
        * twisted analysis

* Repository:
    * tabs or spaces (I saw that `dedis/crypto` uses Tabs)

* Runtime model:
    * single or multi-threaded ?
        * If multi, which paradigm
    * single or multi-process ?
        * If multi, which paradigm

* Optimizations:
    * in line functions?
    * Rustc?  Nightly
    * Target platform?
    * Assembly?

## Goals ##

* Bryan, Ismail
    * industrial environments availability of efficient crypto
    * based on elliptic curves such as Ed25519
    * implemented in a more secure/type safe not garbage collected
    * for efficiency, deterministic performance and cross-language linkability

* Generic library
    * user change curves and suites
    * Eg curve: ECDSA with NIST or with Edwards curve
    * Eg suite: swap ECDSA and DSA

* Similar projects
    * go
        * [dedis crypto](https://godoc.org/github.com/dedis/crypto)
    * rust
        * [rust-crypto](https://github.com/DaGenix/rust-crypto)
        * [zcash-bn pairing](https://github.com/zcash/bn)
        * [octavo](https://github.com/libOctavo/octavo.git)
        * [ring](https://github.com/briansmith/ring)
        * [crypto-bench](https://github.com/briansmith/crypto-bench)
        * [libs.rs](http://libs.rs/cryptography/)
        * [sodiumoxide libsodium binding](https://github.com/dnaq/sodiumoxide)

* Rust motivations
    * code benefits from a modern, safe, fast system programming language

* ICS motivations
    * most ICS protocol are cryptographically unprotected
        * no Confidentiality, integrity and availability

* ECC motivations
    * general pros:
        * small key size but high security level (scales much better than RSA)
            * less computation power and memory needed
            * eg: 256-bit key, up to 128-bit security level
        * no index-calculus attacks
            * unlike DLP mod p
        * generic DLP attack methods:
            * Shanks’ baby-step giant-step method
            * Pollard's rho method
        * low computational power needed
            * Eg: RFID, smart phones
    * ECDSA pros:
        * fast signature and verification
        * deterministic algo allows deterministic implementation
        * random same message signatures (nonce based)
            * eg: sign twice same message, get two different signatures
    * ECDSA cons:
        * malleable signatures

    * Ed25519 pros
        * NIST std curve security opinable
        * Bitcoin is not using them


## Crypto ##

* General resources
    * [awesome crypto](https://github.com/sobolevn/awesome-cryptography)
    * [prime number glossary](http://primes.utm.edu/glossary/home.php)
    * Ben Lynn (stan)
    * http://crypto.stackexchange.com/

* Libs
    * [libsodium](https://download.libsodium.org/doc/)
    * [pairing based crypto](https://crypto.stanford.edu/pbc/)
    * [pgp](https://www.gnu.org/software/libgcrypt/)
    * [tls, ssl](https://www.openssl.org/)
    * [cryptopp c++](http://www.cryptopp.com/wiki/Main_Page)
    * [bcprov java](https://www.bouncycastle.org/latest_releases.html)
    * [pycrypto](https://www.dlitz.net/software/pycrypto/)
    * [BouncyCastle](http://bouncycastle.org/)

* Books
    * http://cacr.uwaterloo.ca/hac/
    * http://www.crypto-textbook.com/

* Zero Knowledge
    * https://blog.cryptographyengineering.com/2014/11/27/zero-knowledge-proofs-illustrated-primer/

### Elliptic Curve Crypto ###

* Links:
    * websites
        * http://hyperelliptic.org/
        * https://www.imperialviolet.org/2010/12/04/ecc.html
        * https://crypto.stanford.edu/pbc/notes/elliptic/
        * https://en.wikipedia.org/wiki/Edwards_curve
    * Formulae
        * http://hyperelliptic.org/EFD/index.html
    * Papers
        * see `d-phd/dedis`
        * http://link.springer.com/article/10.1007/s10623-015-0087-1
    * Benchmarks
        * http://bench.cr.yp.to/

* ECC Weierstrass form
    * E: y^2 = x^3 + ax + b
    * defined over a finite field F\_p: #E(F\_p) points + neutral (infinity)
    * where p > 3, prime number
    * and a, b are taken from F\_p
    * crypto acyclic subgroup given choosing p large prime
    * resulting in acyclic group of n points (order of the sub-group)
    * G is a generator point able to generate all #E(F\_p) points

* Primes
    * Mersenne
        * faster reduction

* Classification
    * with endomorphism
    * with side-channel protection
    * with batch-verification


* NIST std curves (FIPS 186-4 std) are secure?
    * http://crypto.stackexchange.com/questions/10263/should-we-trust-the-nist-recommended-ecc-parameters
    * different names
        * FIPS: P-192, ....
        * practice: `nistp192`
        * Certicom: `secp192r1`
        * OpenSSL: `prime192v1`
    * Bitcoin does not use them
        * uses Certicom's `secp256k1`

* Robust implementation (HW and/or SW):
    * select crypto-strong EC
        * Eg: Ed25519
    * curve's twists security
        * TODO
    * sufficient big randomness pool (high-entropy key)
        * see Debian OpenSSL vulnerabilities
    * https://cryptocoding.net/index.php/Coding_rules
        * Eg: no secret branching (no branch-prediction unit leakage)
        * Eg: no secret memory location (no cache-timing, hyper threading attacks)

* Side-channel attacks (invasive, non invasive):
    * types
        * invasive vs non-invasive
        * high-frequency vs low-frequency
        * hardware equipment vs software equipment
        * trigger-based vs non-trigger based (both HW and SW)
        * hardware vs software
    * physical parameters
        * time
        * electro-magnetic waves
        * electrical power
        * audio waves

* Scalar-by-point multiplication algos
    * naive (vulnerable)
        * double-and-sometimes multiply
    * standard (platform dependent)
        * w-ary non-adjacent form (wNAF) over prime sized field
            * Eg: NIST P-curves, secp256k1

* Software implementation
    * Constant time and constant memory
        * https://github.com/bitcoin/bitcoin/tree/master/src/secp256k1

## Type System ##

### dedis/crypto public API ###

Go code example for ECDH:

    // Crypto setup: NIST-standardized P256 curve with AES-128 and SHA-256
    suite := nist.NewAES128SHA256P256()

    // Alice's public/private keypair
    a := suite.Scalar().Pick(random.Stream) // Alice's private key
    A := suite.Point().Mul(nil, a)          // Alice's public key

    // Bob's public/private keypair
    b := suite.Scalar().Pick(random.Stream) // Alice's private key
    B := suite.Point().Mul(nil, b)          // Alice's public key

    // Assume Alice and Bob have securely obtained each other's public keys.

    // Alice computes their shared secret using Bob's public key.
    SA := suite.Point().Mul(B, a)

    // Bob computes their shared secret using Alice's public key.
    SB := suite.Point().Mul(A, b)

    // They had better be the same!
    if !SA.Equal(SB) {
        panic("Diffie-Hellman key exchange didn't work")
    }
    println("Shared secret: " + SA.String())


### Requirements ###

* Requirements:
    * Integrity
    * Confidentiality
    * Availability
    * PerfectForwardSecrecy
    * FormatPreservingCrypto
    * Symmetric (alias SecretKey)
    * Asymmetric (alias PublicKey)
    * Deterministic
    * Randomized

* Applications:
    * KeyExchange
        * has
        * is
    * Encryption
        * has
        * is
    * MessageAuthenticationCode
        * has
        * is
    * ZeroKnoledgeProof
        * has
        * is
    * DigitalSignature
        * has
        * is

* Schemes:
    * AuthenticatedEncryption
        * has
        * is
    * ElGamal
        * has
        * is
    * RSA
        * has
        * is
    * DiffieHellman
        * has
        * is

* Primitives:
    * StreamCipher
        * has
        * is
    * BlockCipher
        * has
        * is
    * AES
        * has
        * is: BlockCipher
    * DES
        * has
        * is: BlockCipher
    * EllipticCurve
        * has: a, b, p, P, order
        * is: Group, DiscreteLogarithm

    * TrapDoorFunction (alias OneWayFunction)
        * has
        * is

* Functions
    * HashFunction
    * PseudoRandomFunction
    * PseudoRandomPermutation

* Problems
    * DiscreteLogarithm
        * has (struct fields) cyclic_group
    * PrimeFactorization


## Rust ##

### External Crates ###


* [stdx collection](https://github.com/brson/stdx)
    * contains official rust-lang crates
    * and others
* lint
    * [clippy](https://github.com/Manishearth/rust-clippy)
* constant time run
    * [nadeko](https://github.com/klutzy/nadeko)
    * [ctgrind](https://github.com/ebfe/rust-ctgrind)
        * https://github.com/agl/ctgrind

* Fuzzing, static analysis on build
    * see ring repo

* Benchmarking
    * [rust crypto bench](https://github.com/briansmith/crypto-bench)
    * [results](http://bench.cr.yp.to/index.html)

* Data validation
    * input
        * [untrusted](https://github.com/briansmith/untrusted)
    * output
        * `Result` enum with error codes
        * `panic!`

* Coverage
    * TODO

* Testing
    * unit tests
    * integration tests

* Documentation
    * docs.rs


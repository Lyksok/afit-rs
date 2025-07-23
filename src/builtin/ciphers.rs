/********** Cesar Cipher **********/

use crate::builtin::basic_arithmetics::{bezout, gcd};

use super::builtins::modulo;

/* Cesar's cipher encryption
   @param k is an integer corresponding to key
   @param m word to cipher.
   @param b base ; for ASCII codes should be set to 256.
*/
pub fn encrypt_cesar(k: i64, m: &[i64], b: i64) -> Vec<i64> {
    m.iter().map(|c| modulo(c + k + b, b)).collect()
}

/* Cesar's cipher decryption
   @param k is an integer corresponding to key
   @param m encrypted word.
   @param b base ; for ASCII code should be set to 256.
*/
pub fn decrypt_cesar(k: i64, m: &[i64], b: i64) -> Vec<i64> {
    m.iter().map(|c| modulo(c - k + b, b)).collect()
}

/********** RSA Cipher **********/

/* Generate an RSA ciphering keys.
    Involved prime numbers need to be distinct. Output is a couple
    of public, private keys.
    @param p prime number
    @param q prime number
*/
pub fn generate_keys_rsa(p: i64, q: i64) -> ((i64, i64), (i64, i64)) {
    let n = p * q;
    let phi = (p - 1) * (q - 1);
    let mut e = -1;
    for n in (1..=phi - 1).rev() {
        if gcd(n, phi) == 1 {
            e = n;
            break;
        }
    }
    if e == -1 {
        panic!("No number");
    }
    ((n, e), (n, bezout(e, phi).0))
}

/* Encryption using RSA cryptosystem.
   @param m integer hash of message
   @param pub_key a tuple (n, e) composing public key of RSA cryptosystem.
*/
pub fn encrypt_rsa(m: i64, (n, e): (i64, i64)) -> i64 {
    0
}

/* Decryption using RSA cryptosystem.
   @param m integer hash of encrypter message.
   @param pub_key a tuple (n, d) composing private key of RSA cryptosystem.
*/
pub fn decrypt_rsa(m: i64, (n, e): (i64, i64)) -> i64 {
    0
}

/********** ElGamal Cipher **********/

/* Generate ElGamal public data. Generates a couple (g, p)
   where p is prime and g primitive root in F_p.
   @param p is prime having form 2*q + 1 for prime q.
*/
pub fn public_data_g(p: i64) -> (i64, i64) {
    (0, 0)
}

/* Generate ElGamal public and private keys.
   @param pub_data a tuple (g, p) of public data for ElGamal cryptosystem.
*/
pub fn generate_keys_g(g: i64, p: i64) -> (i64, i64) {
    (0, 0)
}

/* ElGamal encryption process.
   @param msg message to be encrypted.
   @param pub_data a tuple (g, p) of ElGamal public data.
   @param kA ElGamal public key.
*/
pub fn encrypt_g(msg: i64, (g, p): (i64, i64), kA: i64) -> (i64, i64) {
    (0, 0)
}

/* ElGamal decryption process.
   @param msg a tuple (msg_a, msg_b) forming an encrypted ElGamal message.
   @param a private key
   @param pub_data a tuple (g, p) of public data for ElGamal cryptosystem.
*/
pub fn decrypt_g((msg_a, msg_b): (i64, i64), a: i64, (g, p): (i64, i64)) -> i64 {
    0
}

// ========================= TESTING =========================

fn str2list(s: &str) -> Vec<i64> {
    let mut result = vec![];
    for c in s.chars() {
        result.push(c as i64);
    }
    result
}

pub fn test_encrypt_cesar() {
    let cases = vec![
        ((2, vec![2, 3, 6], 10), vec![4, 5, 8]),
        ((0, str2list("hello"), 255), str2list("hello")),
        ((2, str2list("ABC"), 255), str2list("CDE")),
        ((-1, str2list("xyz"), 255), str2list("wxy")),
    ];

    for ele in cases {
        let ((k, m, b), res) = ele;
        let result = encrypt_cesar(k, &m, b);
        if result == res {
            println!("encrypt_cesar({},{:?},{})={:?} passed", k, m, b, res);
        } else {
            println!(
                "encrypt_cesar({},{:?},{})={:?} error: expected {:?}",
                k, m, b, result, res
            );
        }
    }
}

pub fn test_decrypt_cesar() {
    let cases = vec![
        ((2, vec![4, 5, 8], 10), vec![2, 3, 6]),
        ((0, str2list("hello"), 255), str2list("hello")),
        ((2, str2list("CDE"), 255), str2list("ABC")),
        ((-1, str2list("wxy"), 255), str2list("xyz")),
    ];

    for ele in cases {
        let ((k, m, b), res) = ele;
        let result = decrypt_cesar(k, &m, b);
        if result == res {
            println!("decrypt_cesar({},{:?},{})={:?} passed", k, m, b, res);
        } else {
            println!(
                "decrypt_cesar({},{:?},{})={:?} error: expected {:?}",
                k, m, b, result, res
            );
        }
    }
}

pub fn test_generate_keys_rsa() {
    let cases = vec![((9967, 9973), true)];
    let is_inverse = |x: i64, y: i64, n: i64| (modulo((modulo(x, n)) * (modulo(y, n)), n)) == 1;

    for ele in cases {
        let ((p, q), exp) = ele;
        let ((_, e), (_, d)) = generate_keys_rsa(p, q);
        let phi = (p - 1) * (q - 1);
        if modulo(e, phi) == 1 {
            if !exp {
                println!("test_generate_keys_rsa({p},{q})=false passed");
            } else {
                println!("test_generate_keys_rsa({p},{q})=true error: expected false");
            }
        } else {
            let res = is_inverse(e, d, phi);
            if res {
                println!("test_generate_keys_rsa({p},{q})={res} passed");
            } else {
                println!("test_generate_keys_rsa({p},{q})=false error: expected true");
            }
        }
    }
}

pub fn test_ciphers() {
    test_encrypt_cesar();
    println!();
    test_decrypt_cesar();
    println!();
    test_generate_keys_rsa();
    println!()
}

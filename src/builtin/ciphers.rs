/********** Cesar Cipher **********/

use super::builtins::modulo;

/* Cesar's cipher encryption
   @param k is an integer corresponding to key
   @param m word to cipher.
   @param b base ; for ASCII codes should be set to 256.
*/
pub fn encrypt_cesar(k: i32, m: &[i32], b: i32) -> Vec<i32> {
    m.iter().map(|c| modulo(c + k + b, b)).collect()
}

/* Cesar's cipher decryption
   @param k is an integer corresponding to key
   @param m encrypted word.
   @param b base ; for ASCII code should be set to 256.
*/
pub fn decrypt_cesar(k: i32, m: &[i32], b: i32) -> Vec<i32> {
    m.iter().map(|c| modulo(c - k + b, b)).collect()
}

/********** RSA Cipher **********/

/* Generate an RSA ciphering keys.
    Involved prime numbers need to be distinct. Output is a couple
    of public, private keys.
    @param p prime number
    @param q prime number
*/
pub fn generate_keys_rsa(p: i32, q: i32) -> ((i32, i32), (i32, i32)) {
    ((0, 0), (0, 0))
}

/* Encryption using RSA cryptosystem.
   @param m integer hash of message
   @param pub_key a tuple (n, e) composing public key of RSA cryptosystem.
*/
pub fn encrypt_rsa(m: i32, (n, e): (i32, i32)) -> i32 {
    0
}

/* Decryption using RSA cryptosystem.
   @param m integer hash of encrypter message.
   @param pub_key a tuple (n, d) composing private key of RSA cryptosystem.
*/
pub fn decrypt_rsa(m: i32, (n, e): (i32, i32)) -> i32 {
    0
}

/********** ElGamal Cipher **********/

/* Generate ElGamal public data. Generates a couple (g, p)
   where p is prime and g primitive root in F_p.
   @param p is prime having form 2*q + 1 for prime q.
*/
pub fn public_data_g(p: i32) -> (i32, i32) {
    (0, 0)
}

/* Generate ElGamal public and private keys.
   @param pub_data a tuple (g, p) of public data for ElGamal cryptosystem.
*/
pub fn generate_keys_g(g: i32, p: i32) -> (i32, i32) {
    (0, 0)
}

/* ElGamal encryption process.
   @param msg message to be encrypted.
   @param pub_data a tuple (g, p) of ElGamal public data.
   @param kA ElGamal public key.
*/
pub fn encrypt_g(msg: i32, (g, p): (i32, i32), kA: i32) -> (i32, i32) {
    (0, 0)
}

/* ElGamal decryption process.
   @param msg a tuple (msg_a, msg_b) forming an encrypted ElGamal message.
   @param a private key
   @param pub_data a tuple (g, p) of public data for ElGamal cryptosystem.
*/
pub fn decrypt_g((msg_a, msg_b): (i32, i32), a: i32, (g, p): (i32, i32)) -> i32 {
    0
}

// ========================= TESTING =========================

fn str2list(s: &str) -> Vec<i32> {
    let mut result = vec![];
    for c in s.chars() {
        result.push(c as i32);
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

pub fn test_ciphers() {
    test_encrypt_cesar();
    println!();
    test_decrypt_cesar();
    println!();
}

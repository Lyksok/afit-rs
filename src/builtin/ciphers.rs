/********** Cesar Cipher **********/

/* Cesar's cipher encryption
   @param k is an integer corresponding to key
   @param m word to cipher.
   @param b base ; for ASCII codes should be set to 256.
*/
pub fn encrypt_cesar(k: i32, m: &[i32], b: i32) -> Vec<i32> {
    vec![]
}

/* Cesar's cipher decryption
   @param k is an integer corresponding to key
   @param m encrypted word.
   @param b base ; for ASCII code should be set to 256.
*/
pub fn decrypt_cesar(k: i32, m: &[i32], b: i32) -> Vec<i32> {
    vec![]
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

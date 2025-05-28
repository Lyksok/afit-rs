use super::{builtins::modulo, power::mod_power};

/* Deterministic primality test */
pub fn is_prime(n: i32) -> bool {
    let mut i = 2;
    while i * i <= n {
        if modulo(n, i) == 0 {
            return false;
        }
        i += 1;
    }
    true
}

/* Primality test based on small Fermat theorem
   @param p tested integer
   @param testSeq sequence of integers against which to test
*/
pub fn is_pseudo_prime(p: i32, test_seq: &[i32]) -> bool {
    for elt in test_seq {
        if mod_power(*elt, p, p) != modulo(*elt, p) {
            return false;
        }
    }
    true
}

// ========================= TESTING =========================

pub fn test_is_prime() {
    let cases = vec![
        (2, true),
        (3, true),
        (5, true),
        (7, true),
        (11, true),
        (13, true),
        (4, false),
        (6, false),
        (12, false),
        (45, false),
        (77, false),
        (63, false),
    ];

    for ele in cases {
        let result = is_prime(ele.0);
        if result == ele.1 {
            println!("is_prime({})={} passed", ele.0, result);
        } else {
            println!("is_prime({})={} error: expected {}", ele.0, result, ele.1);
        }
    }
}

pub fn test_is_pseudo_prime() {
    let cases = vec![
        ((2, vec![2, 4, 8, 12]), true),
        ((11, vec![2, 4, 5, 20]), true),
        ((23, vec![2, 9, 15, 18]), true),
        ((29, vec![30, 41, 52]), true),
        ((4, vec![2, 9, 15, 18]), false),
        ((22, vec![30, 41, 52]), false),
        ((15, vec![2, 9, 15, 18]), false),
        ((27, vec![30, 41, 52]), false),
    ];

    for ele in cases {
        let result = is_pseudo_prime(ele.0 .0, &ele.0 .1);
        if result == ele.1 {
            println!(
                "is_pseudo_prime({},{:?})={} passed",
                ele.0 .0, ele.0 .1, result
            );
        } else {
            println!(
                "is_pseudo_prime({},{:?})={} error: expected {}",
                ele.0 .0, ele.0 .1, result, ele.1
            );
        }
    }
}

pub fn test_test_primes() {
    test_is_prime();
    println!();
    test_is_pseudo_prime();
    println!();
}

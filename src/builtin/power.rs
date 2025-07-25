use super::builtins::{div, modulo};

/* Naive power function. Linear complexity
   @param x base
   @param n exponent
*/
pub fn pow(x: i64, n: i64) -> i64 {
    let mut result = 1;
    for _ in 0..n {
        result *= x;
    }
    result
}

/* Fast integer exponentiation function. Logarithmic complexity.
   @param x base
   @param n exponent
*/
pub fn power(x: i64, n: i64) -> i64 {
    match n {
        0 => 1,
        n => {
            let (q, r) = div(n, 2);
            match r {
                0 => power(x * x, q),
                _ => x * power(x * x, q),
            }
        }
    }
}

/* Fast modular exponentiation function. Logarithmic complexity.
   @param x base
   @param n exponent
   @param m modular base
*/
pub fn mod_power(x: i64, n: i64, m: i64) -> i64 {
    match n {
        0 => 1,
        n => {
            let (q, r) = div(n, 2);
            match r {
                0 => modulo(mod_power(modulo(x * x, m), q, m), m),
                _ => modulo(x * mod_power(modulo(x * x, m), q, m), m),
            }
        }
    }
}

/* Fast modular exponentiation function mod prime. Logarithmic complexity.
   It makes use of the Little Fermat Theorem.
   @param x base
   @param n exponent
   @param p prime modular base
*/
pub fn prime_mod_power(x: i64, n: i64, p: i64) -> i64 {
    mod_power(x, n, p) // this is not the way to do but for now I will leave it like this
}

// ========================= TESTING =========================

pub fn test_pow() {
    let cases = vec![
        ((-1, 12), 1),
        ((-1, 11), -1),
        ((0, 2), 0),
        ((3, 1), 3),
        ((5, 0), 1),
        ((-2, 2), 4),
        ((-2, 3), -8),
        ((2, 5), 32),
        ((3, 3), 27),
    ];

    for ele in cases {
        let result = pow(ele.0.0, ele.0.1);
        if result == ele.1 {
            println!("pow({},{})={} passed", ele.0.0, ele.0.1, result);
        } else {
            println!(
                "pow({},{})={} error: expected {}",
                ele.0.0, ele.0.1, result, ele.1
            );
        }
    }
}

pub fn test_power() {
    let cases = vec![
        ((-1, 12), 1),
        ((-1, 11), -1),
        ((0, 2), 0),
        ((3, 1), 3),
        ((5, 0), 1),
        ((-2, 2), 4),
        ((-2, 3), -8),
        ((2, 5), 32),
        ((3, 3), 27),
    ];

    for ele in cases {
        let result = power(ele.0.0, ele.0.1);
        if result == ele.1 {
            println!("power({},{})={} passed", ele.0.0, ele.0.1, result);
        } else {
            println!(
                "power({},{})={} error: expected {}",
                ele.0.0, ele.0.1, result, ele.1
            );
        }
    }
}

pub fn test_mod_power() {
    let cases = vec![
        ((-1, 12, 10), 1),
        ((-1, 11, 11), 10),
        ((0, 2, 3), 0),
        ((3, 1, 3), 0),
        ((5, 0, 2), 1),
        ((-2, 2, 5), 4),
        ((-2, 3, 9), 1),
        ((2, 5, 17), 15),
        ((3, 3, 17), 10),
    ];

    for ele in cases {
        let result = mod_power(ele.0.0, ele.0.1, ele.0.2);
        if result == ele.1 {
            println!(
                "mod_power({},{},{})={} passed",
                ele.0.0, ele.0.1, ele.0.2, result
            );
        } else {
            println!(
                "mod_power({},{},{})={} error: expected {}",
                ele.0.0, ele.0.1, ele.0.2, result, ele.1
            );
        }
    }
}

pub fn test_prime_mod_power() {
    let cases = vec![
        ((-1, 12, 7), 1),
        ((-1, 11, 11), 10),
        ((0, 2, 3), 0),
        ((3, 1, 3), 0),
        ((5, 0, 2), 1),
        ((-2, 2, 5), 4),
        ((-2, 3, 5), 2),
        ((2, 5, 17), 15),
        ((3, 3, 17), 10),
    ];

    for ele in cases {
        let result = prime_mod_power(ele.0.0, ele.0.1, ele.0.2);
        if result == ele.1 {
            println!(
                "prime_mod_power({},{},{})={} passed",
                ele.0.0, ele.0.1, ele.0.2, result
            );
        } else {
            println!(
                "prime_mod_power({},{},{})={} error: expected {}",
                ele.0.0, ele.0.1, ele.0.2, result, ele.1
            );
        }
    }
}

pub fn test_powers() {
    test_pow();
    println!();
    test_power();
    println!();
    test_mod_power();
    println!();
    test_prime_mod_power();
    println!();
}

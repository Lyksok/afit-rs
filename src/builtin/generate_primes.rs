use super::{builtins::modulo, test_primes::is_prime};
use std::fs;
use std::io::{Read, Write};

/* List composed of 2 and then odd integers starting at 3.
   @param n limit of list of odd integers, minimum value is 2.
*/
pub fn init_eratosthenes(n: i32) -> Vec<i32> {
    if n < 2 {
        panic!("n must be >= 2");
    }
    let mut result = vec![2];
    for i in (3..=n).step_by(2) {
        result.push(i);
    }
    result
}

/* Eratosthene sieve.
    @param n limit of list of primes, starting at 2.
*/
pub fn eratosthenes(n: i32) -> Vec<i32> {
    if n < 2 {
        panic!("n must be >= 2");
    }
    let init = init_eratosthenes(n);
    let mut result = vec![];
    for elt in init {
        let mut is_multiple = false;
        for r in result.iter() {
            if modulo(elt, *r) == 0 {
                is_multiple = true;
                break;
            }
        }
        if !is_multiple {
            result.push(elt);
        }
    }
    result
}

/* Write a list of prime numbers up to limit into a txt file.
    @param n limit of prime numbers up to which to build up a list of primes.
    @param file path to write to.
*/
pub fn write_list_primes(n: i32, file: &str) {
    let mut file = fs::File::create(file).unwrap();

    for elt in eratosthenes(n) {
        let _ = writeln!(file, "{}", elt);
    }
}

/* Load list of primes into Rust environment.
   @param file path to load from.
*/
pub fn read_list_primes(file: &str) -> Vec<i32> {
    let mut file = fs::File::open(file).unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    content
        .split('\n')
        .filter_map(|elt| elt.trim().parse().ok())
        .collect()
}

/* Get biggest prime.
   @param l list of prime numbers.
*/
pub fn last_element(l: &[i32]) -> i32 {
    if l.is_empty() {
        panic!("The list must not be empty");
    }
    *l.last().unwrap()
}

/* Get two biggest primes.
   @param l list of prime numbers.
*/
pub fn last_two(l: &[i32]) -> (i32, i32) {
    if l.len() < 2 {
        panic!("The len of the list must be >= 2");
    }
    let r = l.last_chunk::<2>().unwrap();
    (r[0], r[1])
}

/* Finding couples of primes where second entry is twice the first
   plus 1.
   @param limit positive integer bounding searched for primes.
   @param isprime function testing for (pseudo)primality.
*/
pub fn double_primes(limit: i32, isprime: fn(i32) -> bool) -> Vec<(i32, i32)> {
    let init = eratosthenes(limit);
    let mut result = vec![];
    for elt in init {
        if isprime(2 * elt + 1) {
            result.push((elt, 2 * elt + 1));
        }
    }
    result
}

/* Finding twin primes.
   @param limit positive integer bounding searched for primes.
   @param isprime function testing for (pseudo)primality.
*/
pub fn twin_primes(limit: i32, isprime: fn(i32) -> bool) -> Vec<(i32, i32)> {
    let init = eratosthenes(limit);
    let mut result = vec![];
    for elt in init {
        if isprime(elt + 2) {
            result.push((elt, elt + 2));
        }
    }
    result
}

// ========================= TESTING =========================

pub fn test_init_eratosthenes() {
    let cases = vec![(2, vec![2]), (3, vec![2, 3]), (6, vec![2, 3, 5])];

    for ele in cases {
        let result = init_eratosthenes(ele.0);
        if result == ele.1 {
            println!("init_eratosthenes({})={:?} passed", ele.0, result);
        } else {
            println!(
                "init_eratosthenes({})={:?} error: expected {:?}",
                ele.0, result, ele.1
            );
        }
    }
}

pub fn test_eratosthenes() {
    let cases = vec![
        (2, vec![2]),
        (3, vec![2, 3]),
        (6, vec![2, 3, 5]),
        (25, vec![2, 3, 5, 7, 11, 13, 17, 19, 23]),
    ];

    for ele in cases {
        let result = eratosthenes(ele.0);
        if result == ele.1 {
            println!("eratosthenes({})={:?} passed", ele.0, result);
        } else {
            println!(
                "eratosthenes({})={:?} error: expected {:?}",
                ele.0, result, ele.1
            );
        }
    }
}

pub fn test_double_primes() {
    let cases = vec![((20, is_prime), vec![(2, 5), (3, 7), (5, 11), (11, 23)])];

    for ele in cases {
        let result = double_primes(ele.0 .0, ele.0 .1);
        if result == ele.1 {
            println!("double_primes({},is_prime)={:?} passed", ele.0 .0, result);
        } else {
            println!(
                "double_primes({},is_prime)={:?} error: expected {:?}",
                ele.0 .0, result, ele.1
            );
        }
    }
}

pub fn test_twin_primes() {
    let cases = vec![((20, is_prime), vec![(3, 5), (5, 7), (11, 13), (17, 19)])];

    for ele in cases {
        let result = twin_primes(ele.0 .0, ele.0 .1);
        if result == ele.1 {
            println!("twin_primes({},is_prime)={:?} passed", ele.0 .0, result);
        } else {
            println!(
                "twin_primes({},is_prime)={:?} error: expected {:?}",
                ele.0 .0, result, ele.1
            );
        }
    }
}

pub fn test_generate_primes() {
    test_init_eratosthenes();
    println!();
    test_eratosthenes();
    println!();
    test_double_primes();
    println!();
    test_twin_primes();
    println!();
}

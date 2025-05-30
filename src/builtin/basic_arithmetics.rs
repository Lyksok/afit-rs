use super::builtins::{div, sign};

/* Greater common (positive) divisor of two non-zero integers.
 * @param a non-zero integer
 * @param b non-zero integer
 */
pub fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a * sign(a);
    let mut b = b * sign(b);

    loop {
        if a < b {
            (a, b) = (b, a);
        }
        let (q, r) = div(a, b);
        if r == 0 {
            return a;
        } else {
            b = q;
            a = r;
        }
    }
}

/* Extended euclidean division of two integers NOT RUST DEFAULT
 * Given non-zero entries a b computes triple (u, v, d) such that
 * a*u + b*v = d and d is gcd of a and b.
 * @param a non-zero integer
 * @param b non-zero integer.
 */
pub fn bezout(a: i32, b: i32) -> (i32, i32, i32) {
    let mut a = a;
    let mut b = b;
    let (mut u1, mut v1, mut u2, mut v2) = (1, 0, 0, 1);

    loop {
        let (q, r) = div(a, b);
        if r == 0 {
            return (u2, v2, b);
        } else {
            a = b;
            b = r;
            (u1, v1, u2, v2) = (u2, v2, u1 - q * u2, v1 - q * v2);
        }
    }
}

// ========================= TESTING =========================

pub fn test_gcd() {
    let cases = vec![((32, 6), 2), ((18, 12), 6), ((-18, -12), 6)];

    for ele in cases {
        let result = gcd(ele.0 .0, ele.0 .1);
        if result == ele.1 {
            println!("gcd({},{})={} passed", ele.0 .0, ele.0 .1, result);
        } else {
            println!(
                "gcd({},{})={} error: expected {}",
                ele.0 .0, ele.0 .1, result, ele.1
            );
        }
    }
}

pub fn test_bezout() {
    let cases = vec![
        ((18, 22), (5, -4, 2)),
        ((22, 18), (-4, 5, 2)),
        ((17, 21), (5, -4, 1)),
        ((21, 17), (-4, 5, 1)),
    ];

    for ele in cases {
        let result = bezout(ele.0 .0, ele.0 .1);
        if result == ele.1 {
            println!("bezout({},{})={:?} passed", ele.0 .0, ele.0 .1, result);
        } else {
            println!(
                "bezout({},{})={:?} error: expected {:?}",
                ele.0 .0, ele.0 .1, result, ele.1
            );
        }
    }
}

pub fn test_basic_arithmetics() {
    test_gcd();
    println!();
    test_bezout();
    println!();
}

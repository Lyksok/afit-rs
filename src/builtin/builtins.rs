/* Sign function
    @param x integer
*/
pub fn sign(x: i32) -> i32 {
    if x < 0 {
        -1
    } else {
        1
    }
}

/* Quotient of an integer by a natural number.
    This is the quotient in euclidiant division sense.
    @param a dividend
    @param b natural number you divide by.
*/
pub fn quot(a: i32, b: i32) -> i32 {
    let s = sign(a);
    let mut a = a * s;
    let mut q: i32 = 0;
    while a >= b {
        q += 1;
        a -= b;
    }
    q * s + if s > 0 || a == 0 { 0 } else { -1 }
}

/* Modulo of two integers.
   Following euclidean division NOT RUST DEFAULT. Positive integer
   between 0 (included) and modulo (excluded) resulting from euclidian
   division of entry by modulo.

   @param a input integer
   @param b moduli integer.
*/
pub fn modulo(a: i32, b: i32) -> i32 {
    let b = if b < 0 { -b } else { b };
    match a {
        a if a * sign(a) >= b => modulo(a - b * sign(a), b),
        a if a < 0 => a + b,
        a => a,
    }
}

/* Division of an integer by a natural number. NOT RUST DEFAULT.
   Division of an integer by a non-zero integer b is the unique couple
   of integers (q, r) such that a = b*q + r and r is in [0, abs b[.
   @param a dividend
   @param b integer you divide by.
*/
pub fn div(a: i32, b: i32) -> (i32, i32) {
    let q = quot(a, b);
    let r = a - b * q;
    (q, r)
}

// ========================= TESTING =========================

pub fn test_sign() {
    let cases = vec![(1, 1), (-1, -1), (0, 1)];

    for ele in cases {
        let result = sign(ele.0);
        if result == ele.1 {
            println!("sign({})={} passed", ele.0, result);
        } else {
            println!("sign({})={} error: expected {}", ele.0, result, ele.1);
        }
    }
}

pub fn test_quot() {
    let cases = vec![((10, 3), 3), ((-10, 3), -4), ((10, 2), 5), ((-10, 2), -5)];

    for ele in cases {
        let result = quot(ele.0 .0, ele.0 .1);
        if result == ele.1 {
            println!("quot({},{})={} passed", ele.0 .0, ele.0 .1, result);
        } else {
            println!(
                "quot({},{})={} error: expected {}",
                ele.0 .0, ele.0 .1, result, ele.1
            );
        }
    }
}

pub fn test_modulo() {
    let cases = vec![((10, 3), 1), ((-10, 3), 2), ((10, 2), 0), ((-10, 2), 0)];

    for ele in cases {
        let result = modulo(ele.0 .0, ele.0 .1);
        if result == ele.1 {
            println!("modulo({},{})={} passed", ele.0 .0, ele.0 .1, result);
        } else {
            println!(
                "modulo({},{})={} error: expected {}",
                ele.0 .0, ele.0 .1, result, ele.1
            );
        }
    }
}

pub fn test_div() {
    let cases = vec![
        ((10, 3), (3, 1)),
        ((-10, 3), (-4, 2)),
        ((10, 2), (5, 0)),
        ((-10, 2), (-5, 0)),
    ];

    for ele in cases {
        let result = div(ele.0 .0, ele.0 .1);
        if result == ele.1 {
            println!("modulo({},{})={:?} passed", ele.0 .0, ele.0 .1, result);
        } else {
            println!(
                "modulo({},{})={:?} error: expected {:?}",
                ele.0 .0, ele.0 .1, result, ele.1
            );
        }
    }
}

pub fn test_builtins() {
    test_sign();
    println!();
    test_quot();
    println!();
    test_modulo();
    println!();
    test_div();
}

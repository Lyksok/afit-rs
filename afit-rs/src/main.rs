use afit_rs::builtin;

fn main() {
    builtin::builtins::test_builtins();
    println!();
    builtin::basic_arithmetics::test_basic_arithmetics();
    println!();
    builtin::power::test_powers();
    println!();
    builtin::test_primes::test_test_primes();
    println!();
}

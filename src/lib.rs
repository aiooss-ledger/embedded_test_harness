#![no_std]

use core::fmt;
use panic_semihosting as _;

#[inline(never)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use cortex_m_semihosting::hio::hstdout;

    if let Ok(mut hstdout) = hstdout() {
        write!(hstdout, "{}", args).ok();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        (_print(format_args!($($arg)*)))
    }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub struct TestType {
    pub modname: &'static str,
    pub name: &'static str,
    pub f: fn() -> Result<(), ()>,
}

pub fn test_runner(tests: &[&TestType]) {
    use cortex_m_semihosting::debug::{exit, EXIT_FAILURE, EXIT_SUCCESS};

    println!("--- {} tests ---", tests.len());
    let mut return_code = EXIT_SUCCESS;
    for t in tests {
        match (t.f)() {
            Ok(()) => print!("\x1b[1;32m   ok   \x1b[0m"),
            Err(()) => {
                print!("\x1b[1;31m  fail  \x1b[0m");
                return_code = EXIT_FAILURE;
            }
        }
        println!("{}::{}", t.modname, t.name);
    }
    exit(return_code);
}

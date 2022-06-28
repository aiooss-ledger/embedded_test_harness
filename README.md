# Embedded test harness for Rust `no_std`

Only supports ARM Cortex-M semi-hosting for now.

## How to use

Add `embedded_test_harness` and
`testmacro = { git = "https://github.com/yhql/testmacro"}`
to Cargo.toml `dev-dependencies`.

Then you may write tests following this template:

```Rust
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

use embedded_test_harness::{test_runner, TestType};

#[cfg(test)]
mod tests {
    use testmacro::test_item as test;
    use embedded_test_harness::TestType;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/// _start is the entrypoint specified in the linker
#[no_mangle]
pub fn _start() -> ! {
    #[cfg(test)]
    test_main();

    loop {}
}
```

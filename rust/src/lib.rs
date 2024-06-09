#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn test(input: u8) -> bool;
    }
}

pub fn test(input: u8) -> bool {
    input > 3
}

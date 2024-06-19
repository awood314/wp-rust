fn main() {
    cxx_build::bridges(&["src/iir_filter.rs", "src/mod_delay.rs"])
        .std("c++20")
        .compile("wp-rust");

    println!("cargo:rerun-if-changed=src/iir_filter.rs");
    println!("cargo:rerun-if-changed=src/mod_delay.rs");
}

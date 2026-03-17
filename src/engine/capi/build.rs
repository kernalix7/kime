fn main() {
    println!("cargo:rerun-if-changed=./kime_engine.h");
    println!("cargo:rerun-if-changed=./kime_engine.hpp");
    println!("cargo:rerun-if-changed=../capi");

    let c_binding = cbindgen::generate_with_config(
        "../capi",
        cbindgen::Config::from_file("../capi/cbindgen-c.toml")
            .expect("Failed to load cbindgen-c.toml"),
    )
    .expect("Failed to generate C bindings");

    c_binding.write_to_file("kime_engine.h");

    let cpp_binding = cbindgen::generate_with_config(
        "../capi",
        cbindgen::Config::from_file("../capi/cbindgen-cpp.toml")
            .expect("Failed to load cbindgen-cpp.toml"),
    )
    .expect("Failed to generate C++ bindings");

    cpp_binding.write_to_file("kime_engine.hpp");
}

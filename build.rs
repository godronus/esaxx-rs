#[cfg(feature = "cpp")]
fn main() {
    cc::Build::new()
        .cpp(false)
        .flag("-fexceptions")
        .file("src/esaxx.cpp")
        .include("src")
        .compile("esaxx");
}

#[cfg(not(feature = "cpp"))]
fn main() {}

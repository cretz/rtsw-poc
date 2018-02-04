
fn main() {
    
    println!("cargo:rustc-link-search=native=vendor/openssl/dist/lib");
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=ssl");

    println!("cargo:rustc-link-search=native=vendor/libevent/dist/lib");
    println!("cargo:rustc-link-lib=static=event");
    
    println!("cargo:rustc-link-search=native=vendor/zlib/dist/lib");
    println!("cargo:rustc-link-lib=static=z");
    
    println!("cargo:rustc-link-search=native=vendor/xz/dist/lib");
    println!("cargo:rustc-link-lib=static=lzma");

    println!("cargo:rustc-link-search=native=vendor/tor/src/ext/ed25519/ref10");
    println!("cargo:rustc-link-lib=static=ed25519_ref10");

    println!("cargo:rustc-link-search=native=vendor/tor/src/ext/ed25519/donna");
    println!("cargo:rustc-link-lib=static=ed25519_donna");

    println!("cargo:rustc-link-search=native=vendor/tor/src/trunnel");
    println!("cargo:rustc-link-lib=static=or-trunnel");

    println!("cargo:rustc-link-search=native=vendor/tor/src/ext/keccak-tiny");
    println!("cargo:rustc-link-lib=static=keccak-tiny");

    println!("cargo:rustc-link-search=native=vendor/tor/src/common");
    println!("cargo:rustc-link-lib=static=curve25519_donna");
    println!("cargo:rustc-link-lib=static=or");
    println!("cargo:rustc-link-lib=static=or-crypto");
    println!("cargo:rustc-link-lib=static=or-ctime");
    println!("cargo:rustc-link-lib=static=or-event");

    println!("cargo:rustc-link-search=native=vendor/tor/src/or");
    println!("cargo:rustc-link-lib=static=tor");
    
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=crypt32");
    println!("cargo:rustc-link-lib=gdi32");
}
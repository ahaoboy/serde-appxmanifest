# serde-appxmanifest

A Rust library for parsing Windows `AppxManifest.xml` and `AppxMetadata\\AppxBundleManifest.xml` files, designed to extract metadata from Universal Windows Platform (UWP) application manifests efficiently.

## Usage

```rust
let Some(p) = std::env::args().nth(1) else {
    println!("serde-appxmanifest <AppxManifest.xml>");
    return;
};

let xml = std::fs::read_to_string(&p).expect("failed to read xml file");

let package: Package = from_str(&xml).expect("failed to parse xml");

println!("{package:?}");
```

fn main() {
    let mut feature_count = 0;
    if cfg!(feature = "shtc1") {
        feature_count += 1;
    }
    if cfg!(feature = "shtc3") {
        feature_count += 1;
    }
    if cfg!(feature = "generic") {
        feature_count += 1;
    }
    if feature_count != 1 {
        panic!("\n\nMust select exactly of the supported sensors as target feature in Cargo.toml!\nChoices: 'shtc1', 'shtc3' or 'generic'.\n\nExample:\n\n  shtcx = { version = \"0.1\", features = [\"shtc3\"] }\n\n");
    }

    println!("cargo:rerun-if-changed=build.rs");
}

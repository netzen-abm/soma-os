// File Path: services/backend-rust/build.rs

fn main() {
    // Tell Cargo to rerun this build script only if the bridge interface mapping modifications occur
    println!("cargo:rerun-if-changed=src/uniffi_bridge.rs");

    // Execute the UniFFI scaffolding generation engine across your mobile gateway file code lines
    uniffi::generate_scaffolding("src/uniffi_bridge.rs")
        .expect("UniFFI compilation exception: Failed to auto-generate native mobile language bridge scaffolding bindings.");
}

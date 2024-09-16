fn main() {
    // This is not an error, the run will be called with the 
    // cli feature flag instead of adding it as a direct dependency
    uniffi::uniffi_bindgen_main()
}


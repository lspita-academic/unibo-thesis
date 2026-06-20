fn main() {
    // The ESP-IDF Rust library exposes high-level abstractions of the hardware
    // components with a single API that supports different devices.
    esp_idf_svc::log::init_from_env();

    let wifi = AsyncWifi::new(...);
    match wifi.connect().await {
        Ok(_) => log::info!("Wifi connected!"),
        Err(e) => log::error!("Error connecting to wifi: {e}");
    }

    // The original C primitives of the framework are also provided for a more
    // fine-grained control over the system.
    esp_idf_svc::sys::xTaskCreatePinnedToCore(...);

    // The declared components bindings are also generated in dedicated
    // modules.
    esp_idf_svc::sys::zenoh_pico::z_open(...);
}

// Example of a bindgen-generated module.
mod zenoh_pico {
    // Comments above definitions of any type become doc comments.
    /// A variable-length encoding unsigned integer.
    pub type _z_zint_t = usize;

    // Typedefs become type aliases.
    // Since this is a function pointer alias, that could be null, it is
    // translated into an `Option` type.
    pub type _z_id_cmp_f = ::core::option::Option<
        unsafe extern "C" fn(left: *const _z_id_t, right: *const _z_id_t) -> ::core::ffi::c_int,
    >

    // Structs are marked with the `repr(C)` attribute to get compiled with a
    // compatible memory layout.
    // They also automatically derive traits that mimic standard C structs
    // behaviours, and all their fields become public.
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct _z_id_t {
        pub id: [u8; 16usize],
    }

    // All symbols are marked as unsafe by default, because the Rust compiler
    // cannot enforce the ownership model rules on external programs.
    unsafe extern "C" {
        // Global variables are marked as `static`.
        pub static empty_id: _z_id_t;

        // Pointers do not become references, and C types exposed by the `core`
        // package are used across all definitions.
        pub fn _z_id_cmp(left: *const _z_id_t, right: *const _z_id_t) -> ::core::ffi::c_int;
    }
}

// The ESP-IDF Rust library exposes high-level abstractions of the hardware
// components with a single API that supports different devices.
let wifi = AsyncWifi::new(...);
wifi.connect().await;

// The original C primitives of the framework are also provided for a more
// fine-grained control over the system.
esp_idf_svc::sys::xTaskCreatePinnedToCore(...);

// The declared components bindings are also generated in dedicated
// modules.
esp_idf_svc::sys::zenoh_pico::z_open(...);

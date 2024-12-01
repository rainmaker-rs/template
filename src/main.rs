fn main() {
    {% if esp -%}
    // It is necessary to call this function once while running on ESP platform. 
    // Otherwise some patches to the runtime implemented by esp-idf-sys might not link properly.
    // See https://github.com/esp-rs/esp-idf-template/issues/71
    {% if linux -%}
    #[cfg(target_os = "espidf")]
    {% endif -%}
    esp_idf_svc::sys::link_patches();

    {% endif -%}

    println!("Hello, world!");
}

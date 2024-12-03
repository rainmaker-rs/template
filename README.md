# Project template for rainmaker-rs

### This is a poject template for getting started with  [rainmaker-rs](https://github.com/rainmaker-rs/rainmaker). 


It perform the project setup as required(sdkonfig, partitions, etc) to that you can get started with builing actual application

---

### Usage
- Install cargo-generate \
`$ cargo install cargo-generate`
- Crate a new project using this template \
`$ cargo generate rainmaker-rs/template` \
Follow the on-screen prompts thereafter.
- Make sure you've performed claiming process for your device before running the application.

---

### Cargo command alias
This template defines a few cargo command alias for easing the development process \
They are as follows:
- ESP32(only available if ESP32 support enabled):
    - `cargo run_esp`: builds and flashes the application for the microcontroller choosen suring project initialization
    - `cargo add_esp`: adds a dependency only for ESP32 targets.
- Linux(only available if Linux support enabled):
    - `cargo run_linux`: builds and runs the application on the host linux system
    - `cargo add_linux`: adds a dependency only for Linux targets.

use esp_idf_hal::delay::FreeRtos;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use log::*;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    
    let peripherals = Peripherals::take().unwrap();
    let sig_in = PinDriver::input(peripherals.pins.gpio18)?;

    loop {
        FreeRtos::delay_ms(1000);
        if sig_in.is_high() {
            info!("HIGH");
        }else {
            info!("LOW");
        }
    }
}

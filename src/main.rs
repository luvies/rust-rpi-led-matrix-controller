use rpi_led_matrix::{LedMatrix, LedMatrixOptions};

fn main() {
    println!("Hello, world!");

    let mut options = LedMatrixOptions::new();
    options.set_hardware_mapping("adafruit-hat");
    options.set_chain_length(1);
    options.set_hardware_pulsing(false);
    //options.set_inverse_colors(true);
    //options.set_refresh_rate(true);
    LedMatrix::new(Some(options)).unwrap();

    ();
}

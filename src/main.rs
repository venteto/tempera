// checked with `cargo clippy`
use std::f64;                // our temperatures are floats
use std::io::{self, Write};  // `print!` fails without this
use std::time::Instant;      // for benchmarking


fn main() {
    temperature_converter_basic();
}

fn temperature_converter_basic() {
    // GET THE SCALE -----------------------------------------------------------
    print!("Enter a temperature scale (F, C or K):  ");
    io::stdout().flush().unwrap();  // `print!` fails without this

    let mut scale_raw = String::new();

    io::stdin()
        .read_line(&mut scale_raw)
        .expect("Failed to read from stdin");

    let scale = scale_raw.trim().to_ascii_uppercase();

    fn is_valid_scale(v: &str) -> bool {
        const SERVICE_VALUES: [&str; 3] = ["F", "C", "K"];
        SERVICE_VALUES.iter().any(|x| *x == v)
    }

    // use implicit newlines
    static TEMPERATURE_RAW_ERR_MSG: &str = "WARN: Not a valid temperature; this tool expects:
    - a base 10 integer or floats only
    - positive or negative numbers
    - '.' as a decimal separator
    - no thousands separator
    - example: -12345.67";

    if is_valid_scale(&scale) {
        // GET THE TEMPERATURE ------------------------- ---------------
        print!("Enter a temperature:  ");
        io::stdout().flush().unwrap();  // `print!` fails without this

        let mut temperature_raw = String::new();

        io::stdin()
            .read_line(&mut temperature_raw)
            .expect("Failed to read from stdin");

        // also remove any comma thousands separator
        let temperature = temperature_raw.trim().replace(',',"");

        let now = Instant::now();  // attempt at benchmarking

        // HANDLE CASES ---- -------------------------------------------
        // for now we do not return any exit code, regardless of OS
        if scale == "F" {  // CONVERT FROM FAHRENHEIT
            match temperature.parse::<f64>() {
                Ok(input) => println!(
                    "  {input}° Fahrenheit = {cel:.2}°C | {kel:.2}K | {ran:.2}°Ra",

                    // convert Fahrenheit to Celsius
                    // Google converter: 92 F = 33.3333 C
                    cel=(temperature.parse::<f64>()
                        .unwrap() - 32.0) * (5.0 / 9.0),

                    // convert Fahrenheit to Kelvin
                    // x °F ≘ (x + 459.67) × 5/9 K
                    // Google converter: 92 F = 306.483 K
                    kel=(temperature.parse::<f64>()
                        .unwrap() + 459.67) * (5.0 / 9.0),

                    // convert Fahrenheit to Rankine
                    // Google converter: 92 F = 551.67 R
                    ran=temperature.parse::<f64>()
                        .unwrap() + 459.67,
                ),
                Err(..) => println!("{TEMPERATURE_RAW_ERR_MSG}")
            }
            // output the execution time of the conversion
            let elapsed = now.elapsed();
            println!("  Conversion execution time: {:.2?}", elapsed);
        } else if scale == "C" {  // CONVERT FROM CELSIUS
            match temperature.parse::<f64>() {
                Ok(input) => println!(
                    "  {input}° Celsius = {fah:.2}°F | {kel:.2}K | {ran:.2}°Ra",

                    // convert Celsius to Fahrenheit
                    // Google converter: 12 C = 53.6 F
                    fah=(temperature.parse::<f64>()
                        .unwrap() * (9.0 / 5.0)) + 32.0,

                    // convert Celsius to Kelvin
                    // Google converter: 12 C = 285.15 K
                    kel=temperature.parse::<f64>()
                        .unwrap() + 273.15,

                    // convert Celsius to Rankine
                    // Google converter: 12 C = 513.27 R
                    ran=(temperature.parse::<f64>()
                        .unwrap() + 273.15) * (9.0 / 5.0),
                ),
                Err(..) => println!("{TEMPERATURE_RAW_ERR_MSG}")
            }
            // output the execution time of the conversion
            let elapsed = now.elapsed();
            println!("  Conversion execution time: {:.2?}", elapsed);
        } else if scale == "K" {  // CONVERT FROM KELVIN
            match temperature.parse::<f64>() {
                Ok(input) => println!(
                    "  {input} Kelvin = {fah:.2}°F | {cel:.2}°C | {ran:.2}°Ra",

                    // convert Kelvin to Fahrenheit
                    // x K ≘ (x × 9/5 − 459.67) °F
                    // Google converter: 15K = -432.67 F
                    fah=(temperature.parse::<f64>()
                        .unwrap() * (9.0 / 5.0)) - 459.67,

                    // convert Kelvin to Celsius
                    // x K ≘ (x − 273.15) °C
                    // Google converter: 15K = -258.15 C
                    cel=temperature.parse::<f64>()
                        .unwrap() - 273.15,

                    // convert Kelvin to Rankine
                    // x K ≘ x × 9/5 °R
                    // Google converter: 15K = 27 R
                    ran=temperature.parse::<f64>()
                        .unwrap() * (9.0 / 5.0),
                ),
                Err(..) => println!("{TEMPERATURE_RAW_ERR_MSG}")
            }
            // output the execution time of the conversion
            let elapsed = now.elapsed();
            println!("  Conversion execution time: {:.2?}", elapsed);
        } else {
            println!("WARN: Bad input, enter F, C, or K");
        } // end of cases IF
    } else {
        println!("WARN: Not a valid scale; enter F, f, C, c, K or k");
    } // end of valid scale IF
} // end of temperature_converter_basic


/*
    TODO:

    comma as thousands separator (locale considerations are not a priority, Rust
    by default ignores locale, which is good enough for me)

    dynamic precision for floats / drop trailing zeros / output integer
    as applicable

    test suite

    implement Rankine conversion block?

    merge scale and value selection into one step and parse the input to decide
    what to do?

    refactor

    break everything other than main out into a separate file/module for the
    heck of it?
*/

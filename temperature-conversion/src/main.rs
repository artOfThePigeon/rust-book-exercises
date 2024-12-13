use std::io;

fn main() {
    println!("Let's convert some temperatures! Do you want to convert to Celsius or Fahrenheit?");

    loop {
        let temp_scale = get_target_scale();
        let temperature = get_temperature();
        let conversion = convert_temperature(temperature, &temp_scale);

        match temp_scale {
            TemperatureScale::Celcius => {
                println!("{temperature}° Fahrenheit converts to {conversion:.2}° Celsius.")
            }
            TemperatureScale::Fahrenheit => {
                println!("{temperature}° Celsius converts to {conversion:.2}° Fahrenheit.")
            }
        }

        if convert_again() {
            continue;
        } else {
            break println!("Have a breezy day!");
        }
    }

    enum TemperatureScale {
        Fahrenheit,
        Celcius,
    }

    fn get_target_scale() -> TemperatureScale {
        loop {
            println!("Please enter the letter F or C for the scale you want to convert to.");
            let mut temp_scale = String::new();
            io::stdin().read_line(&mut temp_scale).expect("error");

            match temp_scale.trim().to_uppercase().as_str() {
                "F" => break TemperatureScale::Fahrenheit,
                "C" => break TemperatureScale::Celcius,
                _ => continue,
            };
        }
    }

    fn get_temperature() -> f64 {
        loop {
            println!("Please enter a temperature:");
            let mut temperature: String = String::new();
            io::stdin().read_line(&mut temperature).expect("error");

            match temperature.trim().parse() {
                Ok(num) => break num,
                Err(_) => continue,
            };
        }
    }
    fn convert_temperature(temp: f64, scale: &TemperatureScale) -> f64 {
        match scale {
            TemperatureScale::Celcius => ((temp - 32.0) * 5.0) / 9.0,
            TemperatureScale::Fahrenheit => (temp * 1.8) + 32.0,
        }
    }

    fn convert_again() -> bool {
        println!("Would you like to convert another temperature? Y or N");
        loop {
            let mut choice: String = String::new();
            io::stdin().read_line(&mut choice).expect("error");

            match choice.trim().to_uppercase().as_str() {
                "Y" => break true,
                "N" => break false,
                _ => {
                    println!("Please enter Y or N.");
                    continue;
                }
            }
        }
    }
}

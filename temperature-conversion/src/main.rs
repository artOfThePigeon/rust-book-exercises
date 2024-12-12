use std::io;

fn main() {
    println!("Let's convert some temperatures!");

    'temp_scale: loop {
        println!("Do you want to convert to Celsius or Fahrenheit? Please enter the letter F or C for the scale you want to convert to.");
        let mut temp_scale = String::new();

        io::stdin().read_line(&mut temp_scale).expect("error");

        let temp_scale: char = match temp_scale.trim().to_uppercase().as_str() {
            "F" => 'F',
            "C" => 'C',
            _ => continue 'temp_scale,
        };

        'get_temp: loop {
            println!("Please enter a temperature:");
            let mut temperature: String = String::new();
            io::stdin().read_line(&mut temperature).expect("error");
            let temperature: f64 = if let Ok(num) = temperature.trim().parse() {
                num
            } else {
                continue 'get_temp;
            };

            let conversion = match temp_scale {
                'C' => ((temperature - 32.0) * 5.0) / 9.0,
                'F' => (temperature * 1.8) + 32.0,
                _ => panic!("what in the hell bobby!"),
            };

            match temp_scale {
                'C' => println!(
                    "{}° Fahrenheit converts to {:.2}° Celsius.",
                    temperature, conversion
                ),
                'F' => println!(
                    "{}° Celsius converts to {:.2}° Fahrenheit.",
                    temperature, conversion
                ),
                _ => panic!("what in the hell bobby!"),
            };
            break;
        }
        println!("Would you like to convert another temperature? Y or N");

        'keep_playing: loop {
            let mut choice: String = String::new();
            io::stdin().read_line(&mut choice).expect("error");

            match choice.trim().to_uppercase().as_str() {
                "Y" => break 'keep_playing,
                "N" => break 'temp_scale,
                _ => {
                    println!("Please enter Y or N.");
                    continue 'keep_playing;
                }
            }
        }
    }
    println!("Have a breezy day!");
}

use std::io;

fn main() {
    println!(
        "Let's convert some temperatures! For now, we are going to convert Fahrenheit to Celsius."
    );

    'get_temp: loop {
        println!("Please enter a temperature:");
        let mut temperature: String = String::new();
        io::stdin().read_line(&mut temperature).expect("error");
        let temperature: f64 = if let Ok(num) = temperature.trim().parse() {
            num
        } else {
            continue 'get_temp;
        };

        let conversion: f64 = ((temperature - 32.0) * 5.0) / 9.0;
        println!(
            "{}° Fahrenheit converts to {:.2}° Celsius.",
            temperature, conversion
        );

        println!("Would you like to convert another temperature? Y or N");

        'keep_playing: loop {
            let mut choice: String = String::new();
            io::stdin().read_line(&mut choice).expect("error");

            match choice.trim() {
                "Y" => break 'keep_playing,
                "N" => break 'get_temp,
                _ => {
                    println!("Please enter Y or N.");
                    continue 'keep_playing;
                }
            }
        }
    }
    println!("Have a breezy day!");
}
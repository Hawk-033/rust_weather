fn main() {
   conversion_question_tree()
}

fn conversion_question_tree() {
    println!(
        "Which Temperature Conversion would you like to do? \n 1:Celsius to Fahrenheit \n 2:Fahrenheit to Celsius \n 3:Celsius to Kelvin \n 4:Kelvin to Celsius"
    );
    let mut conversion_question: String = String::new();
    std::io::stdin()
        .read_line(&mut conversion_question)
        .unwrap();
    let conversion_question: u8 = conversion_question.trim().parse().unwrap();
    if conversion_question == 1 {
        println!("What is the Temperature in Celsius?");
        let mut celsius_temp = String::new();
        std::io::stdin().read_line(&mut celsius_temp).unwrap();
        let celsius_temp: f64 = celsius_temp.trim().parse().unwrap();
        let celsius_to_fahrenheit_conversion = (celsius_temp * 1.8) + 32.0;
        println!(
            "Temperature in Fahrenheit: {}°F",
            celsius_to_fahrenheit_conversion
        );
    } else if conversion_question == 2 {
        println!("What is the Temperature in Fahrenheit?");
        let mut fahrenheit_temp = String::new();
        std::io::stdin().read_line(&mut fahrenheit_temp).unwrap();
        let fahrenheit_temp: f64 = fahrenheit_temp.trim().parse().unwrap();
        let fahrenheit_to_celsius_conversion = (fahrenheit_temp - 32.0) / 1.8;
        println!(
            "Temperature in Celsius: {}°C",
            fahrenheit_to_celsius_conversion
        );
    } else if conversion_question == 3 {
        println!("What is the Temperature in Celsius?");
        let mut celsius2_temp = String::new();
        std::io::stdin().read_line(&mut celsius2_temp).unwrap();
        let celsius2_temp: f64 = celsius2_temp.trim().parse().unwrap();
        let celsius_to_kelvin_conversion = celsius2_temp + 273.15;
        println!("Temperature in Kelvin: {}K", celsius_to_kelvin_conversion);
    } else if conversion_question == 4 {
        println!("What is the Temperature in Kelvin?");
        let mut kelvin_temp = String::new();
        std::io::stdin().read_line(&mut kelvin_temp).unwrap();
        let kelvin_temp: f64 = kelvin_temp.trim().parse().unwrap();
        let kelvin_to_celsius_conversion = kelvin_temp - 273.15;
        println!("Temperature in Celsius: {}°C", kelvin_to_celsius_conversion);
    }
}
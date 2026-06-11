use num_bigint::BigUint;
use std::io;

fn main() {
    //    let mut count = 0;
    //    'counting_up: loop {
    //        println!("count = {count}");
    //        let mut remaining = 10;
    //
    //        loop {
    //            println!("remaining = {remaining}");
    //            if remaining == 9 {
    //                break;
    //            }
    //            if count == 2 {
    //                break 'counting_up;
    //            }
    //            remaining -= 1;
    //        }
    //
    //        count += 1;
    //    }
    //    println!("End count = {count}");
    //
    //    test_loop();
    //    lift_off();
    //    lift_off_2();
    //    temp_converter();
    fibonacci();
}

//fn test_loop() {
//    let a = [10, 20, 30, 40, 50];
//    let mut index = 0;
//    for element in a {
//        println!("a[{index}] = {element}");
//        index += 1;
//    }
//}
//
//fn lift_off() {
//    let mut number = 3;
//    while number != 0 {
//        println!("T-minus {number}...");
//        number -= 1;
//    }
//    println!("Lift off!");
//}
//
//fn lift_off_2() {
//    for number in (1..4).rev() {
//        println!("T-minus {number}...");
//    }
//    println!("LIFTOFF!");
//}
//
//fn temp_converter() {
//    println!("Welcome to the temperature converter!");
//
//    let unit = loop {
//        println!("Enter temperature in celcius or farenheit (C/F):");
//        let mut input = String::new();
//        io::stdin()
//            .read_line(&mut input)
//            .expect("Failed to read input");
//        match input.trim() {
//            "C" => break "C",
//            "F" => break "F",
//            _ => println!("Invalid input. Please enter C or F."),
//        }
//    };
//
//    if unit == "C" {
//        let temperature = loop {
//            println!("Enter temperature in celcius:");
//            let mut input = String::new();
//            io::stdin()
//                .read_line(&mut input)
//                .expect("Failed to read input");
//            match input.trim().parse::<f64>() {
//                Ok(value) => break value,
//                Err(_) => println!("Invalid input. Please enter a number."),
//            }
//        };
//        let farenheit = (temperature * 9.0 / 5.0) + 32.0;
//        println!("Farenheit: {farenheit:.2}");
//    } else {
//        let temperature = loop {
//            println!("Enter temperature in farenheit:");
//            let mut input = String::new();
//            io::stdin()
//                .read_line(&mut input)
//                .expect("Failed to read input");
//            match input.trim().parse::<f64>() {
//                Ok(value) => break value,
//                Err(_) => println!("Invalid input. Please enter a number."),
//            }
//        };
//        let celcius = (temperature - 32.0) * 5.0 / 9.0;
//        println!("Celcius: {celcius:.2}");
//    }
//}

fn fibonacci() {
    println!("Enter the number of terms to be generated in the Fibonacci sequence:");
    let number_of_terms = loop {
        println!("Enter a positive integer:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse::<usize>() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please enter a positive integer."),
        }
    };
    let mut a = BigUint::from(0u32);
    let mut b = BigUint::from(1u32);
    for _ in 0..number_of_terms {
        println!("{}", a);
        let temp = a.clone();
        a = b.clone();
        b = temp + b;
    }
    println!("Fibonacci sequence generated successfully.")
}

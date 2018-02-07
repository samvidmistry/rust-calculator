use std::io;

fn main() {
    loop {
        match std::process::Command::new("clear").status() {
            Ok(_) => "",
            Err(_) => ""
        };
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");
        println!("Enter your choice: ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error taking input");
        let choice: i32 = choice.trim().parse().expect("Please enter an integer number");
        if choice < 1 || choice > 5 {
            println!("The choice was inappropriate. Please choose something from menu");
            continue;
        }
        if choice == 5 {
            break;
        }
        println!("Enter First operand: ");
        let mut first = String::new();
        io::stdin().read_line(&mut first).expect("Error taking input");
        let first: i32 = first.trim().parse().expect("Please enter an integer number");
        println!("Enter second operand: ");
        let mut second = String::new();
        io::stdin().read_line(&mut second).expect("Error taking input");
        let second: i32 = second.trim().parse().expect("Please enter an integer number");
        let result = match choice {
        	1 => first + second,
        	2 => first - second,
        	3 => first * second,
        	4 => first / second,
        	_ => {
        		println!("Please enter a valid choice");
        		0
        	}
        };
        println!("The result is {}", result);
        io::stdin().read_line(&mut String::new()).expect("");
    }
}

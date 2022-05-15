use std::io;

fn main() {
    println!("Welcome to The Calculator!!\n\n");

    loop {
        println!("Choose the operation: \n");
        println!("1 - Plus");
        println!("2 - Minus");
        println!("3 - Divide");
        println!("4 - Multiply");
        println!("5 - Exit");

        let mut chosen_operation = String::new();

        io::stdin()
            .read_line(&mut chosen_operation)
            .expect("Failed to get value");

        let _chosen_operation: i32 = match chosen_operation.trim().parse() {
            Ok(num) => num,
            Err(_err) => continue
        };

        if _chosen_operation == 5 {
            println!("\nThanks for using the app!");
            break;
        } else {
            get_values_and_calculate(_chosen_operation);
        }
    }

}

fn get_values_and_calculate(chosen_operation: i32) {

    let x = get_value();
    let y = get_value();

    let mut result = 0;

    if chosen_operation == 1 {
        result = x + y;
    } else if chosen_operation == 2 {
        result = x - y;
    } else if chosen_operation == 3 {
        result = x/y;
    } else if chosen_operation == 4 {
        result = x*y;
    }

    println!("\nThe result is: {}\n", result);

}

fn get_value() -> i32 {

    let mut x = String::new();

    println!("\nType a number value: ");

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to get value!");

    let _int_value: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_err) => 0
    };

    _int_value
}

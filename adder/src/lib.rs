#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two doesn't equal to two"))
        }
    } 

    #[test]
    fn add_two_test() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            height: 64,
            width: 64
        };

        let smaller = Rectangle {
            height: 32,
            width: 32,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn check_greetings() {
       let result = greetings("Julio");
       assert!(result.contains("Julio"), "Error found on check_greetings value got was: {}", result);
    }

    #[test]
    #[should_panic(expected = "Num too big:")]
    fn guess_panicking() {
        Guess::new_value(200);
    }

    #[test]
    #[should_panic(expected = "Num too small:")]
    fn guess_small() {
        Guess::new_value(-1);
    }
}

fn greetings(name: &str) -> String {
    format!("Greetings {}!", name)
}

struct Guess {
    value: i32
}

impl Guess {
    fn new_value(value: i32) -> Guess {
        if value < 1  {
            panic!("Num too small: {}", value)
        } else if value > 100 {
            panic!("Num too big: {}", value)
        }
        Guess { value }
    }
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

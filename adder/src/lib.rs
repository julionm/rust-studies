#[cfg(test)]
mod tests {
    use super::*;

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

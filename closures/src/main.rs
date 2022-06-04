use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

// memoization
struct Cacher<T>
where
    T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
where 
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// * created this new struct to resolve the problem of 
// * not caching multiple executions of the value function
struct CacherImproved<T, F, V>
where
    F: Eq + Hash,
    T: Fn(F) -> V
{
    calculation: T,
    value: HashMap<F, V>
}

impl<T, F, V>
CacherImproved<T, F, V>
where 
    F: Eq + Hash + Copy,
    V: Copy,
    T: Fn(F) -> V
{
    fn new(calculation: T) -> CacherImproved<T, F, V> {
        CacherImproved{
            calculation,
            value: HashMap::new()
        }
    }

    fn value(&mut self, args: F) -> V {
        match self.value.get(&args) {
            Some(v) => *v,
            None => {
                let result = (self.calculation)(args);
                self.value.insert(args, result);
                result
            }
        }
    }
}

fn main() {
    let simulated_user_especified_value = 10;
    let simulated_random_number = 7;

    // ? examples for inline closures
   generate_workout(simulated_user_especified_value, simulated_random_number);

   let x = vec![1, 2, 3];
   // ? a closure can access it's environmental variables\

   // there are three ways a closure can use a environemtn variable
   // 1 - taking ownership (FnOnce)
   // 2 - borrow mutably (FnMut)
   // 3 - borrom immutably (Fn)
   // rust infers what it's intended to use
   // but in this case, we're telling rust to move the values
   let equel_to_x = move |z| z == x;

   let y = vec![1, 2, 3];

   assert!(equel_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {
    // ! the type inferation on closures happens the first time it's called
    // ! after that, the types are locked
    let mut expensive_result = CacherImproved::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cacher_improved() {
        let mut c = CacherImproved::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn test_basic_cacher() {
        let mut c = CacherImproved::new(|a| a);

        let v1 = c.value(1);

        assert_eq!(v1, 1);
    }

    #[test]
    fn test_multiple_args_caching() {
        let mut c = CacherImproved::new(|(a, b): (u32, u32)| a + b );

        let v1 = c.value((1, 3));
        let v2 = c.value((3, 4));

        assert_eq!(v2, 7);
    }
}

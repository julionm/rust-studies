#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_sizes(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // * iter takes ownership of the vector
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter(); // iterator do v1

    // ? for loops take ownership and made it mutably behind the scenes
    for val in v1_iter {
        println!("{}", val);
    }

    let v1_iter = v1.iter();

    let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();

    /*
    * The Iterators work with two more method types
    * - adaptors: modificate the iterator in some way, or mapping values,
    *   filtering, reducing
    * - consumers
    */
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn testing_iters() {
        let v1 = vec![1,2,3];

        let mut v1_iter = v1.iter(); // produces an iterator over immutable references

        assert_eq!(v1_iter.next(), Some(&1)); // immutable refs
        assert_eq!(v1_iter.next(), Some(&2)); // immutable refs
        assert_eq!(v1_iter.next(), Some(&3)); // immutable refs
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1,2,3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        // v1_iter is not available anymore because
        // ! sum method took ownership of the iterator

        assert_eq!(total, 6);
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("nike air")
            },
            Shoe {
                size: 12,
                style: String::from("nike bolha")
            },
            Shoe {
                size: 13,
                style: String::from("nike air force")
            },
            Shoe {
                size: 10,
                style: String::from("anti-adidas")
            },
            Shoe {
                size: 12,
                style: String::from("nike air jordan")
            }
        ];

        let in_my_size = shoes_in_sizes(shoes, 10);

        assert_eq!(in_my_size, vec![
            Shoe {
                size: 10,
                style: String::from("nike air")
            },
            Shoe {
                size: 10,
                style: String::from("anti-adidas")
            }
        ])

    }

    #[test]
    fn calling_custom_iterator() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

}

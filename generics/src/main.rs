struct IntPoint<T> {
    x: T,
    y: T
}

struct GenericPoint<T, U> {
    x: T,
    y: U
}

impl<T> IntPoint<T> {

    fn get_x(&self) -> &T {
        &self.x
    }

}

impl<T, U> GenericPoint<T, U> {

    // the self value was moved here, that's the why of not using the borrowing operator
    // i.e.
    // ! MOVED NOT BORROWED
    fn mixup<A, B>(self, other: GenericPoint<A, B>) -> GenericPoint<T, B> {
        GenericPoint {
            x: self.x,
            y: other.y
        }
    }

}

fn main() {
    let new_generic_point = GenericPoint {
        x: 10,
        y: 5.5
    };

    let int_point = IntPoint {
        x: 10,
        y: 10
    };

    let array1 = vec![10, 20, 30, 23, 45, 5];

    println!("{}", largest(&array1));

    let array2 = vec!['a', 'v', 'z', 'c'];

    println!("{}", largest(&array2));

}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {

    let mut largest = list[0];

    for &i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}

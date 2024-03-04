fn main() {
    println!("Hello, world!");
    let x = {
        let x = 5;
        x + 6
    };
    println!("The value of x is: {}", x);

    fn print_number(x: i32) {
        match x {
            1 => println!("One"),
            2 => println!("Two"),
            _ => println!("{}", x),
        }
    }

    print_number(1);
    print_number(2);
    print_number(14);

    struct Number{
        odd: bool,
        value: i32,
    }
    impl Number {
        fn is_positive(&self) -> bool {
            self.value > 0
        }
    }

    let mut z = Number {odd: true, value: 5};
    println!("Is z positive? {}", z.is_positive());

    z.value = -1;
    println!("Is z positive? {}", z.is_positive());

    // generic types
    struct Pair<T> {
        x: T,    // x and y are of type T which is a generic type (can be any type)
        y: T
    }

    let p = Pair {x: 5, y: 10};
    let okay = Pair {x: "hello", y: "world"};
    println!("p.x = {}", p.x);
    println!("okay.x = {} and ", okay.x);

    // vectors type

}

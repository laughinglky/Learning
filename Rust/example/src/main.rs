fn main() {
    let a = '😊';
    println!("a:{}", a);
    println!("test={}", test(2));

    let list1 = [1,2,3,4,5];
    for e in list1.iter() {
        println!("{}", e);
    }
    println!("=======");
    for num1 in 1..4 {
        println!("{}", num1);
    }
    println!("=======");
    for num2 in (1..4).rev() {
        println!("{}", num2);
    }
    println!("=======");
    let mut str1 = "test";
    println!("{}", str1);
    str1 = "test1";
    println!("{}", str1);
    println!("=======");


    let answer = 42;
    println!("{}", answer);
    assert_eq!(answer, 42);
    // assert_eq!(answer, 43);

    let mut sum = 0.0;
    for i in 0..5 {
        if i % 2 == 0 {
            println!("even:{}", i);
        } else {
            println!("odd:{}", i);
        }

        // let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        // println!("{}:{}", even_odd, i);
        
        sum += i as f64;
    }
    println!("sum:{}", sum);
    println!("=======");

    let mut p = Person::new("John","Smith");
    println!("person {} {}", p.first_name,p.last_name);
    println!("full_name: {}", p.full_name());
    println!("p: {:?}", p);
    p.set_first_name("laughing");
    println!("p: {:?}", p);
    println!("p.to_tuple: {:?}", p.to_tuple());
    // println!("p: {:?}", p); // value borrowed here after move
    println!("=======");
    
    let a = A { s: "hello dammit" };
    println!("{:?}", a);
    println!("{:?}", a.s);
    
    println!("=======");
    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);

    println!("=======");
    let n = 42;
    dump(&n);

    println!("=======");
    let start = Direction::Right;
    println!("Direction.Right= {:?}", start);
    println!("Direction.Right.str= {:?}", start.as_str());
    let mut d = start;
    for _ in 0..8 {
        println!("d {:?}", d);
        d = d.next();
    }
    let s = Speed::Slow;
    let speed = s as u32;
    println!("speed {}", speed);

    println!("=======");
    let f = |x| x * x;
    let res = f(10);
    println!("res {}", res);

    println!("=======");
    let f = foo::Foo{s: "hello"};
    println!("{:?}", f);

    println!("=======");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn test(x: i32) -> i32{
    x
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

impl Person {

    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    fn to_tuple(self) -> (String,String) {
        (self.first_name, self.last_name)
    }

}

#[derive(Debug)]
struct A {
    s: &'static str
}

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn dump<T> (value: &T)
where T: std::fmt::Debug {
    println!("value is {:?}",value);
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn as_str(&self) ->  &'static str {
        match *self { // *self has type Direction
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right"
        }
    }

    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
}

// C风格枚举
enum Speed {
    Slow = 10,
    Medium = 20,
    Fast = 50
}

mod foo {
    #[derive(Debug)]
    pub struct Foo {
        pub s: &'static str
    }
}
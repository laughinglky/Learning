fn main() {
    // 1.1 变量和可变性
    // (1)变量默认是不可变的，加 mut 才是可变的
    println!("1.1 变量和可变性");
    let a1 = 5;
    // a1 = 6;  报错
    println!("The value of a1 is: {}", a1);
    let mut a2= 6;
    println!("The value of a2 is: {}", a2);
    a2 = 7;
    println!("The value of a2 is: {}", a2);
    // (2)常量，不可加 mut
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);
    // (3)遮蔽 声明和前面变量具有相同名称的新变量，则第一个变量被第二个变量遮蔽了，前后数据类型可不同
    let a3 = 1;
    println!("The value of a3 is: {}", a3);
    let a3 = 5;
    println!("The value of a3 is: {}", a3);
    {
        let a3 = 3.14;
        println!("The value of a3 is: {}", a3);
    }
    println!("=====");

    // 1.2 数据类型
    // (1)标量类型：Rust有4个基本的标量类型：整型、浮点型、布尔型、字符
    /*
    整型
            长度  有符号类型  无符号类型
            8       i8          u8
            16      i16         u16
            32      i32         u32
            64      i64         u64
            128     i128        u128
            arch    isize       usize
     isize 和 usize 类型取决于程序运行的计算机体系结构，在表中表示为“arch”：
     若使用 64 位架构系统则为 64 位，若使用 32 位架构系统则为 32 位

     整型字面量
     十进制          98_222
     十六进制         0xff
     十八进制         0o77
     二进制          0b1111_0000
     字节(仅限于u8)   b'A'

     注意：
        当使用 --release 参数进行发布（release）模式构建时，Rust 不检测会导致 panic 的整型溢出。
        相反当检测到整型溢出时，Rust 会进行一种被称为二进制补码包裹（two’s complement wrapping）的操作。
        简而言之，大于该类型最大值的数值会被“包裹”成该类型能够支持的对应数字的最小值。
        比如在 u8 的情况下，256 变成 0，257 变成 1，依此类推。程序不会 panic，但是该变量的值可能不是你期望的值。
        依赖整型溢出包裹的行为不是一种正确的做法
        要显式处理溢出的可能性，可以使用标准库针对原始数字类型提供的以下一系列方法：
        1、使用 wrapping_* 方法在所有模式下进行包裹，例如 wrapping_add
        2、如果使用 checked_* 方法时发生溢出，则返回 None 值
        3、使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
        4、使用 saturating_* 方法使值达到最小值或最大值

    字符
        我们声明的 char 字面量采用单引号括起来，这与字符串字面量不同，字符串字面量是用双引号括起来。
        Rust 的字符类型大小为 4 个字节，表示的是一个 Unicode 标量值，这意味着它可以表示的远远不止是 ASCII。
        标音字母，中文/日文/韩文的文字，emoji，还有零宽空格(zero width space)在 Rust 中都是合法的字符类型。
        Unicode 值的范围为 U+0000 ~ U+D7FF 和 U+E000~U+10FFFF。不过“字符”并不是 Unicode 中的一个概念，
        所以人在直觉上对“字符”的理解和 Rust 的字符概念并不一致
     */
    println!("1.2 数据类型");
    let b1 = 'z';
    let b2 = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", b1, b2, heart_eyed_cat);
    // (2)复合类型：元组(tuple)  数组(array)
    /*
        元组是将多种类型的多个值组合到一个复合类型中的一种基本方式。元组的长度是固定的：声明后，它们就无法增长或缩小
            没有任何值的元组 () 是一种特殊的类型，只有一个值，也写成 ()。该类型被称为单元类型（unit type），该值被称为单元值（unit value）。如果表达式不返回任何其他值，就隐式地返回单元值。
        数组的每个元素必须具有相同的类型。与某些其他语言中的数组不同，Rust 中的数组具有固定长度。
            当你希望将数据分配到栈（stack）而不是堆（heap）时，或者当你希望确保始终具有固定数量的元素时，数组特别有用。
            但它们不像 vector （译注：中文字面翻译为“向量”，在 Rust 中意义为“动态数组，可变数组”）类型那么灵活。vector 类型类似于标准库中提供的集合类型，其大小允许增长或缩小。如果不确定是使用数组还是 vector，那就应该使用一个 vector
     */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x,y,z is: {} {} {}", x, y, z);
    println!("The value of tup is: {:?}", tup);
    println!("The value of item is: {} {} {}", tup.0, tup.1, tup.2);
    let tup_empty: () = ();
    println!("The value of tup_empty is: {:?}", tup_empty);
    let array1 = [1, 2, 3, 4, 5];
    let array1:[i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of array1 is: {:?}", array1);
    let array1 = [3; 5]; // [3,3,3,3,3]
    println!("The value of array1 is: {:?}", array1);
    println!("{}", array1[0]);
    println!("=====");

    // 1.3 函数
    /*
        Rust 是一门基于表达式（expression-based）的语言，所以这是一个需要理解的重要区别。
        语句（statement）是执行一些操作但不返回值的指令。
        表达式（expression）计算并产生一个值。
     */
    println!("1.3 函数");
    another_function(66);
    println!("{}", five());
    println!("=====");

    // 2.1 所有权
    /*
        管理程序使用计算机内存的方法：垃圾回收、开发者自己分配和释放内存
        Rust 则选择了第三种方式：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。在运行时，所有权系统的任何功能都不会减慢程序

        栈中的所有数据都必须占用已知且固定的大小。在编译时大小未知或大小可能变化的数据，要改为存储在堆上。
        堆是缺乏组织的：当向堆放入数据时，你要请求一定大小的空间。内存分配器（memory allocator）在堆的某处找到一块足够大的空位，
        把它标记为已使用，并返回一个表示该位置地址的 指针（pointer）。这个过程称作 在堆上分配内存（allocating on the heap），有时简称为 “分配”（allocating）。
        将数据推入栈中并不被认为是分配。因为指针的大小是已知并且固定的，你可以将指针存储在栈上，不过当需要实际数据时，必须访问指针

        所有权规则：
            1、Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
            2、值在任一时刻有且只有一个所有者。
            3、当所有者（变量）离开作用域，这个值将被丢弃。
     */
    println!("2.1 所有权");
    // (1)内存在拥有它的变量离开作用域后就被自动释放
    {
        let s = String::from("hello"); // 从此处起，s 开始有效
        // 使用 s
    }                                  // 此作用域已结束，s 不再有效
    /*
    这是一个将 String 需要的内存返回给分配器的很自然的位置：当 s 离开作用域的时候。
    当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。
    Rust 在结尾的 } 处自动调用 drop
     */

    // (2)变量与数据交互的方式（一）：移动
    // 将 5 绑定到 x；接着生成一个值 x 的拷贝并绑定到 y”。现在有了两个变量，x 和 y，都等于 5。这也正是事实上发生了的，因为整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中
    let x = 5;
    let y = x;
    println!("{} {}", x, y);
    // String 由三部分组成，一个指向存放字符串内容内存的指针，一个长度，和一个容量。这一组数据存储在栈上。右侧则是堆上存放内容的内存部分。
    // 长度表示 String 的内容当前使用了多少字节的内存。容量是 String 从分配器总共获取了多少字节的内存。长度与容量的区别是很重要的，不过在当前上下文中并不重要，所以现在可以忽略容量。
    // 当我们将 s1 赋值给 s2，String 的数据被复制了，这意味着我们从栈上拷贝了它的指针、长度和容量。我们并没有复制指针指向的堆上数据
    // 两个数据指针指向了同一位置。这就有了一个问题：当 s2 和 s1 离开作用域，他们都会尝试释放相同的内存。这是一个叫做 二次释放（double free）的错误，也是之前提到过的内存安全性 bug 之一。两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。
    // 为了确保内存安全，这种场景下 Rust 的处理有另一个细节值得注意。在 let s2 = s1 之后，Rust 认为 s1 不再有效，因此 Rust 不需要在 s1 离开作用域后清理任何东西
    let s1 = String::from("hello");
    let s2 = s1;  // s1被移动到s2，s1无效了，不能使用了
    // println!("{}, world!", s1);

    // (3)变量与数据交互的方式（二）：克隆
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // (4)只在栈上的数据：拷贝
    // 像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。这意味着没有理由在创建变量 y 后使 x 无效
    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    // (5)所有权与函数
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效
    let x = 5;                 // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x
    // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
    // 所以不会有特殊操作

    // (6)返回值与作用域

    // 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
    // 当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有
    println!("=====");

    // 2.2 引用和借用
    println!("2.2 引用和借用");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    let mut s = String::from("hello");
    change(&mut s);  // 可变引用
    // 可变引用有一个很大的限制：在同一时间，只能有一个对某一特定数据的可变引用。尝试创建两个可变引用的代码将会失败
    println!("s=={}", s);
    println!("=====");

    // 2.3 切片slice
    println!("2.3 切片slice");
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // error
    println!("the first word is: {}", word);

    let my_string = String::from("hello world");
    // `first_word` 接受 `String` 的切片，无论是部分还是全部
    let word = first_word1(&my_string[0..6]);
    let word = first_word1(&my_string[..]);
    // `first_word` 也接受 `String` 的引用，
    // 这等同于 `String` 的全部切片
    let word = first_word1(&my_string);
    let my_string_literal = "hello world";
    // `first_word` 接受字符串字面量的切片，无论是部分还是全部
    let word = first_word1(&my_string_literal[0..6]);
    let word = first_word1(&my_string_literal[..]);
    // 因为字符串字面值**就是**字符串 slice，这样写也可以，即不使用 slice 语法！
    let word = first_word1(my_string_literal);
    println!("=====");

    // 3. 结构体
    println!("3. 结构体");
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1={:?}", user1);
    // 结构体更新
    let user2 = User{
        email: String::from("test@163.com"),
        ..user1
    };
    println!("user2={:#?}", user2);  // 格式化输出
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // 类单元结构体
    let subject = AlwaysEqual;
    // 结构体输出
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    // 方法：与函数类似，不同的是在结构体的上下文中被定义
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    // 关联函数
    let sq = Rectangle::square(3);
    println!("=====");
    // 4. 枚举和模式匹配
    println!("4. 枚举和模式匹配");
    // 4.1 枚举
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("enum:{:?}  {:?}", four, six);
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    // 枚举绑定方法
    let m = Message::Write(String::from("hello"));
    m.call();
    // Option枚举 定义与标准库中
    /*
    enum Option<T> {
        Some(T),
        None,
    }
     */
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some_number:{:?}  some_string:{:?} absent_number:{:?}", some_number, some_string, absent_number);

    // 4.2 match控制流
    let res_match1:u8 = value_in_cents(Coin::Penny);
    println!("value_in_cents(Penny)={:?}", res_match1);
    let res_match2:u8 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("value_in_cents(Penny)={:?}", res_match2);
    // 通配符 _ 会匹配所有的值
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    // 4.3 if let控制流
    let some_u8_value = Some(3u8);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }
    if let Some(3) = some_u8_value {
        println!("three");
    }

    println!("=====");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

#[derive(Debug)] // 这样可以可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        println!("Message.call()");
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn first_word1(s: &str) -> &str {
    let bytes = s.as_bytes();  // as_bytes 方法将 String 转化为字节数组
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();  // as_bytes 方法将 String 转化为字节数组
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    // &:允许使用值但是不获取所有权
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

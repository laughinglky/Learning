fn main() {
    
    // 可变绑定与不可变绑定
    let a = 11;
    println!("a={}", a);
    let mut b = 12;
    println!("b={}", b);
    b = 18;
    println!("b={}", b);
    println!("==========");
    
    // 所有权转移 move语义
    let place1 = "hello";
    let place2 = "hello1111".to_string();
    let other = place1;  // 所有权转移
    println!("other={}", other);
    println!("place1={}", place1);
    let other = place2;
    // println!("place2={}", place2);   // err:value borrowed here after move;move occurs because `place2` has type `String`, which does not implement the `Copy` trait
    println!("other={}", other);
    println!("==========");
    
    // 借用 copy语义
    let a = [1, 2, 3, 4, 5]; 
    let b = &a;
    println!("{:p}", b);
    let mut c = vec![1,2, 3, 4, 5];
    let d = &mut c;  // 1.必须是可变引用才能修改值 2.c1必须是可变绑定 -> 要获取可变引用，必须先声明可变绑定
    d.push(6);
    println!("{:?}", d);
    let e = &42;  // 字面量42是值表达式，加上借用操作符& 相当于值表达式在位置上下文进行求值，编译器会为&42创建一个临时值
    assert_eq!(42, *e);
    println!("==========");

    // 函数指针  函数是一等公民，其自身就可以作为函数的参数和返回值使用
    let a = 2;
    let b = 3;
    assert_eq!(math(sum, a, b), 5);
    assert_eq!(math(product, a, b), 6);
    assert_eq!(true_maker()(), true);
    println!("==========");

    // 闭包
    let out = 42;
    let closure_annotated = |i:i32, j:i32| -> i32 {i + j + out};
    let closure_inferred = |i, j| i + j + out;
    let i = 1;
    let j = 2;
    assert_eq!(45, closure_annotated(i, j));
    assert_eq!(45, closure_inferred(i, j));
    let a = 2;
    let b = 3;
    assert_eq!(math1(|| a + b), 5);
    assert_eq!(math1(|| a * b), 6);
    let result = two_times_impl();
    assert_eq!(result(2), 4);
    println!("==========");


}

// 函数作为参数
pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn product(a: i32, b: i32) -> i32 {
    a * b
}

// 函数作为返回值
fn is_true() -> bool {
    true
}
fn true_maker() -> fn() -> bool {
    is_true  // 使用了函数名字作为函数指针
}

// 闭包作为参数?
fn math1<F: Fn() -> i32>(op: F) -> i32 {
    op()
}
// 闭包作为返回值?
fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    move |j| j * i
}

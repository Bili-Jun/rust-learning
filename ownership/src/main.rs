fn main() {
    // println!("Hello, world!");

    let s1 = String::from("hello");
    // let s2 = s1; // 会报错,因为所有权,s1被移动至s2,s1的内存已被释放
    // println!("{}, world!", s1);

    // 可以使用clone来处理如上问题
    let s2 = s1.clone();

    println!("{}, world!, s2 = {}", s1, s2);

    // 整型等已知大小的类型直接存储在栈内,拷贝时无需移动并释放
    let x = 5;
    let y = x;
    println!("{}", x);

    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

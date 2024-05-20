// use num::complex::Complex;
use std::fmt::Debug;
use std::mem::size_of_val;
fn main() {
    // let a = Complex { re: 2.1, im: -1.2 };
    // let b = Complex::new(0.1,0.2);
    // let r = a + b;
    // println!("{} + {}i", r.re, r.im);

    let v: u16 = 38_u8 as u16;
    println!("{}", v);

    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    let v1 = 247_u8 + 8;
    let v2 = i8::wrapping_add(119, 8);
    println!("{},{}", v1, v2);

    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");

    let c1 = "中";
    print_char(c1);

    println!("unit() size {}", size_of_val(&()));

    let x = 3;
    let v = x;

    assert!(v == 3);

    let v = {
        let mut x = 1;
        x += 2;
        x
    };
    println!("{}", v);

    assert_eq!(v, 3);

    // let s = sum(1 , 2);
    // assert_eq!(s, 3);

    another_function(5, 6.1);

    println!("{:?}", report('A'));
    let mut s = String::from("ABC");
    println!("{:?}", clear(&mut s));

    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    print();

    // never_return();

    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let a = 1;
    // 值copy;
    let b = a;
    println!("{},{}", a, b);

    let s1 = String::from("hello");
    // 移交所有权
    let s2 = s1;
    //    println!("{}",s1);

    let x: &str = "hello,world";
    let y = x;
    println!("{},{}", x, y);

    let x = String::from("hello, world");
    // let y = x.clone();
    let y = x.as_str();
    println!("{},{}", x, y);

    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);

    let s = give_ownership();
    println!("{}", s);

    let s = String::from("hello, world");

    print_str(s.clone());

    println!("{}", s);

    let x = (1, 2, (), "hello".to_string());
    let y = &x;
    println!("{:?}, {:?}", x, y);

    let s = String::from("hello, ");

    // 只修改下面这行代码 !
    let mut s1 = s;

    s1.push_str("world");

    let x = Box::new(5);

    let mut y = Box::new(4);

    *y = 4;

    assert_eq!(*x, 5);

    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // 仅修改下面这行代码，且不要使用 `_s`
    println!("{:?}", t.1);

    let t = (String::from("hello"), String::from("world"));

    // 填空，不要修改其它代码
    let (ref s1, ref s2) = t;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")


    let x = 5;
    let y = &x;
    
    assert_eq!(5,*y);

    let x = 5;
    let p = &x;
 
    println!("x 的内存地址是 {:p}", p);

    let x = 5;
    let y = &x;

    assert_eq!(5, *y);
    let mut s = String::from("hello, ");

    // borrow_object(&s);

    let mut s = String::from("hello, ");

    push_str(&mut s);

    let mut s = String::from("hello, ");

    let p = &mut s;
    
    p.push_str("world");

    let c = '中';

    let r1 = &c;
    let ref r2 = c;

    assert_eq!(*r1, *r2);
    
    assert_eq!(get_addr(r1),get_addr(r2));

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);


    let mut  s = String::from("hello, ");

    borrow_object(&mut s);

    let mut s = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");

    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
    // println!("{}",r1);


}

fn borrow_object(s: &String) {}

// fn borrow_object(s: &mut String) {}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

fn push_str(s: &mut String) {
    s.push_str("world")
}


// fn borrow_object(s: &String) {}

fn print_str(s: String) {
    println!("{}", s)
}

fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    // let _s = s.clone().into_bytes();
    // let _s = s.as_bytes();
    s
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

// fn never_return() -> ! {
//    panic!("!")
// }

fn print() -> () {
    println!("hello,world");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

fn clear(text: &mut String) -> () {
    *text = String::from("");
}

fn another_function(x: i32, y: f64) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
fn print_char(c: &str) {
    println!("{}", c);
}

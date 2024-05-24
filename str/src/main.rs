use utf8_slice;

fn main() {
    let my_name = "Pascal";
    greet(my_name.to_string());

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    let hello = &s[..2];
    let world = &s[6..];
    let st = &s[..];
    dbg!(hello);
    dbg!(world);
    dbg!(st);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    dbg!(slice);

    let s = String::from("hello, world");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    let s1 = String::from("hello");
    let h = &s1[0..2];
    dbg!(h);

    let mut s = String::from("Hello");
    s.push_str("rust");
    println!("追加{}", s);
    s.push('!');
    println!("push{}", s);

    s.insert(5, ',');
    println!("inerst{}", s);

    s.insert_str(6, "I LIKE");
    println!("inerst_str{}", s);

    let e = s.replace("I", " xixi_xixi ");
    println!("replace{}", e);
    let mut t = s.replacen("I", " xixi_xixi ", 0);
    println!("replace{}", t);

    t.replace_range(7..8, "R");
    dbg!(t);
    let mut u = String::from("skdkdksl");
    dbg!(u.pop());
    dbg!(u.pop());

    println!("{}", std::mem::size_of_val(u.as_str()));

    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
                            can span multiple lines.
                            The linebreak and indentation here ->\
                            <- can be escaped too!";
    println!("{}", long_string);

    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    let s: &str = "hello, world";

    // let s: Box<&str> = "hello, world".into();
    // greetings(&s);
    // greetings(*s);
    //  greetings(s.to_string().as_str());

    let mut s = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    // println!("{}", s)

    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);

    let s = String::from("hello, world");
    greetings(s);

    let s = "hello, world";
    let s1: &str = s;

    let long_delimiter = r#######"Hello, "#""######""#######;
    //  assert_eq!(long_delimiter, "Hello, \"##\"");

    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
    assert_eq!(h1, "中");

    for c in "你好，世界".chars() {
        println!("{}", c)
    }

    let s = "The 🚀 huogoes to the !";
    let roocket = utf8_slice::slice(s,4,5);

    dbg!(roocket);
    dbg!(s);

    let tup:(i32,f64,u8) = (500,6.4,1);
    dbg!(tup);

    let (x,y,z) = tup;
    println!("the value of y is: {}",y);

    let five_hundred = tup.0;
    let one = tup.2;
    dbg!(five_hundred,one);

    let _t0: (u8,i16) = (0, -1);
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);

    let tup = (1, 6.4, "hello");

    let (x,z,y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
    let (x, y, z);

    // 填空
    (y,z,x) = (1, 2, 3);
    
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    let (x, y) = sum_multiply((2,3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

fn greetings(s: String) {
    println!("{}", s)
}

// fn greetings(s: &str) {
//     println!("{}", s)
// }

fn say_hello(s: &str) {
    println!("{}", s);
}

fn greet(name: String) {
    println!("Hello,{}!", name);
}

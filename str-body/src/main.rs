fn main() {
    // let mut user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let user2 = build_user(
    //     String::from("wangergou"),
    //     String::from("wangergou@example.com"),
    // );

    // dbg!(user2);

    // let user3 = User {
    //     email: String::from("wangergou2@example.com"),
    //     ..user1
    // };

    // dbg!(user3);

    // let f1 = File {
    //     name: String::from("f1.txt"),
    //     data: Vec::new(),
    // };

    // let f1_name = &f1.name;
    // let f1_length = &f1.data.len();

    // println!("{:?}", f1);
    // dbg!(f1_name, f1_length);

    // let bool = Color(0,0,0);
    // let origin = Point(0,0,0);

    // let subject = AlwaysEqual;

    // let user1 = User {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let scale = 2;
    // let rect1 = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };

    // dbg!(&rect1);
    // println!("{:?}", rect1);


    let u = Unit;
    do_something_with_unit(u);


    let v = Point(0, 127, 255);
    check_color(v);

    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    p.age = 30;

    p.name = String::from("sunfei");
    println!("{:?}",p);

    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;

    println!("{}, {}",_name, f.data);
}


fn do_something_with_unit(u: Unit) { }

struct Unit;
trait SomeTrait { }

// 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
impl SomeTrait for Unit { }

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn check_color(p: Point) {
    let Point(x, z, y) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
    
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// #[derive(Debug)]
// struct File {
//     name: String,
//     data: Vec<u8>,
// }

// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// struct AlwaysEqual;

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

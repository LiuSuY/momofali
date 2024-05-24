
use crate::List::*;

// #[derive(Debug)]
// enum PokerSuit {
//     Clubs,
//     Spades,
//     Diamonds,
//     Hearts,
// }

// // #[derive(Debug)]
// // struct PokerCard {
// //     suit: PokerSuit,
// //     value: u8
// // }

// enum PokerCard {
//     Clubs(u8),
//     Spades(u8),
//     Diamonds(char),
//     Hearts(char),
// }

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    // let heart = PokerSuit::Hearts;
    // let diamond = PokerSuit::Diamonds;

    // print_suit(heart);
    // print_suit(diamond);

    // let c1 = PokerCard {
    //     suit: PokerSuit::Clubs,
    //     value: 1,
    // };
    // let c2 = PokerCard {
    //     suit: PokerSuit::Diamonds,
    //     value: 12,
    // };

    // dbg!(c1);
    // dbg!(c2);

    // let c1 = PokerCard::Spades(5);
    // let c2 = PokerCard::Diamonds('A');

    // let _some_number = Some(5);
    // let _some_string = Some("a string");
    // let _absent_number: Option<i32> = None;

    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);

    let msg1 = Message::Move{x:1,y:2}; // 使用x = 1, y = 2 来初始化
    let msg2 = Message::Write("hello, world!".to_string()); // 使用 "hello, world!" 来初始化
    let msg = Message::Move{x: 1, y: 2};

    if let Message::Move{x:a,y:b} = msg {
        assert_eq!(b,2);
    } else {
        panic!("不要让这行代码运行！");
    }

    let msgs: [Message;3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    // if let Some(n) = six {
    //     println!("{}", n);
    //     return;
    // } 
        
    // panic!("不要让这行代码运行！");

    // 创建一个新的链表(也是空的)
    let mut list = List::new();

    // 添加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 打印列表的当前状态
    println!("链表的长度是: {}", list.len());
    println!("{}", list.stringify());
 
}


enum List {
    // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
    Cons(u32, Box<List>),
    // Nil: 链表中的最后一个节点，用于说明链表的结束
    Nil,
}

// 为枚举实现一些方法
impl List {
    // 创建空的链表
    fn new() -> List {
        // 因为没有节点，所以直接返回 Nil 节点
        // 枚举成员 Nil 的类型是 List
        Nil
    }

    // 在老的链表前面新增一个节点，并返回新的链表
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回链表的长度
    fn len(&self) -> u32 {
        match *self {
            // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 空链表的长度为 0
            Nil => 0
        }
    }

    // 返回链表的字符串表现形式，用于打印输出
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // 递归生成字符串
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn show_message(msg: Message) {
    println!("{:?}", msg);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum Number {
    Zero,
    One,
    Two,
}

#[derive(Debug)]
enum Number1 {
    Zero = 0,
    One,
    Two,
}

#[derive(Debug)]
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// fn print_suit(card: PokerSuit) {
//     // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
//     println!("{:?}", card);
// }

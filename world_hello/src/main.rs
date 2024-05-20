fn main() {
//    greet_world();

//    greet_line();
//     let a = 10;
//     let b: i32 = 20;
//     let c = 30i32;
//     let d = 30_i32;
//     let e = add(add(a,b),add(c,d));

//     println!("( a + b ) + ( c + d ) = {}" ,e);

//     let _t = 5;

//     let (a, mut b): (bool,bool) = (true,false);

//     println!("a = {:?}, b = {:?}",a,b);

//     b = true;
//     println!("a = {:?}, b = {:?}",a,b);
//     assert_eq!(a,b);



//     let (a,b,c,d,e);

//     (a,b) = (1,2);
//     [c,..,d,_] = [1,2,3,4,5];
//     Struct { e, .. } = Struct { e: 5 };

//     assert_eq!([1,2,1,4,5],[a,b,c,d,e]);

//     const MAX_POINTS: u32 = 1000_000;


    // let x: i32 = 0; // 未初始化，但被使用
    // let _y: i32; // 未初始化，也未被使用
    // println!("x is equal to {}", x); 


    // let mut x = 1;
    // x += 2;
    // println!("x = {}", x); 

   
    println!("{}, world", define_x()); 

    // let x:i32 = 5;
    // {
    //     let x = 12;
    //     // assert_eq!(x, 5);
    //     assert_eq!(x, 12);
    // }

    // // assert_eq!(x,12);
    // assert_eq!(x,5);
    // let x = 42;
    // println!("{}", x);


    // let mut x: i32 = 1;
    // x = 7;
    // // 遮蔽且再次绑定
    // let mut x = x; 
    // x += 3;


    // let y = 4;
    // // 遮蔽
    // let y = "I can also be bound to text!"; 

    // let _x = 1; 
    // let x = 1; 
    // println!("{}",x);

    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x,y], [3,2]);
    ();
    //类型推导
    let guesss = "42".parse::<u32>().expect("Not a number");

    //IEEE 754 问题依然存在
    // 隐
    println!("0.1 + .0.2 = {}",0.1+0.2);
    // 显
    println!("0.1 + .0.2 = {}",0.1f64+0.2f64);
    println!("0.1 + .0.2 = {}",0.1f32+0.2f32);

    let a : u8 = 255;
    let b = a.wrapping_add(1);
    // 循环溢出处理
    println!("{}", b);  // 19


    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    let a = 20;
    let a = a >> 1;
    println!("{}",a);

    for i in 1..=5{
        println!("{}",i);
    }

    for i in 'a'..='z' {
        println!("{}",i);
    }
}

fn define_x() -> String{
    let x = "hello".to_string();
    x
}

// struct Struct {
//     e: i32
// }

// fn add(i:i32, j:i32) -> i32 {
//     i + j
// }

// fn greet_line() {
//     let penguin_data = "\
//     common name,length (cm) Little penguin,33
//     Yellow-eyed penguin,65
//     Fiordland penguin,60
//     Invalid,data
//     ";

//     let records = penguin_data.lines();


//     for (i, record) in records.enumerate(){
//         if i == 0 || record.trim().len() == 0 {
//             continue;
//         }


//         let fields: Vec<&str> = record
//         .split(',')
//         .map(|field|field.trim())
//         .collect();
//         if cfg!(debug_assertions) {
//             eprintln!("debug: {:?} -> {:?}", record, fields)
//         }

//         let name = fields[0];

//         if let Ok(length) = fields[1].parse::<f32>(){
//             println!("{}, {}cm",name,length);
//         }
//     }
// }


// fn greet_world(){

//     let southern_germany = "Grub Gott";
//     let chinese = "世界，你好";
//     let english = "World, hello";
//     let regions = [southern_germany, chinese, english];
//     for region in regions.iter() {
//         println!("{}",&region)
//     }
// }
fn main() {
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));

    println!("{:#?}", array);

    // 使用合适的类型填空
    let arr: [_; 5] = [1, 2, 3, 4, 5];

    // 修改以下代码，让它顺利运行
    assert!(arr.len() == 5);

    // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
    let _arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];

    // 填空
    // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
    // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
    assert!(std::mem::size_of_val(&arr) == 12);

    // 填空
    // let list: [i32; 100] = std::array::from_fn(|_i| 1_i32);
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    // 修复错误
    let _arr = [1, 2, 3];

    let arr = ['a', 'b', 'c'];

    let ele = arr[0]; // 只修改此行来让代码工作

    assert!(ele == 'a');

    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
    let _name0 = names.get(0).unwrap();

    // 但是下标索引就存在越界的风险了
    let _name1 = &names[1];

    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let n = 5;

    let big_n = 
    if n < 10 && n > -10 {
        println!(" 数字太小，先增加 10 倍再说");

        10 * n
    } else {
        println!("数字太大，我们得让它减半");

        n / 2
    };

    println!("{} -> {}", n, big_n);

    for n in 1..100 { // 修改此行，让代码工作
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    let names = [String::from("liming"),String::from("hanmeimei")];
    for _name in &names {
        // do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // numbers中的元素实现了 Copy，因此无需转移所有权
    for _n in numbers {
        // do something with name...
    }
    
    println!("{:?}", numbers);

    let a = [4,3,2,1];

    // 通过索引和值的方式迭代数组 `a` 
    for (i,v) in a.iter().enumerate() {
        println!("第{}个元素是{}",i+1,v);
    }

    let mut n = 1;

    // 当条件为真时，不停的循环
    while n <= 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }


        n += 1;
    }

    println!("n 的值是 {}, 循环结束",n);

    let mut n = 0;
    for _i in 0..=100 {
       if n == 66 {
           break;
       }
       n += 1;
    }

    assert_eq!(n, 66);


    let mut n = 0;
    for _i in 0..=100 {
       if n != 66 {
           n+=1;
           continue;
       }
       
       break;
    }

    assert_eq!(n, 66);
    

    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过当此循环的剩余代码
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }

    assert_eq!(count, 5);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break 20;
        }
    };

    assert_eq!(result, 20);


    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            println!("执行inner1");
            if count >= 20 {
                // 这只会跳出 inner1 循环
                dbg!(count,"inner");
                break 'inner1; // 这里使用 `break` 也是一样的
            }
            count += 2;
        }
        println!("执行inner2");
        count += 5;
        dbg!(count);

        'inner2: loop {
            println!("执行inner3");
            if count >= 30 {
                dbg!(count,"inner");
                break 'outer;
            }

            continue 'outer;
        }
    }

    assert!(count == 30);
}

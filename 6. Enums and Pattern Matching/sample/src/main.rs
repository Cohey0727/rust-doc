fn main() {
    main_6_1();
}

// 6.1. Defining an Enum

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main_6_1() {
    println!("6.1. Defining an Enum");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four: {:?}", four);
    println!("six: {:?}", six);

    let x: i8 = 5;
    let y: Option<i8> = Some(10);
    // let sum = x + y;
    let sum = match y {
        Some(val) => x + val,
        None => x,
    };
    println!("sum: {}", sum)
}

fn main() {
    println!("4 Understanding Ownership");
    main_ownership();
    main_references_borrowing();
    main_the_slice_type();
}

// 4.1. What is Ownership?
fn main_ownership() {
    println!("4.1. What is Ownership?");
    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(s); // sの値が関数にムーブされ...
                        // ... ここではもう有効ではない

    // println!("{}", s); // エラー

    let x = 5; // xがスコープに入る

    makes_copy(x); // xも関数にムーブされるが、
                   // i32はCopyなので、この後にxを使っても
                   // 大丈夫

    println!("{}", x); // エラー出ない
} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

// 4.2. References and Borrowing
fn main_references_borrowing() {
    println!("4.2. References and Borrowing");
    let mut s = String::from("hello");
    change(&mut s);
    s.push_str(" OMG");

    let r1 = &mut s;
    // let r2 = &mut s; // 2ヶ所で同時に可変な参照を渡すことはできない。
    r1.push_str(" OMG");

    let r3 = r1; // ここでr1は参照を失う。
    r3.push_str(" OMG");

    println!("{}", s); // 参照渡してであれば、sは解放されてないし、変更も反映される。
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// The Slice Type
fn main_the_slice_type() {
    println!("4.3. The Slice Type");
    let mut s = String::from("Hello World");
    first_word(&s);
    s.clear(); // sがmutであっても
    println!("{}", s);
    // println!("{}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (_i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &"aaa";
        }
    }

    &"eeee"
}

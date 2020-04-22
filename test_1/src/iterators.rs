/// Tests des iterators
pub fn test() {
    for item_ref in (&[11u8, 22, 33]).iter() {
        print!("{} ", *item_ref);
    }
    println!();
    for item_ref in [44, 55, 66].iter() {
        print!("{} ", *item_ref);
    }
    println!();
    for item_ref in vec!['a', 'b', 'c'].iter_mut() {
        *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
        print!("{} ", *item_ref);
    }
    println!();

    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.iter().filter(|x| **x < 0) {
        print!("{} ", n);
    }
    println!();

    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.iter().map(|x| *x * 2) {
        print!("{} ", n);
    }
    println!();

    let arr = ['a', 'b', 'c'];
    for (i, ch) in arr.iter().enumerate() {
        print!("{} {}, ", i, *ch);
    }
    println!();

    let s = "Hello, world!";
    let ch = 'R';
    println!("\"{}\" {} '{}'.",
             s,
             if s.chars().any(|c| c == ch) {
                 "contains"
             } else {
                 "does not contain"
             },
             ch);

    print!("{} ", [45, 8, 2, 6].iter()
        .all(|n: &i32| -> bool { *n > 0 }));
    println!();
    print!("{} ", [45, 8, -2, 6].iter()
        .all(|n: &i32| *n > 0));
    println!();

    let a = "String".to_owned();
    let sum: i32 = a.bytes()
        .map(|x: u8| x as i32)
        .sum();
    println!("sum={}", sum);

    let arr = [45, 8, -2, 6];
    match arr.iter().min() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
    match arr.iter().max() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
    println!();

    let arr = ["hello", "brave", "new", "world"];
    match arr.iter().min() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
    match arr.iter().max() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
    println!();

    let arr = [36, 1, 15, 9, 4];
    let v = arr.iter().collect::<Vec<&i32>>();
    println!("{:?}", v);

    let arr = [36, 1, 15, 9, 4];
    let v: Vec<_> = arr.iter().collect();
    println!("{:?}", v);

    let s = "Hello";
    println!("{:?}", s.chars().collect::<String>());
    println!("{:?}", s.chars().collect::<Vec<char>>());
    println!("{:?}", s.bytes().collect::<Vec<u8>>());
    println!("{:?}", s.as_bytes().iter().collect::<Vec<&u8>>());

    let arr = [66, -8, 43, 19, 0, -31];
    let v = arr
        .iter()
        .filter(|x| **x > 0)
        .map(|x| *x * 2)
        .collect::<Vec<_>>();
    println!("{:?}", v);

    let v = [66, -8, 43, 19, 0, -31]
        .iter()
        .filter(|x| { print!("F{} ", x); **x > 0 })
        .map(|x| { print!("M{} ", x); *x * 2 })
        .collect::<Vec<_>>();
    println!("\n{:?}", v);
}

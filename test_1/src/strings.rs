/// Tests strings
pub fn test() {
    let mut s = "Test".to_owned();
    println!("{}", s);

    test_string(&mut s);
    println!("{}", s);

    test_string(&mut s);
    println!("{}", s);

    // let mut r = "Toto".to_owned();
    let mut r = 42;
    let t = &mut r;
    println!("t: {}", t);
    *t += 1;
    let u = &mut r;
    *u += 5;
    println!("u: {}", u);
    println!("r: {}", r);
}

fn test_string(s: &mut String) {
    s.push_str("_1");
}

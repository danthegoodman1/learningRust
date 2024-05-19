use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    let vinf = vec![1, 2];

    v.push(vinf[0]);
    v.push(vinf[1]);
    let v = dbg!(v);

    let first: &i32 = &v[1];
    dbg!(first);

    let hundred: Option<&i32> = vinf.get(100);
    match hundred {
        Some(hundred) => println!("The hundred element is {hundred}"),
        None => println!("There is no hundred element."),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}")
    }

    let mut v = vec![1, 2, 3, 4];
    for i in &mut v {
        *i += 10
    }
    dbg!(v);

    // For vectors that have "different" types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    dbg!(row);

    // Strings

    let data = "some data";
    let s = data.to_string();
    dbg!(s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    dbg!(s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // uses references so it doesn't take any ownership
    let s = dbg!(s);
    for c in s.chars() {
        println!("{c}");
    }

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    dbg!(s); // will be 'Зд' because that's 4 bytes
    for c in hello.chars() {
        println!("{c}");
    }
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }



    // Hash maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 100);
    scores.insert(String::from("Yellow"), 10);

    let team_name = String::from("Blue");
    let score = match scores.get(&team_name).copied() {
        None => {
            println!("did not exist! returning");
            return
        },
        Some(score) => score
    };
    dbg!(score);

    scores.entry(String::from("Blue")).or_insert(50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

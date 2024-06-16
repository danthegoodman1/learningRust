fn main() {
    let vec = vec!["a", "b", "c", "d", "e", "f", "g"];
    let mut iter = vec.iter().peekable();

    let mut thing = true;
    while let Some(i) = iter.next() {
        println!("Got i {} thing {}", i, thing);
        println!("Peeked {:?}", iter.peek());
        if let Some(&&"d") = iter.peek() {
            println!("Got d in peek, skipping");
            iter.next();
            thing = false;
        } else {
            thing = true;
        }
    }
}

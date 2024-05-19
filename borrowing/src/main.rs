fn main() {
    let mut v = vec![1, 2, 3];
    let last: i32;
    {
        let item = v.last().unwrap();
        last = item.clone();
    }
    v.push(4);
    println!("{:?}", last)
}

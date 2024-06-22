use rayon::prelude::*;

fn main() {
    let strings = vec!["a", "b", "c", "d", "e"];
    let res = strings.par_iter().fold(|| String::new(), |mut cum: String, new| {
        println!("fold item {}-{}", cum, new);
        cum.push_str(&new);
        return cum
    }).reduce(|| String::new(), |mut cum, new| {
        cum.push_str(&new);
        return cum
    });
    println!("{}", res);
}

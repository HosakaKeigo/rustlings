fn main() {
    let mut res = 42;
    let option = Some(12);

    // if let を使って書き換え
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}

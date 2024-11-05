fn main() {
    const SIZE: usize = 5;
    let width = SIZE * 2 - 1;

    for i in 0..(SIZE * 2 - 1) {
        let stars = if i < SIZE { 2 * i + 1 } else { 2 * (SIZE * 2 - i - 2) + 1 };
        let spaces = (width - stars) / 2;


        print!("{:width$}", "", width = spaces);
        println!("{}", "*".repeat(stars));
    }
}

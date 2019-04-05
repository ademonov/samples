use std::io;

fn main() -> io::Result<()> {
    let t = read_number();
    for _i in 0..t {
        let n = read_number();
        println!("{}", get_sum(n - 1));
    }

    Ok(())
}

fn read_number() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error!");
    input.trim().parse::<i64>().expect("parse error!")
}

fn get_sum(limit: i64) -> i64 {
    get_sum_n(limit, 3) + get_sum_n(limit, 5) - get_sum_n(limit, 3 * 5)
}

fn get_sum_n(limit: i64, n: i64) -> i64 {
    let count = limit / n;  
    let last = count * n;
    (n + last) * count / 2
}

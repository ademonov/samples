use std::io;

fn main() -> io::Result<()> {
    let t = read_number();
    for _i in 0..t {
        let n = read_number();
        println!("{}", get_result(n));
    }

    Ok(())
}

fn read_number() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error!");
    input.trim().parse::<i64>().expect("parse error!")
}

fn get_result(limit: i64) -> i64 {
    let mut sum = 0;
    let mut previous = 1;
    let mut current = 1;
    while (current < limit)
    {
        if (current % 2 == 0)
        {
            sum += current;
        }

        let next = previous + current;
        previous = current;
        current = next;
    }
    sum
}


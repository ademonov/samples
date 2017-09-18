fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
  let step = g as u64;
  for i in m..n-step {
    if is_prime(i) {
        for j in i+1..i+step-1 {
            if is_prime(j) {
                continue;
            }
        }
        if is_prime(i+step) {
            return Some((i, i+step));
        }
    }
  }
  None
}

fn is_prime(x: u64) -> bool {
    let upper_limit = (x as f64).sqrt() as u64 + 1;
    for i in 2..upper_limit {
        if x % i == 0 {
            return false;
        }
    }
    true
}
extern crate futures;

fn main() {
    use futures::future::*;

    // Part 1
    let simple_future = lazy(|| {
        println!("Running the closure.");
        ok::<u32, ()>(42)
    });

    // Part 2
    println!("Waiting on the future.");
    let result = simple_future.wait();

    println!("{:?}", result);
}
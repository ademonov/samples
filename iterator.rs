fn main() {
    let a = ["StrA", "StrB", "StrC"];
    let it = a.iter();
    print1(it);


    let a = ["StrA", "StrB", "StrC"];
    let it = a.iter();
    print2(it);
    print2(["A".to_string(), "B".to_string()].iter());      
    
}

fn print1<'a, T>(iterator: T) where T: Iterator<Item=&'a &'a str> {
    for item in iterator {
        print!("{} ", item)
    }
}

fn print2<T, S>(iterator: T) 
where T: Iterator<Item=S>,
      S: AsRef<str> {
    for item in iterator {
        print!("{} ", item.as_ref())
    }
}

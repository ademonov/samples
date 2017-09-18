fn find_nb(n: u64) -> i32 {
    let mut v = 0u64;
    for i in 1.. {
        v += i*i*i;
        
        if v > n { 
            return -1;
        } 
        
        if v == n {
            return i as i32;
        }          
    }
    -1
    
}
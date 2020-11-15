pub fn factorial(num: u64) -> u64 {
    //Implement the function here
    println!("the number is: {}", num);
    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }

    if num == 2 {
        return 2;
    }

    // res = 10*9 -> res*8 -> res*7
    // 10 --> 10*9*8*7*6
    // 4 (24) -> 4*3*2*1

    let mut res = 1;
    for n in 1..num + 1 {
        println!("{}", res);
        res *= n;
    }

    res
}

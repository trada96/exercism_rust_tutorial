pub fn square_of_sum(n: u32) -> u32 {
    let sum: u32 = (1..n+1).sum();
    println!("sum {:?}", sum);
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let sum: u32 = (1..n+1).map(|x| x.pow(2)).sum();
    println!("sum {:?}", sum);
    sum
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

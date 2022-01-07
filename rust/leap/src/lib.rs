pub fn is_leap_year(year: u64) -> bool {
    // unimplemented!("true if {} is a leap year", year)

    let surplus_4:u64 = year % 4; // so du
    let surplus_100:u64 = year %100;
    let surplus_400:u64 = year %400;


    match (surplus_4, surplus_100, surplus_400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _,_) => true,
        (_,_,_)=>false
    }
    

}

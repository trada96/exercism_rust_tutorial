pub fn is_armstrong_number(num: u32) -> bool {
    // Example 153
    let num_to_str = num.to_string(); // = "153"
    let len_of_num = num_to_str.len() as u32; // = "3"

    let result = num_to_str.chars()
        .map(|c| c.to_digit(10).unwrap()) //= ("1", "5", 3)
        .map(|d| d.pow(len_of_num))//= (1, 125, 27)
        .sum();  // = 153

    num == result
}

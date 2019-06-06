pub fn is_armstrong_number(num: u32) -> bool {
    let txt: String = num.to_string();
    let count: u32 = txt.len() as u32;

    let sum = txt.chars().fold(0, |acc, x| {
        acc + (x.to_digit(10).unwrap()).pow(count)
    });
    sum == num
}


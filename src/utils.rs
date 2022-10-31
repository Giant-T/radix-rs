use rand::Rng;

pub fn fill_array(array: &mut [u32]) {
    let len = array.len();
    let mut rng = rand::thread_rng();

    for i in 0..len {
        array[i] = rng.gen_range(0..=len) as u32;
    }
}

fn count_digits(number: u32) -> u32 {
    if number / 10 == 0 {
        return 1;
    }

    return 1 + count_digits(number / 10);
}

pub fn max_number_digits(array: &[u32]) -> u32 {
    let mut max: u32 = array[0];

    for number in array {
        if number > &max {
            max = *number; 
        }
    }

    count_digits(max)
}

pub fn get_digit_at(number: &u32, digit: u32) -> u32 {
    return number / 10_u32.pow(digit) % 10_u32;
}

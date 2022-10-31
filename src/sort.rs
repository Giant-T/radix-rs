use super::utils;

fn remplir(array: &[u32], radix_array: &mut Vec<Vec<u32>>, digit: u32) {
    let mut digit_value: u32;

    for num in array {
        digit_value = utils::get_digit_at(num, digit);
        radix_array[digit_value as usize].push(*num);
    }
}

fn vider(array: &mut [u32], radix_array: &mut Vec<Vec<u32>>) {
    let mut index: usize = 0;

    for v in radix_array {
        while !v.is_empty() {
            array[index] = v.remove(0);
            index += 1;
        }
    }
}

pub fn radix(array: &mut [u32]) {
    let nb_digits: u32 = utils::max_number_digits(&array);
    let mut radix_array: Vec<Vec<u32>> = vec![vec![]; 10];

    for i in 0..nb_digits {
        remplir(array, &mut radix_array, i);
        vider(array, &mut radix_array);
    }
}

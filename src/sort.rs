use super::utils;

fn radixer(array: &mut Vec<u32>, digit: u32) {
    let mut radix_array: Vec<Vec<u32>> = vec![vec![]; 10];
    let mut index: usize = 0;

    array.iter().for_each(|num: &u32| {
        let digit_value = utils::get_digit_at(num, digit);
        radix_array[digit_value as usize].push(*num);
    });

    radix_array.iter().for_each(|v| {
        v.iter().for_each(|item| {
            array[index] = *item;
            index += 1;
        });
    });
}

pub fn radix(array: &mut Vec<u32>) {
    let nb_digits: u32 = utils::max_number_digits(&array);

    for i in 0..nb_digits {
        radixer(array, i);
    }
}

use super::utils;

fn radixer(array: &mut Vec<u32>, digit: u32) {
    let mut radix_array: Vec<Vec<u32>> = vec![vec![]; 10];
    let mut index: usize = 0;

    // en rust les foreachs sont des abstractions qui ne coute rien (cpu, memoire)
    // de plus que leurs équivoques plus verbeux.
    array.iter().for_each(|num: &u32| {
        let digit_value: u32 = utils::get_digit_at(num, digit);
        radix_array[digit_value as usize].push(*num);
    });

    //                             Ceci est une closure, une fonction anonyme qui est passée 
    //                           ↓ en paramètre d'une autre fonction.
    radix_array.iter().for_each(|v: &Vec<u32>| {
        v.iter().for_each(|item: &u32| {
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

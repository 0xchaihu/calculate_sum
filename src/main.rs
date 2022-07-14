fn cal_sum(array: &[u32]) -> Option<u32> {
    let mut it = array.iter();
    it.try_fold(0u32, |acc, &x| acc.checked_add(x))
}
fn main() {
    let array_0 = [941, 941, 941];
    let array_1 = [0xFFFFFFFF, 0, 2, 3, 4, 5, 6, 7, 8];

    let sum_of_array_0 = cal_sum(&array_0);
    print!("The sum of array_0 is: ");
    match sum_of_array_0 {
        Some(sum) => println!("{:?}", sum),
        None => {
            println!("overflow: None")
        }
    }

    let sum_of_array_1 = cal_sum(&array_1);
    print!("The sum of array_1 is: ");
    match sum_of_array_1 {
        Some(sum) => println!("{:?}", sum),
        None => {
            println!("overflow: None")
        }
    }
}

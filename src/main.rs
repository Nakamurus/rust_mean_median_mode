use std::collections::HashMap;

fn main() {
    let mut integer_list = [1, 9, 3, 6, 5, 6, 4, 8, 11, 2];

    let mut sum = 0;

    let mut map = HashMap::new();

    for element in integer_list.iter() {
        sum += element;
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }

    let mut max = 0;

    let mut mode: i32 = 0;

    println!("{:?}", map);

    for (k, v) in &map {
        if v > &max {
            max = *v;
            mode = **k;
        }
    }

    println!("mode: {}", mode);

    let length = integer_list.len() as f64;

    let mean = sum as f64 / length;

    println!("mean: {}", mean);

    integer_list.sort_unstable();

    let median_index = (length / 2.0).floor();

    let median = integer_list[median_index as usize];

    println!("median: {:?}", median);
}

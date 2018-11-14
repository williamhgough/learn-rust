use std::collections::HashMap;

fn main() {
    let nums = &[1, 2, 3, 4, 4, 4, 4, 3, 1, 5, 6, 7, 8, 22];
    println!("{}", mean(nums));
    println!("{}", median(nums));
    println!("{}", mode(nums));
}

fn mean(list: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}

fn median(list: &[i32]) -> f64 {
    let len = list.len();
    let mid = len / 2;
    if len % 2 == 0 {
        mean(&list[(mid - 1)..(mid + 1)])
    } else {
        f64::from(list[mid])
    }
}

fn mode(list: &[i32]) -> i32 {
    let mut occurences: HashMap<&i32, i32> = HashMap::new();
    let mut max: (i32, i32) = (0, 0);

    for entry in list {
        let count = occurences.entry(entry).or_insert(0);
        *count += 1;
    }

    for (&&key, &val) in &occurences {
        if val > max.1 {
            max = (key, val);
        }
    }

    max.0
}

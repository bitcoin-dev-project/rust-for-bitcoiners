fn generate_subsets(n: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    // Start with an empty subset
    let current = Vec::new();
    backtrack(&mut result, &current, 1, n);
    result
}

fn backtrack(result: &mut Vec<Vec<i32>>, current: &[i32], start: i32, n: i32) {
    // subset without the start element
    result.push(current.clone());

    for i in start..=n {
        let mut new_subset = current.clone();
        // subset with the start element
        new_subset.push(i);
        backtrack(result, &new_subset, i + 1, n);
    }
}

fn main() {
    let n = 3;
    let subsets = generate_subsets(n);
    for subset in subsets {
        println!("{:?}", subset);
    }
}

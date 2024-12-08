use std::env;
use std::fs;


fn main() {
    let mut first_vec: Vec<i64> = Vec::new();
    let mut second_vec: Vec<i64> = Vec::new();
    let mut final_value: i64 = 0;

    let file_path: &str = "/home/dilraj/Documents/hobby/AdventOfCode_2024/day_1/input.txt";
    
    let input_data = match fs::read_to_string(file_path) {
        Ok(in_data) => in_data,
        Err(_) => panic!("Problem with reading file."),
    };

    for (i, line) in input_data.split("\n").enumerate() {
        if i == (input_data.split("\n").collect::<Vec<_>>().len() - 1) {break;}
        for (j, input) in line.split("   ").enumerate() {
            if j == 0 {
                first_vec.push(input.trim().parse::<i64>().unwrap());
	    } else {
                second_vec.push(input.trim().parse::<i64>().unwrap());
            }
	}
    }
    first_vec.sort();
    second_vec.sort();

    for i in 0..first_vec.len() {
	let first = first_vec[i];
	let second = second_vec[i];
	let difference = first - second;
	final_value += difference.abs();
    }
    // Output to the first part!
    println!("First output is: {}", final_value);
    //
    
    let mut inter_value: i64 = 0;
    let mut unique_entries: Vec<i64> = Vec::new();
    let mut entry_counts: Vec<i64> = Vec::new();

    for entry in &second_vec {
        if *entry != inter_value {
            inter_value = *entry;
            unique_entries.push(inter_value);
            entry_counts.push(1);
        } else {
            let last_index = entry_counts.len() - 1;
            entry_counts[last_index] += 1;
        }
    }

    let mut second_value: i64 = 0;

    for entry in &first_vec {
        let index = match unique_entries.binary_search(entry) {
            Ok(r) => r,
            Err(_) => continue,
        };
        second_value += (*entry)*entry_counts[index];
    }
    println!("Second output is: {}", second_value);
}


extern crate chrono;

use std::collections::HashMap;
use std::io;

fn main() {
    let list = vec![1,2,1,3,2,3,3,3,4,5,6,5,7,3,2,8,5,1,9];

    mode(list);
    let map = add_employee_to_org();
    let dept = "b".to_string();
    let sorted_list = get_sorted_emplyee_for_dep(map, &dept);
    println!("Sorted list of employees for dept {} is {:?}", &dept, sorted_list);    
}

fn mode (list: Vec<i32>) {

    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in list {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut largest_count: i32 = 0;
    let mut largest_count_num: i32 = 0;

    for (num, count_num) in map {
        if count_num > largest_count {
            largest_count = count_num;
            largest_count_num = num;
        }
    }

    println!("The mode of the list is {} with count {}", largest_count_num, largest_count);
}

fn add_employee_to_org () -> HashMap<String, String> {
    // Initialise a Hashmap as a state bag
    let mut map: HashMap<String, String> = HashMap::new();
    loop {
        // Accept a string from IO
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to get input");
        let sliced_input = &input[..];

        // println!("{:?}", sliced_input);        
        // Switch to stop adding employees
        if sliced_input == "done\n" {
            // println!("{:?}", sliced_input);
            break;
        }

        let mut name: &str = "";
        let mut department: &str = "";
        let e: Vec<&str> = sliced_input.split_whitespace().collect();

        for (index, t) in e.iter().enumerate() {
            if index == 1 {
                name = *t;
            }

            if index == 3 {
                department = *t;
            }
        }
        
        let name = match name.trim().parse() {
            Ok(result)  => result,
            Err(_)      => continue
        };

        let department = match department.trim().parse() {
            Ok(result)  => result,
            Err(_)      => continue
        };

        map.entry(name).or_insert(department);
        // println!("The list of employees and their department is {:?}", map);
    }

    map
}

fn get_sorted_emplyee_for_dep (
    map: HashMap<String, String>, 
    dep: &String) -> Vec<String> {
    let mut list: Vec<String> = vec![];

    for (v, k) in map {
        if k.trim() == dep {
            list.push(v);
        }
    }
    
    list.sort();
    // println!("The list of employees and their department is {:?}", list);
    list
}

// fn RLE () {

//     // Read the data from the file
//     let story = String::from("A list of characters to encode using RLE and basic Huffman encoding. This will just be to use encode encode and reduce size");

//     // Create a map of words from word to character
//     let mut map = HashMap::new();

//     for word in story.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     // Then iterate through the words and replace the words with code

//     // If the word gets repeated consecutively, add 1 to the count
// }

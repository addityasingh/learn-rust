extern crate chrono;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let list = vec![1,2,1,3,2,3,3,3,4,5,6,5,7,3,2,8,5,1,9];
    
    mode(list);
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

fn addEmployeeToOrg () {

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

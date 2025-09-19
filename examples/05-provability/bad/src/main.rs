// BAD: Can panic, low provability score

fn divide_numbers(a: i32, b: i32) -> i32 {
    a / b // Will panic if b == 0
}

fn get_array_element(arr: &[i32], index: usize) -> i32 {
    arr[index] // Will panic if index out of bounds
}

fn parse_number(s: &str) -> i32 {
    s.parse().unwrap() // Will panic on invalid input
}

fn calculate_average(numbers: &[i32]) -> f64 {
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64 // Will panic if numbers is empty (division by zero)
}

fn get_first_char(s: &str) -> char {
    s.chars().next().unwrap() // Will panic on empty string
}

fn process_config(config: Option<String>) -> String {
    config.unwrap() // Will panic if None
}

fn access_nested_data(data: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    data[row][col] // Double panic potential
}

fn main() {
    println!("Division: {}", divide_numbers(10, 2));

    let numbers = vec![1, 2, 3, 4, 5];
    println!("Element: {}", get_array_element(&numbers, 2));

    println!("Parsed: {}", parse_number("42"));

    println!("Average: {}", calculate_average(&numbers));

    println!("First char: {}", get_first_char("hello"));

    println!("Config: {}", process_config(Some("production".to_string())));

    let matrix = vec![vec![1, 2], vec![3, 4]];
    println!("Matrix element: {}", access_nested_data(&matrix, 1, 0));
}
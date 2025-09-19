// GOOD: No panic potential, high provability

fn divide_numbers(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn get_array_element(arr: &[i32], index: usize) -> Option<i32> {
    arr.get(index).copied()
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse()
        .map_err(|e| format!("Failed to parse number: {}", e))
}

fn calculate_average(numbers: &[i32]) -> Result<f64, String> {
    if numbers.is_empty() {
        return Err("Cannot calculate average of empty array".to_string());
    }

    let sum: i32 = numbers.iter().sum();
    Ok(sum as f64 / numbers.len() as f64)
}

fn get_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

fn process_config(config: Option<String>) -> String {
    config.unwrap_or_else(|| "default".to_string())
}

fn access_nested_data(data: &[Vec<i32>], row: usize, col: usize) -> Result<i32, String> {
    data.get(row)
        .and_then(|row_data| row_data.get(col))
        .copied()
        .ok_or_else(|| format!("Index out of bounds: row {}, col {}", row, col))
}

fn main() {
    match divide_numbers(10, 2) {
        Ok(result) => println!("Division: {}", result),
        Err(e) => eprintln!("Division error: {}", e),
    }

    let numbers = vec![1, 2, 3, 4, 5];
    match get_array_element(&numbers, 2) {
        Some(element) => println!("Element: {}", element),
        None => println!("Index out of bounds"),
    }

    match parse_number("42") {
        Ok(parsed) => println!("Parsed: {}", parsed),
        Err(e) => eprintln!("Parse error: {}", e),
    }

    match calculate_average(&numbers) {
        Ok(avg) => println!("Average: {}", avg),
        Err(e) => eprintln!("Average error: {}", e),
    }

    match get_first_char("hello") {
        Some(ch) => println!("First char: {}", ch),
        None => println!("Empty string"),
    }

    println!("Config: {}", process_config(Some("production".to_string())));

    let matrix = vec![vec![1, 2], vec![3, 4]];
    match access_nested_data(&matrix, 1, 0) {
        Ok(element) => println!("Matrix element: {}", element),
        Err(e) => eprintln!("Matrix access error: {}", e),
    }
}
// GOOD: Complexity under 5 per function
fn process_user_input(input: &str, mode: i32, flags: u32) -> Result<String, String> {
    validate_input(input)?;

    match mode {
        1 => process_mode_one(input, flags),
        2 => process_mode_two(flags),
        _ => Err("Unknown mode".to_string()),
    }
}

fn validate_input(input: &str) -> Result<(), String> {
    if input.is_empty() {
        Err("Empty input".to_string())
    } else {
        Ok(())
    }
}

fn process_mode_one(input: &str, flags: u32) -> Result<String, String> {
    if flags & 0x01 == 0 {
        return Err("Flag not set".to_string());
    }

    if input.len() <= 100 {
        return Err("Input too short".to_string());
    }

    Ok(determine_user_type(input))
}

fn determine_user_type(input: &str) -> String {
    if input.contains("admin") {
        "Admin mode".to_string()
    } else if input.contains("user") {
        "User mode".to_string()
    } else {
        "Guest mode".to_string()
    }
}

fn process_mode_two(flags: u32) -> Result<String, String> {
    if flags & 0x02 != 0 {
        Ok("Mode 2 active".to_string())
    } else {
        Err("Mode 2 requires flag".to_string())
    }
}

fn main() {
    let result = process_user_input("admin_user_12345_very_long_input_to_exceed_100_characters_minimum_requirement", 1, 0x01);
    println!("Result: {:?}", result);
}
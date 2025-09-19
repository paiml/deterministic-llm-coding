// BAD: Cyclomatic complexity of 15+ (PMAT will flag this)
fn process_user_input(input: &str, mode: i32, flags: u32) -> Result<String, String> {
    if input.is_empty() {
        return Err("Empty input".to_string());
    } else {
        if mode == 1 {
            if flags & 0x01 != 0 {
                if input.len() > 100 {
                    if input.contains("admin") {
                        return Ok("Admin mode".to_string());
                    } else {
                        if input.contains("user") {
                            return Ok("User mode".to_string());
                        } else {
                            return Ok("Guest mode".to_string());
                        }
                    }
                } else {
                    return Err("Input too short".to_string());
                }
            } else {
                return Err("Flag not set".to_string());
            }
        } else if mode == 2 {
            if flags & 0x02 != 0 {
                return Ok("Mode 2 active".to_string());
            } else {
                return Err("Mode 2 requires flag".to_string());
            }
        } else {
            return Err("Unknown mode".to_string());
        }
    }
}

fn main() {
    let result = process_user_input("admin_user_12345_very_long_input_to_exceed_100_characters_minimum_requirement", 1, 0x01);
    println!("Result: {:?}", result);
}
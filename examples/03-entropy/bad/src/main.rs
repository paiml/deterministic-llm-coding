// BAD: Duplicated patterns (PMAT entropy detection will flag this)

fn process_user(name: &str, age: u32) -> String {
    let mut result = String::new();
    result.push_str("User: ");
    result.push_str(name);
    result.push_str(", Age: ");
    result.push_str(&age.to_string());
    result
}

fn process_admin(name: &str, level: u32) -> String {
    let mut result = String::new();
    result.push_str("Admin: ");
    result.push_str(name);
    result.push_str(", Level: ");
    result.push_str(&level.to_string());
    result
}

fn process_guest(name: &str, duration: u32) -> String {
    let mut result = String::new();
    result.push_str("Guest: ");
    result.push_str(name);
    result.push_str(", Duration: ");
    result.push_str(&duration.to_string());
    result
}

fn format_user_details(name: &str, email: &str, id: u32) -> String {
    let mut details = String::new();
    details.push_str("Name: ");
    details.push_str(name);
    details.push_str(", Email: ");
    details.push_str(email);
    details.push_str(", ID: ");
    details.push_str(&id.to_string());
    details
}

fn format_admin_details(name: &str, email: &str, id: u32) -> String {
    let mut details = String::new();
    details.push_str("Name: ");
    details.push_str(name);
    details.push_str(", Email: ");
    details.push_str(email);
    details.push_str(", ID: ");
    details.push_str(&id.to_string());
    details
}

fn format_guest_details(name: &str, email: &str, id: u32) -> String {
    let mut details = String::new();
    details.push_str("Name: ");
    details.push_str(name);
    details.push_str(", Email: ");
    details.push_str(email);
    details.push_str(", ID: ");
    details.push_str(&id.to_string());
    details
}

fn main() {
    println!("{}", process_user("Alice", 25));
    println!("{}", process_admin("Bob", 3));
    println!("{}", process_guest("Charlie", 7));

    println!("{}", format_user_details("Alice", "alice@example.com", 101));
    println!("{}", format_admin_details("Bob", "bob@example.com", 102));
    println!("{}", format_guest_details("Charlie", "charlie@example.com", 103));
}
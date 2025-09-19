// GOOD: DRY principle applied, no duplication

fn format_entity(entity_type: &str, name: &str, attribute: &str, value: u32) -> String {
    format!("{}: {}, {}: {}", entity_type, name, attribute, value)
}

fn process_user(name: &str, age: u32) -> String {
    format_entity("User", name, "Age", age)
}

fn process_admin(name: &str, level: u32) -> String {
    format_entity("Admin", name, "Level", level)
}

fn process_guest(name: &str, duration: u32) -> String {
    format_entity("Guest", name, "Duration", duration)
}

fn format_contact_details(name: &str, email: &str, id: u32) -> String {
    format!("Name: {}, Email: {}, ID: {}", name, email, id)
}

fn format_user_details(name: &str, email: &str, id: u32) -> String {
    format_contact_details(name, email, id)
}

fn format_admin_details(name: &str, email: &str, id: u32) -> String {
    format_contact_details(name, email, id)
}

fn format_guest_details(name: &str, email: &str, id: u32) -> String {
    format_contact_details(name, email, id)
}

fn main() {
    println!("{}", process_user("Alice", 25));
    println!("{}", process_admin("Bob", 3));
    println!("{}", process_guest("Charlie", 7));

    println!("{}", format_user_details("Alice", "alice@example.com", 101));
    println!("{}", format_admin_details("Bob", "bob@example.com", 102));
    println!("{}", format_guest_details("Charlie", "charlie@example.com", 103));
}
pub fn prioritize_event(role: &str) -> u8 {
    match role {
        "admin" => 1,
        "support" => 2,
        "user" => 3,
        _ => 4,
    }
}


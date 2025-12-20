use rand::{distributions::Alphanumeric, Rng};

/// Generate unique booking code (LB-XXXXX)
pub fn generate_booking_code() -> String {
    let suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    format!("LB-{}", suffix.to_uppercase())
}

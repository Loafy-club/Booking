use rand::{distributions::Alphanumeric, Rng};

/// Calculate the total number of slots needed for a booking.
///
/// A booking always includes the user plus any guests they bring.
///
/// # Arguments
/// * `guest_count` - Number of guests the user is bringing
///
/// # Returns
/// Total slots needed (1 for the user + guest_count)
#[inline]
pub fn calculate_total_slots(guest_count: i32) -> i32 {
    1 + guest_count
}

/// Generate unique booking code (LB-XXXXX)
pub fn generate_booking_code() -> String {
    let suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    format!("LB-{}", suffix.to_uppercase())
}

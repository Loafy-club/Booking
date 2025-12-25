//! Validation helpers for enum values.
//!
//! These constants and functions centralize validation of string values
//! against known valid options.

/// Valid user role values
pub const VALID_ROLES: &[&str] = &["user", "organizer", "admin"];

/// Valid payment status values
pub const VALID_PAYMENT_STATUSES: &[&str] = &["pending", "confirmed", "failed", "refunded"];

/// Valid payment method values
pub const VALID_PAYMENT_METHODS: &[&str] = &["qr", "stripe", "cash", "free"];

/// Validates that a value is one of the allowed values.
///
/// # Arguments
/// * `value` - The value to validate
/// * `valid_values` - Slice of valid values
/// * `field_name` - Name of the field (for error message)
///
/// # Returns
/// * `Ok(())` if value is valid
/// * `Err(String)` with a descriptive error message if invalid
pub fn validate_enum_value(value: &str, valid_values: &[&str], field_name: &str) -> Result<(), String> {
    if valid_values.contains(&value) {
        Ok(())
    } else {
        Err(format!(
            "Invalid {}. Must be one of: {}",
            field_name,
            valid_values.join(", ")
        ))
    }
}

/// Validates a role value
pub fn validate_role(role: &str) -> Result<(), String> {
    validate_enum_value(role, VALID_ROLES, "role")
}

/// Validates a payment status value
pub fn validate_payment_status(status: &str) -> Result<(), String> {
    validate_enum_value(status, VALID_PAYMENT_STATUSES, "payment status")
}

/// Validates a payment method value
pub fn validate_payment_method(method: &str) -> Result<(), String> {
    validate_enum_value(method, VALID_PAYMENT_METHODS, "payment method")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_role() {
        assert!(validate_role("user").is_ok());
        assert!(validate_role("organizer").is_ok());
        assert!(validate_role("admin").is_ok());
        assert!(validate_role("invalid").is_err());
    }

    #[test]
    fn test_validate_payment_status() {
        assert!(validate_payment_status("pending").is_ok());
        assert!(validate_payment_status("confirmed").is_ok());
        assert!(validate_payment_status("invalid").is_err());
    }

    #[test]
    fn test_validate_payment_method() {
        assert!(validate_payment_method("stripe").is_ok());
        assert!(validate_payment_method("qr").is_ok());
        assert!(validate_payment_method("invalid").is_err());
    }
}

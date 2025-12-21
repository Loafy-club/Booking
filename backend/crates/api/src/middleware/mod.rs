pub mod auth;

pub use auth::{AuthUser, AppState, require_role};

// OptionalAuthUser is defined but not currently exported/used
// Re-export when needed: pub use auth::OptionalAuthUser;

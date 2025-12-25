use chrono::{DateTime, Duration, Utc};

/// Represents a parsed period filter with optional since date and duration
#[derive(Debug, Clone)]
pub struct PeriodFilter {
    /// The start date for filtering (None means no date filter / "all time")
    pub since: Option<DateTime<Utc>>,
    /// The number of days in the period (None for "all time")
    pub days: Option<i64>,
}

impl PeriodFilter {
    /// Returns the since date, using the default (30 days ago) if not set
    pub fn since_or_default(&self) -> DateTime<Utc> {
        self.since.unwrap_or_else(|| Utc::now() - Duration::days(30))
    }
}

/// Parse a period string into a PeriodFilter
///
/// Supported values:
/// - "7d" - Last 7 days
/// - "30d" - Last 30 days
/// - "90d" - Last 90 days
/// - "365d" - Last 365 days
/// - "all" - All time (no date filter)
/// - Any other value defaults to 30 days
pub fn parse_period(period: &str) -> PeriodFilter {
    match period {
        "7d" => PeriodFilter {
            since: Some(Utc::now() - Duration::days(7)),
            days: Some(7),
        },
        "30d" => PeriodFilter {
            since: Some(Utc::now() - Duration::days(30)),
            days: Some(30),
        },
        "90d" => PeriodFilter {
            since: Some(Utc::now() - Duration::days(90)),
            days: Some(90),
        },
        "365d" => PeriodFilter {
            since: Some(Utc::now() - Duration::days(365)),
            days: Some(365),
        },
        "all" => PeriodFilter {
            since: None,
            days: None,
        },
        _ => PeriodFilter {
            since: Some(Utc::now() - Duration::days(30)),
            days: Some(30),
        },
    }
}

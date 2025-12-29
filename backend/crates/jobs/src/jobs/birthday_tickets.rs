use chrono::{Datelike, Local};
use loafy_db::{
    models::{bonus_types, transaction_types},
    queries::{config, subscriptions, ticket_transactions, users},
    PgPool,
};

/// Allocate birthday tickets to eligible users
/// Runs daily at 00:01 local time
///
/// Eligibility requirements:
/// - User has birthday set
/// - Account is at least 30 days old (configurable)
/// - Has an active subscription
/// - Has not already received birthday bonus this year
pub async fn allocate_birthday_tickets(pool: &PgPool) -> anyhow::Result<()> {
    let today = Local::now().naive_local().date();
    let current_year = today.year();

    // Get configuration values
    let min_account_age_days = config::get_birthday_account_age_days(pool).await.unwrap_or(30);
    let bonus_tickets = config::get_birthday_bonus_tickets(pool).await.unwrap_or(1);

    // Find eligible users
    let eligible_users =
        users::find_birthday_bonus_eligible(pool, today, min_account_age_days, current_year)
            .await?;

    if eligible_users.is_empty() {
        tracing::debug!("No users eligible for birthday bonus today");
        return Ok(());
    }

    tracing::info!(
        "Found {} users eligible for birthday bonus today",
        eligible_users.len()
    );

    for user in eligible_users {
        tracing::info!(
            "Processing birthday bonus for user {} ({})",
            user.id,
            user.email
        );

        // Get user's subscription
        let subscription = match subscriptions::find_by_user_id(pool, user.id).await? {
            Some(sub) if sub.is_active() => sub,
            _ => {
                tracing::warn!(
                    "User {} no longer has active subscription, skipping",
                    user.id
                );
                continue;
            }
        };

        // Add bonus tickets to subscription
        match subscriptions::add_bonus_tickets(pool, subscription.id, bonus_tickets).await {
            Ok(new_balance) => {
                // Log the ticket transaction
                if let Err(e) = ticket_transactions::create_with_pool(
                    pool,
                    user.id,
                    Some(subscription.id),
                    None,
                    transaction_types::BONUS_BIRTHDAY,
                    bonus_tickets,
                    new_balance,
                    Some("Birthday bonus ticket"),
                    None,
                )
                .await
                {
                    tracing::error!(
                        "Failed to log birthday ticket transaction for user {}: {}",
                        user.id,
                        e
                    );
                }

                // Record the bonus ticket award to prevent duplicates
                if let Err(e) = ticket_transactions::create_bonus_ticket(
                    pool,
                    user.id,
                    bonus_types::BIRTHDAY,
                    bonus_tickets,
                    Some("Birthday bonus"),
                    None,
                    None,
                    Some(current_year),
                )
                .await
                {
                    tracing::error!(
                        "Failed to record birthday bonus for user {}: {}",
                        user.id,
                        e
                    );
                }

                tracing::info!(
                    "✓ Granted {} birthday ticket(s) to user {} - new balance: {}",
                    bonus_tickets,
                    user.id,
                    new_balance
                );
            }
            Err(e) => {
                tracing::error!(
                    "Failed to add birthday tickets for user {}: {}",
                    user.id,
                    e
                );
            }
        }
    }

    Ok(())
}

-- Fix bookings where price_paid_vnd incorrectly included guest prices
-- The bug: price_paid_vnd was set to (user_price + guest_price) instead of just user_price
-- This caused double-counting when calculating total as price_paid_vnd + guest_price_paid_vnd

UPDATE bookings
SET price_paid_vnd = price_paid_vnd - guest_price_paid_vnd
WHERE guest_count > 0
  AND guest_price_paid_vnd > 0;

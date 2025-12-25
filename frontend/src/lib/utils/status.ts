/**
 * Status utilities for badge variants and labels.
 *
 * These utilities centralize the logic for determining badge variants
 * based on status values, ensuring consistency across the application.
 */

import type { BadgeVariant } from '$lib/components/ui/badge/badge.svelte';

/**
 * User role type for type safety.
 */
export type UserRole = 'admin' | 'organizer' | 'user';

/**
 * Payment status type for type safety.
 */
export type PaymentStatus = 'pending' | 'confirmed' | 'failed' | 'refunded';

/**
 * Session status type for type safety.
 */
export type SessionStatus = 'upcoming' | 'in-progress' | 'completed' | 'cancelled' | 'full';

/**
 * Gets the badge variant for a user role.
 *
 * @param role - The user role (admin, organizer, user)
 * @returns The appropriate badge variant
 *
 * @example
 * ```svelte
 * <Badge variant={getRoleBadgeVariant(user.role)}>{user.role}</Badge>
 * ```
 */
export function getRoleBadgeVariant(role: string): BadgeVariant {
	switch (role) {
		case 'admin':
			return 'admin';
		case 'organizer':
			return 'organizer';
		default:
			return 'muted';
	}
}

/**
 * Gets the badge variant for a payment status.
 *
 * @param status - The payment status (pending, confirmed, failed, refunded)
 * @returns The appropriate badge variant
 *
 * @example
 * ```svelte
 * <Badge variant={getPaymentBadgeVariant(booking.payment_status)}>
 *   {booking.payment_status}
 * </Badge>
 * ```
 */
export function getPaymentBadgeVariant(status: string): BadgeVariant {
	switch (status) {
		case 'confirmed':
			return 'confirmed';
		case 'pending':
			return 'pending';
		case 'failed':
		case 'refunded':
			return 'failed';
		default:
			return 'muted';
	}
}

/**
 * Gets the badge variant for a booking status, considering cancellation.
 *
 * @param paymentStatus - The payment status of the booking
 * @param cancelled - Whether the booking has been cancelled
 * @returns The appropriate badge variant
 *
 * @example
 * ```svelte
 * <Badge variant={getBookingBadgeVariant(booking.payment_status, !!booking.cancelled_at)}>
 *   {getBookingStatusKey(booking.payment_status, !!booking.cancelled_at)}
 * </Badge>
 * ```
 */
export function getBookingBadgeVariant(
	paymentStatus: string,
	cancelled: boolean
): BadgeVariant {
	if (cancelled) return 'cancelled';
	if (paymentStatus === 'confirmed') return 'confirmed';
	return 'pending';
}

/**
 * Gets the i18n key for a booking status.
 *
 * @param paymentStatus - The payment status of the booking
 * @param cancelled - Whether the booking has been cancelled
 * @returns The status key for translation (e.g., 'cancelled', 'confirmed', 'pending')
 *
 * @example
 * ```svelte
 * {t(`admin.bookings.status.${getBookingStatusKey(booking.payment_status, cancelled)}`)}
 * ```
 */
export function getBookingStatusKey(
	paymentStatus: string,
	cancelled: boolean
): 'cancelled' | 'confirmed' | 'pending' {
	if (cancelled) return 'cancelled';
	if (paymentStatus === 'confirmed') return 'confirmed';
	return 'pending';
}

/**
 * Gets the badge variant for a session status.
 *
 * @param status - The session status
 * @param cancelled - Whether the session has been cancelled
 * @param isFull - Whether the session is at full capacity
 * @returns The appropriate badge variant
 *
 * @example
 * ```svelte
 * <Badge variant={getSessionBadgeVariant(session.status, session.cancelled, session.available_slots === 0)}>
 *   {session.status}
 * </Badge>
 * ```
 */
export function getSessionBadgeVariant(
	status: string,
	cancelled: boolean,
	isFull: boolean = false
): BadgeVariant {
	if (cancelled) return 'cancelled';
	if (isFull) return 'full';

	switch (status) {
		case 'upcoming':
			return 'upcoming';
		case 'in-progress':
			return 'in-progress';
		case 'completed':
			return 'completed';
		default:
			return 'muted';
	}
}

import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import type { Snippet } from 'svelte';
import type { Booking, Session } from './types';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

// Type for components that accept element refs (required by shadcn-svelte)
export type WithElementRef<T, E extends HTMLElement = HTMLElement> = T & {
	ref?: E | null;
};

// Types required by shadcn-svelte dropdown-menu components
export type WithoutChild<T> = T extends { child?: unknown } ? Omit<T, 'child'> : T;
export type WithoutChildren<T> = T extends { children?: unknown } ? Omit<T, 'children'> : T;
export type WithoutChildrenOrChild<T> = WithoutChild<WithoutChildren<T>>;

export function formatCurrency(amount: number, currency = 'VND'): string {
	return new Intl.NumberFormat('vi-VN', {
		style: 'currency',
		currency
	}).format(amount);
}

export function formatDate(date: string | Date, format: 'short' | 'long' = 'short'): string {
	const d = typeof date === 'string' ? new Date(date) : date;

	if (format === 'short') {
		return new Intl.DateTimeFormat('vi-VN', {
			dateStyle: 'short',
			timeStyle: 'short'
		}).format(d);
	}

	return new Intl.DateTimeFormat('vi-VN', {
		dateStyle: 'long',
		timeStyle: 'short'
	}).format(d);
}

// ============================================================================
// Booking Helper Functions
// ============================================================================

/**
 * Check if a booking is pending payment (not cancelled and payment not confirmed)
 */
export function isBookingPending(booking: Booking): boolean {
	return booking.payment_status === 'pending' && !booking.cancelled_at;
}

/**
 * Check if a booking can be cancelled (not already cancelled and payment not confirmed)
 */
export function canCancelBooking(booking: Booking): boolean {
	return !booking.cancelled_at && booking.payment_status !== 'confirmed';
}

/**
 * Calculate total price for a booking
 */
export function getBookingTotal(booking: Booking): number {
	return booking.price_paid_vnd + booking.guest_price_paid_vnd;
}

// ============================================================================
// Session Helper Functions
// ============================================================================

/**
 * Combine session date and time into a Date object
 */
export function getSessionDateTime(session: Session): Date {
	return new Date(`${session.date}T${session.time}`);
}

/**
 * Check if a session can be booked
 */
export function canBookSession(session: Session): boolean {
	if (session.cancelled) return false;
	if (session.available_slots === 0) return false;

	const now = new Date();
	const startTime = getSessionDateTime(session);

	return now < startTime;
}

// ============================================================================
// Error Handling
// ============================================================================

/**
 * Extract a user-friendly error message from various error types.
 * Handles Axios errors, standard Error objects, and unknown error types.
 */
export function extractErrorMessage(error: unknown, fallback = 'An unexpected error occurred'): string {
	if (!error) return fallback;

	// Axios error with response
	if (typeof error === 'object' && error !== null) {
		const err = error as Record<string, unknown>;

		// Axios error format: err.response?.data?.message
		if (err.response && typeof err.response === 'object') {
			const response = err.response as Record<string, unknown>;
			if (response.data && typeof response.data === 'object') {
				const data = response.data as Record<string, unknown>;
				if (typeof data.message === 'string') {
					return data.message;
				}
			}
		}

		// Standard Error object
		if (err.message && typeof err.message === 'string') {
			return err.message;
		}
	}

	// String error
	if (typeof error === 'string') {
		return error;
	}

	return fallback;
}

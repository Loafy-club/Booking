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

export function formatCompactCurrency(amount: number, currency = 'VND'): string {
	const symbol = currency === 'VND' ? '₫' : '$';

	if (amount >= 1_000_000_000) {
		return `${(amount / 1_000_000_000).toFixed(1).replace(/\.0$/, '')}B ${symbol}`;
	}
	if (amount >= 1_000_000) {
		return `${(amount / 1_000_000).toFixed(1).replace(/\.0$/, '')}M ${symbol}`;
	}
	if (amount >= 1_000) {
		return `${(amount / 1_000).toFixed(0)}K ${symbol}`;
	}
	return `${amount} ${symbol}`;
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

/**
 * Format date only (no time) - e.g., "Sat, Dec 28"
 */
export function formatDateOnly(date: string | Date): string {
	const d = typeof date === 'string' ? new Date(date) : date;
	return new Intl.DateTimeFormat('en-US', {
		weekday: 'short',
		month: 'short',
		day: 'numeric'
	}).format(d);
}

/**
 * Format time only - e.g., "10:00 AM"
 */
export function formatTime(time: string): string {
	// time is in "HH:MM:SS" format
	const [hours, minutes] = time.split(':').map(Number);
	const date = new Date();
	date.setHours(hours, minutes, 0);
	return new Intl.DateTimeFormat('en-US', {
		hour: 'numeric',
		minute: '2-digit',
		hour12: true
	}).format(date);
}

/**
 * Calculate duration between start and end times
 * Returns formatted string like "2h" or "1h 30m"
 */
export function formatDuration(startTime: string, endTime?: string): string | null {
	if (!endTime) return null;

	const [startHours, startMinutes] = startTime.split(':').map(Number);
	const [endHours, endMinutes] = endTime.split(':').map(Number);

	let durationMinutes = (endHours * 60 + endMinutes) - (startHours * 60 + startMinutes);

	// Handle sessions crossing midnight
	if (durationMinutes < 0) {
		durationMinutes += 24 * 60;
	}

	const hours = Math.floor(durationMinutes / 60);
	const minutes = durationMinutes % 60;

	if (hours === 0) {
		return `${minutes}m`;
	} else if (minutes === 0) {
		return `${hours}h`;
	} else {
		return `${hours}h ${minutes}m`;
	}
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
 * Check if a booking can be cancelled (not already cancelled)
 * Note: Backend enforces cancellation deadline based on subscription status
 */
export function canCancelBooking(booking: Booking): boolean {
	return !booking.cancelled_at && booking.payment_status !== 'cancelled';
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

		// Axios error format: err.response?.data?.message or err.response?.data (plain text)
		if (err.response && typeof err.response === 'object') {
			const response = err.response as Record<string, unknown>;
			if (response.data) {
				// Plain text error response
				if (typeof response.data === 'string' && response.data.trim()) {
					return response.data;
				}
				// JSON error response with message field
				if (typeof response.data === 'object') {
					const data = response.data as Record<string, unknown>;
					if (typeof data.message === 'string') {
						return data.message;
					}
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

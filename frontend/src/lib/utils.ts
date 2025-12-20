import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

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

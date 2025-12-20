import { goto } from '$app/navigation';
import { authStore } from '$lib/stores/auth.svelte';

export function requireAuth() {
	if (!authStore.isAuthenticated) {
		goto('/auth/login');
		return false;
	}
	return true;
}

export function requireRole(role: 'user' | 'organizer' | 'admin') {
	if (!authStore.isAuthenticated) {
		goto('/auth/login');
		return false;
	}

	const userRole = authStore.user?.role;

	if (role === 'admin' && userRole !== 'admin') {
		goto('/');
		return false;
	}

	if (role === 'organizer' && userRole !== 'organizer' && userRole !== 'admin') {
		goto('/');
		return false;
	}

	return true;
}

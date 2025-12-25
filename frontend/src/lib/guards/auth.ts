import { goto } from '$app/navigation';
import { authStore } from '$lib/stores/auth.svelte';

/**
 * Wait for auth store to finish loading and user data to be fetched.
 * Returns a promise that resolves when auth is ready.
 * Times out after 5 seconds to prevent infinite waiting.
 */
async function waitForAuth(): Promise<void> {
	const timeout = 5000;
	const startTime = Date.now();

	// Poll until loading is complete AND user data is available (if authenticated)
	return new Promise((resolve) => {
		const check = () => {
			// Timeout after 5 seconds
			if (Date.now() - startTime > timeout) {
				resolve();
				return;
			}

			// Wait for loading to complete
			if (authStore.loading) {
				setTimeout(check, 50);
				return;
			}

			// If we have a supabase user but no backend user yet, wait for user fetch
			if (authStore.supabaseUser && !authStore.user) {
				setTimeout(check, 50);
				return;
			}

			resolve();
		};
		check();
	});
}

export async function requireAuth(): Promise<boolean> {
	// Wait for auth store to finish loading before checking authentication
	await waitForAuth();

	if (!authStore.isAuthenticated) {
		goto('/auth/login');
		return false;
	}
	return true;
}

export async function requireRole(role: 'user' | 'organizer' | 'admin'): Promise<boolean> {
	await waitForAuth();

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

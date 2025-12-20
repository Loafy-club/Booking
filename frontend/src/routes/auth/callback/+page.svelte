<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { supabase } from '$lib/auth/supabase';
	import { api } from '$lib/api/client';
	import { authStore } from '$lib/stores/auth.svelte';

	let error = $state<string | null>(null);
	let processing = $state(true);

	onMount(async () => {
		try {
			// Get the auth code from URL hash
			const hashParams = new URLSearchParams(window.location.hash.substring(1));
			const code = hashParams.get('code');

			if (!code) {
				// Try to get session from Supabase (already handled by Supabase client)
				const {
					data: { session }
				} = await supabase.auth.getSession();

				if (!session) {
					throw new Error('No authentication code or session found');
				}

				// Exchange with backend
				await api.auth.callback(session.access_token);

				// Fetch user data
				await authStore.fetchUser();

				// Redirect to home
				goto('/');
			} else {
				// Exchange code with Supabase
				const { error: exchangeError } = await supabase.auth.exchangeCodeForSession(code);

				if (exchangeError) {
					throw exchangeError;
				}

				// Get the new session
				const {
					data: { session }
				} = await supabase.auth.getSession();

				if (!session) {
					throw new Error('Failed to get session after code exchange');
				}

				// Exchange with backend
				await api.auth.callback(session.access_token);

				// Fetch user data
				await authStore.fetchUser();

				// Redirect to home
				goto('/');
			}
		} catch (err: any) {
			console.error('Auth callback error:', err);
			error = err.message || 'Authentication failed';
			processing = false;

			// Redirect to login after 3 seconds
			setTimeout(() => {
				goto('/auth/login');
			}, 3000);
		}
	});
</script>

<svelte:head>
	<title>Authenticating... - Loafy Club</title>
</svelte:head>

<div class="flex min-h-screen items-center justify-center">
	<div class="text-center">
		{#if processing}
			<div class="mb-4 h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
			<h2 class="text-xl font-semibold">Signing you in...</h2>
			<p class="mt-2 text-sm text-muted-foreground">Please wait while we complete authentication</p>
		{:else if error}
			<div class="rounded-lg bg-destructive/10 p-6">
				<h2 class="text-xl font-semibold text-destructive">Authentication Failed</h2>
				<p class="mt-2 text-sm text-muted-foreground">{error}</p>
				<p class="mt-4 text-xs text-muted-foreground">Redirecting to login...</p>
			</div>
		{/if}
	</div>
</div>

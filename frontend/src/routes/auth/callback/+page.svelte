<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { supabase } from '$lib/auth/supabase';
	import { api } from '$lib/api/client';
	import { authStore } from '$lib/stores/auth.svelte';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { Card } from '$lib/components/ui/card';
	import { Logo } from '$lib/components/ui/logo';
	import { Spinner } from '$lib/components/ui/spinner';
	import { X } from 'lucide-svelte';

	const t = useTranslation();

	// Data from +page.ts load function
	let { data } = $props();

	let error = $state<string | null>(null);
	let processing = $state(true);

	onMount(() => {
		console.log('=== Auth Callback Page Loaded ===');
		console.log('Full URL:', window.location.href);
		console.log('Hash present:', !!window.location.hash);

		// Check for OAuth errors in URL params
		if (data.error) {
			console.error('OAuth error:', data.error, data.error_description);
			error = data.error_description || `OAuth error: ${data.error}`;
			processing = false;
			setTimeout(() => goto('/auth/login'), 5000);
			return;
		}

		let handled = false;

		const handleSession = async (session: any) => {
			if (handled) return;
			handled = true;

			console.log('Processing session for:', session.user?.email);

			// Register with backend
			try {
				await api.auth.callback(session.access_token);
				console.log('Backend registration successful');
			} catch (apiError: any) {
				console.warn('Backend callback failed:', apiError.message);
			}

			// Initialize auth store with the session
			await authStore.initialize(session);

			// Redirect to home
			goto('/');
		};

		const processHashTokens = async () => {
			// Parse tokens from URL hash (implicit flow)
			const hash = window.location.hash.substring(1);
			if (!hash) {
				console.log('No hash in URL');
				return false;
			}

			const params = new URLSearchParams(hash);
			const accessToken = params.get('access_token');
			const refreshToken = params.get('refresh_token');
			const expiresAt = params.get('expires_at');
			const expiresIn = params.get('expires_in');

			if (!accessToken || !refreshToken) {
				console.log('No tokens in hash');
				return false;
			}

			console.log('Found tokens in hash');

			try {
				// Decode JWT to get user info for logging
				const tokenParts = accessToken.split('.');
				const payload = JSON.parse(atob(tokenParts[1]));
				console.log('User email:', payload.email);

				// Clear the hash from URL first
				window.history.replaceState(null, '', window.location.pathname);

				// Use Supabase's setSession to properly integrate with its state management
				// This is the correct way to set a session from tokens
				console.log('Setting session via Supabase setSession...');
				const { data, error: setSessionError } = await supabase.auth.setSession({
					access_token: accessToken,
					refresh_token: refreshToken
				});

				if (setSessionError) {
					console.error('Failed to set session:', setSessionError.message);
					throw setSessionError;
				}

				if (!data.session) {
					console.error('setSession returned no session');
					throw new Error('Failed to establish session');
				}

				console.log('Session set successfully for:', data.session.user?.email);

				// Handle the session
				await handleSession(data.session);
				return true;
			} catch (err: any) {
				console.error('Failed to process tokens:', err.message);
			}

			return false;
		};

		// Process hash tokens immediately
		processHashTokens().then(async (success) => {
			if (success || handled) return;

			// If no hash tokens, check for existing session
			console.log('Checking for existing session...');
			const { data: { session } } = await supabase.auth.getSession();

			if (session) {
				await handleSession(session);
			} else {
				console.error('No session found');
				error = 'Authentication failed. Please try again.';
				processing = false;
				setTimeout(() => goto('/auth/login'), 5000);
			}
		});
	});
</script>

<svelte:head>
	<title>Authenticating... - Loafy Club</title>
</svelte:head>

<PageBackground>
	<div class="fixed inset-0 flex items-center justify-center">
		<div class="w-full max-w-sm px-4">
			<div class="mb-8 flex justify-center">
				<Logo size="lg" showText={false} />
			</div>

			<Card variant="glass">
				<div class="text-center py-8 px-4">
					{#if processing}
						<div class="flex justify-center mb-6">
							<Spinner class="size-16 text-orange-500" />
						</div>
						<h2 class="text-2xl font-bold text-foreground font-display">
							{t('auth.callback.signingIn')}
						</h2>
						<p class="mt-3 text-muted-foreground">{t('auth.callback.pleaseWait')}</p>
					{:else if error}
						<div class="flex justify-center mb-6">
							<div class="h-16 w-16 flex items-center justify-center rounded-full bg-error-bg">
								<X class="h-8 w-8 text-error-text" />
							</div>
						</div>
						<h2 class="text-2xl font-bold text-foreground font-display">
							{t('auth.callback.failed')}
						</h2>
						<p class="mt-3 text-muted-foreground text-sm">{error}</p>
						<p class="mt-4 text-sm text-muted-foreground">{t('auth.callback.redirecting')}</p>
					{/if}
				</div>
			</Card>
		</div>
	</div>
</PageBackground>

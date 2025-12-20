<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { supabase } from '$lib/auth/supabase';
	import { api } from '$lib/api/client';
	import { authStore } from '$lib/stores/auth.svelte';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { Logo } from '$lib/components/ui/logo';

	let error = $state<string | null>(null);
	let processing = $state(true);

	onMount(async () => {
		try {
			const hashParams = new URLSearchParams(window.location.hash.substring(1));
			const code = hashParams.get('code');

			if (!code) {
				const {
					data: { session }
				} = await supabase.auth.getSession();

				if (!session) {
					throw new Error('No authentication code or session found');
				}

				await api.auth.callback(session.access_token);
				await authStore.fetchUser();
				goto('/');
			} else {
				const { error: exchangeError } = await supabase.auth.exchangeCodeForSession(code);

				if (exchangeError) {
					throw exchangeError;
				}

				const {
					data: { session }
				} = await supabase.auth.getSession();

				if (!session) {
					throw new Error('Failed to get session after code exchange');
				}

				await api.auth.callback(session.access_token);
				await authStore.fetchUser();
				goto('/');
			}
		} catch (err: any) {
			console.error('Auth callback error:', err);
			error = err.message || 'Authentication failed';
			processing = false;

			setTimeout(() => {
				goto('/auth/login');
			}, 3000);
		}
	});
</script>

<svelte:head>
	<title>Authenticating... - Loafy Club</title>
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
	<link href="https://fonts.googleapis.com/css2?family=Baloo+2:wght@400;500;600;700;800&display=swap" rel="stylesheet">
</svelte:head>

<PageBackground>
	<div class="flex min-h-screen items-center justify-center px-4">
		<div class="w-full max-w-sm">
			<div class="mb-8 flex justify-center">
				<Logo size="lg" showText={false} />
			</div>

			<GlassCard>
				<div class="text-center py-4">
					{#if processing}
						<div class="mx-auto mb-6 h-12 w-12 animate-spin rounded-full border-4 border-orange-200 border-t-orange-500"></div>
						<h2 class="text-xl font-bold text-gray-800" style="font-family: 'Baloo 2', sans-serif;">
							Signing you in...
						</h2>
						<p class="mt-2 text-sm text-gray-600">Please wait while we complete authentication</p>
					{:else if error}
						<div class="mx-auto mb-6 flex h-12 w-12 items-center justify-center rounded-full bg-red-100">
							<svg class="h-6 w-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
							</svg>
						</div>
						<h2 class="text-xl font-bold text-gray-800" style="font-family: 'Baloo 2', sans-serif;">
							Authentication Failed
						</h2>
						<p class="mt-2 text-sm text-gray-600">{error}</p>
						<p class="mt-4 text-xs text-gray-500">Redirecting to login...</p>
					{/if}
				</div>
			</GlassCard>
		</div>
	</div>
</PageBackground>

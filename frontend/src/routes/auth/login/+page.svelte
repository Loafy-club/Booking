<script lang="ts">
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { Logo } from '$lib/components/ui/logo';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { onMount } from 'svelte';

	let loading = $state(false);
	let error = $state<string | null>(null);

	onMount(() => {
		if (authStore.isAuthenticated) {
			goto('/');
		}
	});

	async function handleGoogleSignIn() {
		loading = true;
		error = null;
		try {
			await authStore.signInWithGoogle();
		} catch (err: any) {
			error = err.message || 'Failed to sign in with Google';
			loading = false;
		}
	}

	async function handleFacebookSignIn() {
		loading = true;
		error = null;
		try {
			await authStore.signInWithFacebook();
		} catch (err: any) {
			error = err.message || 'Failed to sign in with Facebook';
			loading = false;
		}
	}

	async function handleAppleSignIn() {
		loading = true;
		error = null;
		try {
			await authStore.signInWithApple();
		} catch (err: any) {
			error = err.message || 'Failed to sign in with Apple';
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Sign In - Loafy Club</title>
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
	<link href="https://fonts.googleapis.com/css2?family=Baloo+2:wght@400;500;600;700;800&display=swap" rel="stylesheet">
</svelte:head>

<PageBackground>
	<div class="flex min-h-screen items-center justify-center px-4 py-12">
		<div class="w-full max-w-md">
			<!-- Logo -->
			<AnimatedContainer animation="fade-up" class="mb-8 flex justify-center">
				<Logo size="xl" showRing />
			</AnimatedContainer>

			<!-- Card -->
			<AnimatedContainer animation="fade-up" delay={200}>
				<GlassCard>
					<div class="text-center mb-6">
						<h1
							class="text-2xl font-bold text-gray-800 mb-2"
							style="font-family: 'Baloo 2', sans-serif;"
						>
							Welcome back
						</h1>
						<p class="text-gray-600">Sign in to book your pickleball sessions</p>
					</div>

					{#if error}
						<div class="mb-6 rounded-xl bg-red-50 border border-red-200 p-4">
							<p class="text-sm text-red-600">{error}</p>
						</div>
					{/if}

					<div class="space-y-3">
						<button
							class="w-full flex items-center justify-center gap-3 px-6 py-4 rounded-xl border-2 border-gray-200 bg-white hover:bg-gray-50 hover:border-gray-300 hover:shadow-md transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
							disabled={loading}
							onclick={handleGoogleSignIn}
						>
							<svg class="h-5 w-5" viewBox="0 0 24 24">
								<path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
								<path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
								<path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
								<path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
							</svg>
							<span class="font-medium text-gray-700">Continue with Google</span>
						</button>

						<button
							class="w-full flex items-center justify-center gap-3 px-6 py-4 rounded-xl border-2 border-gray-200 bg-white hover:bg-gray-50 hover:border-gray-300 hover:shadow-md transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
							disabled={loading}
							onclick={handleFacebookSignIn}
						>
							<svg class="h-5 w-5" fill="#1877F2" viewBox="0 0 24 24">
								<path d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.47h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.47h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"/>
							</svg>
							<span class="font-medium text-gray-700">Continue with Facebook</span>
						</button>

						<button
							class="w-full flex items-center justify-center gap-3 px-6 py-4 rounded-xl border-2 border-gray-200 bg-white hover:bg-gray-50 hover:border-gray-300 hover:shadow-md transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
							disabled={loading}
							onclick={handleAppleSignIn}
						>
							<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
								<path d="M17.05 20.28c-.98.95-2.05.8-3.08.35-1.09-.46-2.09-.48-3.24 0-1.44.62-2.2.44-3.06-.35C2.79 15.25 3.51 7.59 9.05 7.31c1.35.07 2.29.74 3.08.8 1.18-.24 2.31-.93 3.57-.84 1.51.12 2.65.72 3.4 1.8-3.12 1.87-2.38 5.98.48 7.13-.57 1.5-1.31 2.99-2.54 4.09l.01-.01zM12.03 7.25c-.15-2.23 1.66-4.07 3.74-4.25.29 2.58-2.34 4.5-3.74 4.25z"/>
							</svg>
							<span class="font-medium text-gray-700">Continue with Apple</span>
						</button>
					</div>

					<!-- Divider -->
					<div class="relative my-6">
						<div class="absolute inset-0 flex items-center">
							<span class="w-full border-t border-gray-200"></span>
						</div>
						<div class="relative flex justify-center text-xs uppercase">
							<span class="bg-white/80 px-3 text-gray-400">Or</span>
						</div>
					</div>

					<Button
						variant="ghost"
						class="w-full text-gray-600 hover:text-gray-800 hover:bg-gray-100/50"
						onclick={() => goto('/sessions')}
					>
						Browse sessions without signing in
					</Button>
				</GlassCard>
			</AnimatedContainer>

			<!-- Footer -->
			<AnimatedContainer animation="fade-up" delay={400}>
				<p class="mt-8 text-center text-sm text-gray-500">
					By signing in, you agree to our
					<a href="/terms" class="text-orange-500 hover:text-orange-600 hover:underline transition-colors">Terms of Service</a>
					and
					<a href="/privacy" class="text-orange-500 hover:text-orange-600 hover:underline transition-colors">Privacy Policy</a>
				</p>

				<div class="mt-4 text-center">
					<a
						href="/"
						class="inline-flex items-center gap-2 text-sm text-gray-500 hover:text-gray-700 transition-colors"
					>
						<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
						</svg>
						Back to home
					</a>
				</div>
			</AnimatedContainer>
		</div>
	</div>
</PageBackground>

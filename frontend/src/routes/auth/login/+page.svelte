<script lang="ts">
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { onMount } from 'svelte';

	let loading = $state(false);
	let error = $state<string | null>(null);

	onMount(() => {
		// Redirect if already authenticated
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
	<title>Login - Loafy Club</title>
</svelte:head>

<div class="flex min-h-screen items-center justify-center bg-loafy-gradient-hero px-4 py-12">
	<div class="w-full max-w-md">
		<!-- Logo/Brand -->
		<div class="mb-8 text-center">
			<a href="/" class="inline-flex flex-col items-center gap-4">
				<img
					src="/mascot.jpeg"
					alt="Loafy the Corgi"
					class="h-24 w-24 rounded-full border-4 border-white shadow-xl"
				/>
				<span class="text-4xl font-bold text-gradient-loafy">Loafy Club</span>
			</a>
		</div>

		<Card.Root class="shadow-lg">
			<Card.Header class="space-y-1 text-center">
				<Card.Title class="text-2xl">Welcome back</Card.Title>
				<Card.Description>Sign in to book your pickleball sessions</Card.Description>
			</Card.Header>

			<Card.Content class="space-y-4">
				{#if error}
					<div class="rounded-lg bg-destructive/10 border border-destructive/20 p-4">
						<p class="text-sm text-destructive">{error}</p>
					</div>
				{/if}

				<div class="space-y-3">
					<Button
						variant="outline"
						size="lg"
						class="w-full relative"
						disabled={loading}
						onclick={handleGoogleSignIn}
					>
						<svg class="mr-3 h-5 w-5" viewBox="0 0 24 24">
							<path
								fill="#4285F4"
								d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
							/>
							<path
								fill="#34A853"
								d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
							/>
							<path
								fill="#FBBC05"
								d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
							/>
							<path
								fill="#EA4335"
								d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
							/>
						</svg>
						Continue with Google
					</Button>

					<Button
						variant="outline"
						size="lg"
						class="w-full"
						disabled={loading}
						onclick={handleFacebookSignIn}
					>
						<svg class="mr-3 h-5 w-5" fill="#1877F2" viewBox="0 0 24 24">
							<path
								d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.47h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.47h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"
							/>
						</svg>
						Continue with Facebook
					</Button>

					<Button
						variant="outline"
						size="lg"
						class="w-full"
						disabled={loading}
						onclick={handleAppleSignIn}
					>
						<svg class="mr-3 h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
							<path
								d="M17.05 20.28c-.98.95-2.05.8-3.08.35-1.09-.46-2.09-.48-3.24 0-1.44.62-2.2.44-3.06-.35C2.79 15.25 3.51 7.59 9.05 7.31c1.35.07 2.29.74 3.08.8 1.18-.24 2.31-.93 3.57-.84 1.51.12 2.65.72 3.4 1.8-3.12 1.87-2.38 5.98.48 7.13-.57 1.5-1.31 2.99-2.54 4.09l.01-.01zM12.03 7.25c-.15-2.23 1.66-4.07 3.74-4.25.29 2.58-2.34 4.5-3.74 4.25z"
							/>
						</svg>
						Continue with Apple
					</Button>
				</div>
			</Card.Content>

			<Card.Footer class="flex flex-col gap-4">
				<div class="relative w-full">
					<div class="absolute inset-0 flex items-center">
						<span class="w-full border-t"></span>
					</div>
					<div class="relative flex justify-center text-xs uppercase">
						<span class="bg-background px-2 text-muted-foreground">Or</span>
					</div>
				</div>

				<Button variant="ghost" class="w-full" onclick={() => goto('/sessions')}>
					Browse sessions without signing in
				</Button>
			</Card.Footer>
		</Card.Root>

		<p class="mt-6 text-center text-sm text-muted-foreground">
			By signing in, you agree to our{' '}
			<a href="/terms" class="text-primary hover:underline">Terms of Service</a>
			{' '}and{' '}
			<a href="/privacy" class="text-primary hover:underline">Privacy Policy</a>
		</p>
	</div>
</div>

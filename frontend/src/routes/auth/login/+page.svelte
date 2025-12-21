<script lang="ts">
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { extractErrorMessage } from '$lib/utils';
	import { Button } from '$lib/components/ui/button';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { Logo } from '$lib/components/ui/logo';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { ArrowLeft } from 'lucide-svelte';
	import { siGoogle, siFacebook } from 'simple-icons';
	import { onMount } from 'svelte';

	const t = useTranslation();

	let loading = $state(false);
	let error = $state<string | null>(null);
	let redirecting = $state(false);

	// Check auth state after mount and redirect if authenticated
	onMount(() => {
		// Wait for auth store to initialize before checking
		const checkAuth = () => {
			// Don't redirect if still loading or already redirecting
			if (authStore.loading || redirecting) return;

			if (authStore.isAuthenticated) {
				console.log('Login page: User already authenticated, redirecting to home');
				redirecting = true;
				goto('/');
			}
		};

		// Check after auth has had time to initialize
		const timeout = setTimeout(checkAuth, 300);
		// Also check after a longer delay in case of slow initialization
		const timeout2 = setTimeout(checkAuth, 1000);

		return () => {
			clearTimeout(timeout);
			clearTimeout(timeout2);
		};
	});

	async function handleOAuthSignIn(
		provider: 'google' | 'facebook',
		signInFn: () => Promise<void>
	) {
		loading = true;
		error = null;
		try {
			await signInFn();
		} catch (err: unknown) {
			error = extractErrorMessage(err, `Failed to sign in with ${provider}`);
			loading = false;
		}
	}

	const handleGoogleSignIn = () => handleOAuthSignIn('google', authStore.signInWithGoogle.bind(authStore));
	const handleFacebookSignIn = () => handleOAuthSignIn('facebook', authStore.signInWithFacebook.bind(authStore));

</script>

<svelte:head>
	<title>Sign In - Loafy Club</title>
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
							class="text-2xl font-bold text-gray-800 mb-2 font-display"
						>
							{t('auth.login.title')}
						</h1>
						<p class="text-gray-600">{t('auth.login.subtitle')}</p>
					</div>

					{#if error}
						<div class="mb-6 rounded-xl bg-red-50 border border-red-200 p-4">
							<p class="text-sm text-red-600">{error}</p>
						</div>
					{/if}

					<div class="space-y-3">
						<Button
							variant="social"
							size="xl"
							disabled={loading}
							onclick={handleGoogleSignIn}
						>
							<svg class="h-5 w-5" viewBox="0 0 24 24" fill="#{siGoogle.hex}">
								{@html siGoogle.svg}
							</svg>
							<span class="font-medium text-gray-700">{t('auth.login.googleButton')}</span>
						</Button>

						<Button
							variant="social"
							size="xl"
							disabled={loading}
							onclick={handleFacebookSignIn}
						>
							<svg class="h-5 w-5" viewBox="0 0 24 24" fill="#{siFacebook.hex}">
								{@html siFacebook.svg}
							</svg>
							<span class="font-medium text-gray-700">{t('auth.login.facebookButton')}</span>
						</Button>

					</div>

					<!-- Divider -->
					<div class="relative my-6">
						<div class="absolute inset-0 flex items-center">
							<span class="w-full border-t border-gray-200"></span>
						</div>
						<div class="relative flex justify-center text-xs uppercase">
							<span class="bg-white/80 px-3 text-gray-400">{t('auth.login.or')}</span>
						</div>
					</div>

					<Button
						variant="ghost"
						class="w-full text-gray-600 hover:text-gray-800 hover:bg-gray-100/50"
						onclick={() => goto('/sessions')}
					>
						{t('auth.login.browseWithoutSignIn')}
					</Button>
				</GlassCard>
			</AnimatedContainer>

			<!-- Footer -->
			<AnimatedContainer animation="fade-up" delay={400}>
				<p class="mt-8 text-center text-sm text-gray-500">
					{t('auth.login.termsText')}
					<a href="/terms" class="text-orange-500 hover:text-orange-600 hover:underline transition-colors">{t('auth.login.termsLink')}</a>
					{t('auth.login.and')}
					<a href="/privacy" class="text-orange-500 hover:text-orange-600 hover:underline transition-colors">{t('auth.login.privacyLink')}</a>
				</p>

				<div class="mt-4 text-center">
					<a
						href="/"
						class="inline-flex items-center gap-2 text-sm text-gray-500 hover:text-gray-700 transition-colors"
					>
						<ArrowLeft class="w-4 h-4" />
						{t('auth.login.backToHome')}
					</a>
				</div>
			</AnimatedContainer>
		</div>
	</div>
</PageBackground>

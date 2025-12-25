<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import Navigation from '$lib/components/Navigation.svelte';
	import PageBackground from '$lib/components/ui/page-background/page-background.svelte';
	import { Card } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { Home, ArrowLeft, Calendar } from 'lucide-svelte';
	import { onMount } from 'svelte';

	const t = useTranslation();

	let loaded = $state(false);

	onMount(() => {
		setTimeout(() => {
			loaded = true;
		}, 100);
	});

	const statusCode = $derived($page.status);
	const errorMessage = $derived($page.error?.message || '');

	const is404 = $derived(statusCode === 404);
</script>

<svelte:head>
	<title>{is404 ? t('error.pageNotFound') : t('error.somethingWentWrong')} - Loafy Club</title>
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<main class="min-h-[calc(100vh-64px-80px)] flex items-center justify-center px-4 py-4 sm:px-6 lg:px-8">
		<Card
			variant="glass"
			class="max-w-xl w-full opacity-0 {loaded ? 'animate-fade-in-up' : ''}"
			style="animation-fill-mode: forwards;"
		>
			<div class="text-center">
				<!-- Error Code -->
				<div class="mb-4">
					<h1 class="text-8xl sm:text-9xl font-extrabold leading-none font-display text-gradient-loafy select-none">
						{statusCode}
					</h1>
				</div>

				<!-- Mascot -->
				<div class="mb-6">
					<img
						src="/mascot.png"
						alt="Loafy the Corgi"
						class="w-24 sm:w-28 mx-auto drop-shadow-lg animate-float"
					/>
				</div>

				<!-- Error Message -->
				<div class="space-y-3 mb-8">
					<h2 class="text-xl sm:text-2xl font-bold text-heading font-display">
						{#if is404}
							{t('error.pageNotFoundTitle')}
						{:else}
							{t('error.somethingWentWrongTitle')}
						{/if}
					</h2>
					<p class="text-body">
						{#if is404}
							{t('error.pageNotFoundDesc')}
						{:else if errorMessage}
							{errorMessage}
						{:else}
							{t('error.somethingWentWrongDesc')}
						{/if}
					</p>
				</div>

				<!-- Action Buttons -->
				<div class="flex flex-col sm:flex-row items-center justify-center gap-3 mb-6">
					<Button
						variant="gradient"
						size="lg"
						class="w-full sm:w-auto shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-300"
						onclick={() => goto('/')}
					>
						<Home class="w-5 h-5 mr-2" />
						{t('error.goHome')}
					</Button>
					<Button
						variant="outline"
						size="lg"
						class="w-full sm:w-auto shadow hover:shadow-lg hover:scale-105 transition-all duration-300"
						onclick={() => goto('/sessions')}
					>
						<Calendar class="w-5 h-5 mr-2" />
						{t('error.browseSessions')}
					</Button>
				</div>

				<!-- Back link -->
				<button
					onclick={() => history.back()}
					class="inline-flex items-center text-muted-foreground hover:text-foreground transition-colors text-sm"
				>
					<ArrowLeft class="w-4 h-4 mr-1" />
					{t('error.goBack')}
				</button>
			</div>
		</Card>
	</main>
</PageBackground>

<style>
	/* Gradient text for the error code */
	.text-gradient-loafy {
		background: linear-gradient(135deg, #FFD060 0%, #FF8C42 50%, #FF6B9D 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	:global(.dark) .text-gradient-loafy {
		background: linear-gradient(135deg, #FFD060 0%, #FF9F5A 50%, #FF85A8 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	/* Fade-in animation */
	@keyframes fade-in-up {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	:global(.animate-fade-in-up) {
		animation: fade-in-up 0.5s ease-out forwards;
	}

	/* Floating mascot animation */
	@keyframes float {
		0%, 100% { transform: translateY(0); }
		50% { transform: translateY(-10px); }
	}

	:global(.animate-float) {
		animation: float 4s ease-in-out infinite;
	}
</style>

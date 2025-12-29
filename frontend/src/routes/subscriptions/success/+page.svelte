<script lang="ts">
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import Navigation from '$lib/components/Navigation.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { CheckCircle, Ticket, Calendar, ArrowRight } from 'lucide-svelte';

	const t = useTranslation();

	onMount(() => {
		// Refresh ticket balance after successful purchase
		authStore.refreshTicketBalance();
	});
</script>

<svelte:head>
	<title>{t('subscriptions.successTitle')} | Loafy Club</title>
</svelte:head>

<div class="flex min-h-screen flex-col bg-background">
<Navigation />

<main class="flex-1">
	<div class="mx-auto max-w-lg px-4 py-16 sm:px-6 lg:px-8">
		<Card.Root class="overflow-hidden text-center">
			<div class="bg-gradient-to-r from-green-500 to-emerald-500 p-8 text-white">
				<div class="mx-auto mb-4 flex size-16 items-center justify-center rounded-full bg-white/20">
					<CheckCircle class="size-10" />
				</div>
				<h1 class="text-2xl font-bold">{t('subscriptions.successTitle')}</h1>
				<p class="mt-2 text-green-100">{t('subscriptions.successMessage')}</p>
			</div>

			<Card.Content class="p-6">
				<div class="mb-6 space-y-4">
					<div class="flex items-center justify-center gap-3 text-lg">
						<Ticket class="size-6 text-orange-500" />
						<span class="font-medium">{t('subscriptions.ticketsAdded', { count: 10 })}</span>
					</div>
					<div class="flex items-center justify-center gap-3 text-muted-foreground">
						<Calendar class="size-5" />
						<span>{t('subscriptions.validFor3Months')}</span>
					</div>
				</div>

				<div class="space-y-3">
					<Button href="/sessions" class="w-full" size="lg">
						{t('subscriptions.bookSession')}
						<ArrowRight class="ml-2 size-4" />
					</Button>
					<Button href="/subscriptions" variant="outline" class="w-full">
						{t('subscriptions.viewSubscription')}
					</Button>
				</div>
			</Card.Content>
		</Card.Root>
	</div>
</main>

<Footer />
</div>

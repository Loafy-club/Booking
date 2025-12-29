<script lang="ts">
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import Navigation from '$lib/components/Navigation.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { XCircle, ArrowLeft, Ticket } from 'lucide-svelte';

	const t = useTranslation();
</script>

<svelte:head>
	<title>{t('subscriptions.cancelledTitle')} | Loafy Club</title>
</svelte:head>

<div class="flex min-h-screen flex-col bg-background">
<Navigation />

<main class="flex-1">
	<div class="mx-auto max-w-lg px-4 py-16 sm:px-6 lg:px-8">
		<Card.Root class="overflow-hidden text-center">
			<div class="bg-gradient-to-r from-gray-500 to-slate-500 p-8 text-white">
				<div class="mx-auto mb-4 flex size-16 items-center justify-center rounded-full bg-white/20">
					<XCircle class="size-10" />
				</div>
				<h1 class="text-2xl font-bold">{t('subscriptions.cancelledTitle')}</h1>
				<p class="mt-2 text-gray-200">{t('subscriptions.cancelledMessage')}</p>
			</div>

			<Card.Content class="p-6">
				<div class="mb-6 rounded-lg bg-muted p-4">
					<div class="flex items-center justify-center gap-2 text-muted-foreground">
						<Ticket class="size-5" />
						<span>{t('subscriptions.benefitsReminder')}</span>
					</div>
				</div>

				<div class="space-y-3">
					<Button href="/subscriptions" class="w-full" size="lg">
						<ArrowLeft class="mr-2 size-4" />
						{t('subscriptions.tryAgain')}
					</Button>
					<Button href="/sessions" variant="outline" class="w-full">
						{t('subscriptions.browseSessions')}
					</Button>
				</div>
			</Card.Content>
		</Card.Root>
	</div>
</main>

<Footer />
</div>

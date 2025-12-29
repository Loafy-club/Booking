<script lang="ts">
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { api } from '$lib/api/client';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Alert from '$lib/components/ui/alert';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Badge } from '$lib/components/ui/badge';
	import Navigation from '$lib/components/Navigation.svelte';
	import {
		Ticket,
		CreditCard,
		CalendarClock,
		Check,
		X,
		RefreshCw,
		Sparkles,
		Star,
		Zap,
		Gift,
		AlertCircle,
		Loader2,
		Clock,
		Users,
		Percent,
		Cake,
		UserPlus
	} from 'lucide-svelte';
	import type { SubscriptionDetailResponse } from '$lib/types';
	import Footer from '$lib/components/Footer.svelte';

	const t = useTranslation();

	let subscription = $state<SubscriptionDetailResponse | null>(null);
	let loading = $state(true);
	let actionLoading = $state(false);
	let error = $state<string | null>(null);
	let cancelDialogOpen = $state(false);

	async function loadSubscription() {
		loading = true;
		error = null;
		try {
			const response = await api.subscriptions.getCurrent();
			subscription = response.data;
		} catch (err) {
			console.error('Failed to load subscription:', err);
			error = t('subscriptions.loadError');
		} finally {
			loading = false;
		}
	}

	async function handlePurchase() {
		actionLoading = true;
		error = null;
		try {
			const response = await api.subscriptions.purchase();
			// Redirect to Stripe Checkout
			window.location.href = response.data.checkout_url;
		} catch (err: any) {
			console.error('Failed to create checkout:', err);
			error = err.response?.data?.message || t('subscriptions.purchaseError');
			actionLoading = false;
		}
	}

	function handleCancel() {
		cancelDialogOpen = true;
	}

	async function confirmCancel() {
		actionLoading = true;
		error = null;
		try {
			const response = await api.subscriptions.cancel();
			subscription = response.data;
			cancelDialogOpen = false;
			await authStore.refreshTicketBalance();
		} catch (err: any) {
			console.error('Failed to cancel subscription:', err);
			error = err.response?.data?.message || t('subscriptions.cancelError');
		} finally {
			actionLoading = false;
		}
	}

	async function handleResume() {
		actionLoading = true;
		error = null;
		try {
			const response = await api.subscriptions.resume();
			subscription = response.data;
			await authStore.refreshTicketBalance();
		} catch (err: any) {
			console.error('Failed to resume subscription:', err);
			error = err.response?.data?.message || t('subscriptions.resumeError');
		} finally {
			actionLoading = false;
		}
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return '-';
		return new Date(dateStr).toLocaleDateString(undefined, {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	function getStatusBadge(status: string, cancelAtPeriodEnd: boolean) {
		if (cancelAtPeriodEnd) {
			return { variant: 'outline' as const, text: t('subscriptions.status.cancelling') };
		}
		switch (status) {
			case 'active':
				return { variant: 'default' as const, text: t('subscriptions.status.active') };
			case 'expired':
				return { variant: 'secondary' as const, text: t('subscriptions.status.expired') };
			case 'cancelled':
				return { variant: 'destructive' as const, text: t('subscriptions.status.cancelled') };
			case 'past_due':
				return { variant: 'destructive' as const, text: t('subscriptions.status.pastDue') };
			default:
				return { variant: 'secondary' as const, text: status };
		}
	}

	onMount(() => {
		loadSubscription();
	});
</script>

<svelte:head>
	<title>{t('subscriptions.pageTitle')} | Loafy Club</title>
</svelte:head>

<div class="flex min-h-screen flex-col bg-background">
<Navigation />

<main class="flex-1">
	<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
		<!-- Header -->
		<div class="mb-8 text-center">
			<h1 class="text-3xl font-bold tracking-tight">{t('subscriptions.title')}</h1>
			<p class="mt-2 text-muted-foreground">{t('subscriptions.description')}</p>
		</div>

		{#if error}
			<Alert.Root variant="destructive" class="mb-6">
				<AlertCircle class="size-4" />
				<Alert.Title>{t('common.error')}</Alert.Title>
				<Alert.Description>{error}</Alert.Description>
			</Alert.Root>
		{/if}

		{#if loading}
			<div class="flex items-center justify-center py-12">
				<RefreshCw class="size-8 animate-spin text-muted-foreground" />
			</div>
		{:else if subscription && subscription.status === 'active'}
			<!-- Active Subscription Card -->
			{@const badge = getStatusBadge(subscription.status, subscription.cancel_at_period_end)}
			<Card.Root class="mb-8 overflow-hidden">
				<div class="bg-gradient-to-r from-orange-500 to-amber-500 p-6 text-white">
					<div class="flex items-center justify-between">
						<div>
							<div class="flex items-center gap-2">
								<p class="text-sm font-medium text-orange-100">{t('subscriptions.currentPlan')}</p>
								<Badge variant={badge.variant} class="bg-white/20 text-white hover:bg-white/30">
									{badge.text}
								</Badge>
							</div>
							<div class="mt-2 flex items-baseline gap-2">
								<span class="text-5xl font-bold">{subscription.tickets_remaining}</span>
								<span class="text-xl text-orange-100">{t('tickets.ticketsLabel')}</span>
							</div>
						</div>
						<div class="rounded-full bg-white/20 p-4">
							<Ticket class="size-12" />
						</div>
					</div>
					<div class="mt-4 flex items-center gap-2 text-sm text-orange-100">
						<CalendarClock class="size-4" />
						<span>
							{subscription.cancel_at_period_end
								? t('subscriptions.expiresOn')
								: t('subscriptions.renewsOn')}:
							{formatDate(subscription.current_period_end)}
						</span>
					</div>
				</div>
				<Card.Content class="p-6">
					{#if subscription.cancel_at_period_end}
						<div class="mb-4 rounded-lg bg-yellow-50 p-4 dark:bg-yellow-950">
							<p class="text-sm text-yellow-800 dark:text-yellow-200">
								{t('subscriptions.cancelledNote')}
							</p>
						</div>
						<Button onclick={handleResume} disabled={actionLoading} class="w-full">
							{#if actionLoading}
								<Loader2 class="mr-2 size-4 animate-spin" />
							{:else}
								<RefreshCw class="mr-2 size-4" />
							{/if}
							{t('subscriptions.resumeAutoRenew')}
						</Button>
					{:else}
						<Button variant="outline" onclick={handleCancel} disabled={actionLoading} class="w-full">
							{#if actionLoading}
								<Loader2 class="mr-2 size-4 animate-spin" />
							{:else}
								<X class="mr-2 size-4" />
							{/if}
							{t('subscriptions.cancelAutoRenew')}
						</Button>
					{/if}
				</Card.Content>
			</Card.Root>
		{:else}
			<!-- No Active Subscription - Purchase Card -->
			<Card.Root class="mb-8 overflow-hidden">
				<Card.Header class="bg-gradient-to-r from-orange-500 to-amber-500 text-white">
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="text-2xl">{t('subscriptions.packTitle')}</Card.Title>
							<Card.Description class="text-orange-100">
								{t('subscriptions.packDescription')}
							</Card.Description>
						</div>
						<div class="rounded-full bg-white/20 p-3">
							<Sparkles class="size-8" />
						</div>
					</div>
				</Card.Header>
				<Card.Content class="p-6">
					<div class="mb-6 text-center">
						<div class="flex items-baseline justify-center gap-1">
							<span class="text-4xl font-bold">800,000</span>
							<span class="text-xl text-muted-foreground">VND</span>
						</div>
						<p class="mt-1 text-sm text-muted-foreground">
							~$32 USD | {t('subscriptions.validity')}
						</p>
					</div>

					<!-- Benefits List -->
					<div class="mb-6 space-y-3">
						<div class="flex items-center gap-3">
							<div class="rounded-full bg-primary/10 p-1.5">
								<Ticket class="size-4 text-primary" />
							</div>
							<span>{t('subscriptions.benefits.tickets')}</span>
						</div>
						<div class="flex items-center gap-3">
							<div class="rounded-full bg-primary/10 p-1.5">
								<Percent class="size-4 text-primary" />
							</div>
							<span>{t('subscriptions.benefits.discount')}</span>
						</div>
						<div class="flex items-center gap-3">
							<div class="rounded-full bg-primary/10 p-1.5">
								<Star class="size-4 text-primary" />
							</div>
							<span>{t('subscriptions.benefits.earlyAccess')}</span>
						</div>
						<div class="flex items-center gap-3">
							<div class="rounded-full bg-primary/10 p-1.5">
								<Zap class="size-4 text-primary" />
							</div>
							<span>{t('subscriptions.benefits.priority')}</span>
						</div>
						<div class="flex items-center gap-3">
							<div class="rounded-full bg-primary/10 p-1.5">
								<Clock class="size-4 text-primary" />
							</div>
							<span>{t('subscriptions.benefits.cancellation')}</span>
						</div>
						<div class="flex items-center gap-3">
							<div class="rounded-full bg-primary/10 p-1.5">
								<RefreshCw class="size-4 text-primary" />
							</div>
							<span>{t('subscriptions.benefits.outOfTicket')}</span>
						</div>
						<div class="flex items-center gap-3">
							<div class="rounded-full bg-primary/10 p-1.5">
								<UserPlus class="size-4 text-primary" />
							</div>
							<span>{t('subscriptions.benefits.referral')}</span>
						</div>
						<div class="flex items-center gap-3">
							<div class="rounded-full bg-primary/10 p-1.5">
								<Cake class="size-4 text-primary" />
							</div>
							<span>{t('subscriptions.benefits.birthdayBonus')}</span>
						</div>
					</div>

					<!-- Guest price note -->
					<div class="mb-6 flex items-center gap-2 rounded-lg bg-muted/50 px-3 py-2 text-sm text-muted-foreground">
						<Users class="size-4 shrink-0" />
						<span>{t('subscriptions.guestPrice')}</span>
					</div>

					<Button
						onclick={handlePurchase}
						disabled={actionLoading}
						class="w-full bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-600 hover:to-amber-600"
						size="lg"
					>
						{#if actionLoading}
							<Loader2 class="mr-2 size-4 animate-spin" />
						{:else}
							<CreditCard class="mr-2 size-4" />
						{/if}
						{t('subscriptions.purchase')}
					</Button>

					<p class="mt-4 text-center text-xs text-muted-foreground">
						{t('subscriptions.autoRenewNote')}
					</p>
				</Card.Content>
			</Card.Root>

			<!-- Comparison Table -->
			<Card.Root class="mb-8">
				<Card.Header>
					<Card.Title class="text-lg">{t('subscriptions.comparisonTitle')}</Card.Title>
				</Card.Header>
				<Card.Content class="p-0">
					<div class="overflow-x-auto">
						<table class="w-full text-sm">
							<thead>
								<tr class="border-b bg-muted/50">
									<th class="px-4 py-3 text-left font-medium"></th>
									<th class="px-4 py-3 text-center font-semibold text-primary">Subscriber</th>
									<th class="px-4 py-3 text-center font-medium text-muted-foreground">Drop-in</th>
								</tr>
							</thead>
							<tbody class="divide-y">
								<tr>
									<td class="px-4 py-3 font-medium">{t('subscriptions.comparison.pricePerSession')}</td>
									<td class="px-4 py-3 text-center">
										<span class="font-semibold text-primary">{t('subscriptions.comparison.subscriberPrice')}</span>
										<Badge variant="secondary" class="ml-1 text-xs">{t('subscriptions.comparison.savings')}</Badge>
									</td>
									<td class="px-4 py-3 text-center text-muted-foreground">{t('subscriptions.comparison.dropInPrice')}</td>
								</tr>
								<tr>
									<td class="px-4 py-3 font-medium">{t('subscriptions.comparison.earlyAccess')}</td>
									<td class="px-4 py-3 text-center font-semibold text-primary">{t('subscriptions.comparison.subscriberEarlyAccess')}</td>
									<td class="px-4 py-3 text-center text-muted-foreground">{t('subscriptions.comparison.dropInEarlyAccess')}</td>
								</tr>
								<tr>
									<td class="px-4 py-3 font-medium">{t('subscriptions.comparison.waitlistPriority')}</td>
									<td class="px-4 py-3 text-center font-semibold text-primary">{t('subscriptions.comparison.subscriberWaitlist')}</td>
									<td class="px-4 py-3 text-center text-muted-foreground">{t('subscriptions.comparison.dropInWaitlist')}</td>
								</tr>
								<tr>
									<td class="px-4 py-3 font-medium">{t('subscriptions.comparison.cancellation')}</td>
									<td class="px-4 py-3 text-center font-semibold text-primary">{t('subscriptions.comparison.subscriberCancellation')}</td>
									<td class="px-4 py-3 text-center text-muted-foreground">{t('subscriptions.comparison.dropInCancellation')}</td>
								</tr>
								<tr>
									<td class="px-4 py-3 font-medium">{t('subscriptions.comparison.outOfTickets')}</td>
									<td class="px-4 py-3 text-center font-semibold text-primary">{t('subscriptions.comparison.subscriberOutOfTickets')}</td>
									<td class="px-4 py-3 text-center text-muted-foreground">{t('subscriptions.comparison.dropInOutOfTickets')}</td>
								</tr>
								<tr>
									<td class="px-4 py-3 font-medium">{t('subscriptions.comparison.birthdayBonus')}</td>
									<td class="px-4 py-3 text-center font-semibold text-primary">{t('subscriptions.comparison.subscriberBirthday')}</td>
									<td class="px-4 py-3 text-center text-muted-foreground">{t('subscriptions.comparison.dropInBirthday')}</td>
								</tr>
								<tr>
									<td class="px-4 py-3 font-medium">{t('subscriptions.comparison.referralBonus')}</td>
									<td class="px-4 py-3 text-center font-semibold text-primary">{t('subscriptions.comparison.subscriberReferral')}</td>
									<td class="px-4 py-3 text-center text-muted-foreground">{t('subscriptions.comparison.dropInReferral')}</td>
								</tr>
							</tbody>
						</table>
					</div>
				</Card.Content>
			</Card.Root>

			<!-- Existing tickets info if any -->
			{#if subscription && subscription.tickets_remaining > 0}
				<Card.Root>
					<Card.Content class="flex items-center justify-between p-6">
						<div class="flex items-center gap-3">
							<Ticket class="size-6 text-orange-500" />
							<div>
								<p class="font-medium">{t('subscriptions.remainingTickets')}</p>
								<p class="text-sm text-muted-foreground">
									{t('subscriptions.ticketsFromPrevious')}
								</p>
							</div>
						</div>
						<span class="text-2xl font-bold">{subscription.tickets_remaining}</span>
					</Card.Content>
				</Card.Root>
			{/if}
		{/if}

		<!-- Link to ticket history -->
		<div class="mt-8 text-center">
			<a href="/tickets" class="text-sm text-muted-foreground hover:text-foreground">
				{t('subscriptions.viewTicketHistory')} &rarr;
			</a>
		</div>
	</div>
</main>

<Footer />
</div>

<!-- Cancel Subscription Dialog -->
<AlertDialog.Root bind:open={cancelDialogOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>{t('subscriptions.cancelDialogTitle')}</AlertDialog.Title>
			<AlertDialog.Description>
				{t('subscriptions.cancelDialogDescription')}
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={actionLoading}>
				{t('common.cancel')}
			</AlertDialog.Cancel>
			<Button
				variant="destructive"
				onclick={confirmCancel}
				disabled={actionLoading}
			>
				{#if actionLoading}
					<Loader2 class="mr-2 size-4 animate-spin" />
				{/if}
				{t('subscriptions.confirmCancelButton')}
			</Button>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

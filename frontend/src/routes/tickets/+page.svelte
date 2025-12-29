<script lang="ts">
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { api } from '$lib/api/client';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Badge } from '$lib/components/ui/badge';
	import Navigation from '$lib/components/Navigation.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { Ticket, History, CalendarClock, TrendingUp, TrendingDown, Gift, RefreshCw, ChevronLeft, ChevronRight } from 'lucide-svelte';
	import type { TicketTransactionResponse, PageInfo } from '$lib/types';

	const t = useTranslation();

	let transactions = $state<TicketTransactionResponse[]>([]);
	let pageInfo = $state<PageInfo | null>(null);
	let loading = $state(true);
	let currentPage = $state(1);
	const perPage = 10;

	async function loadTransactions(page: number = 1) {
		loading = true;
		try {
			const response = await api.subscriptions.getTicketHistory({ page, per_page: perPage });
			transactions = response.data.data;
			pageInfo = response.data.page_info;
			currentPage = page;
		} catch (error) {
			console.error('Failed to load ticket history:', error);
		} finally {
			loading = false;
		}
	}

	function getTransactionIcon(type: string) {
		switch (type) {
			case 'subscription_grant':
				return Gift;
			case 'used':
				return TrendingDown;
			case 'restored':
				return RefreshCw;
			case 'bonus_referral':
			case 'bonus_birthday':
			case 'bonus_manual':
				return Gift;
			case 'revoked':
				return TrendingDown;
			default:
				return Ticket;
		}
	}

	function getTransactionColor(amount: number): string {
		return amount > 0
			? 'text-green-600 dark:text-green-400'
			: 'text-red-600 dark:text-red-400';
	}

	function getTransactionBadgeClass(type: string): string {
		switch (type) {
			case 'subscription_grant':
			case 'bonus_referral':
			case 'restored':
				// Green for positive additions
				return 'bg-secondary text-green-600 hover:bg-secondary/80 dark:text-green-400';
			case 'bonus_manual':
				// Yellow for admin grants
				return 'bg-secondary text-yellow-600 hover:bg-secondary/80 dark:text-yellow-400';
			case 'bonus_birthday':
				// Purple for birthday bonus
				return 'bg-secondary text-purple-600 hover:bg-secondary/80 dark:text-purple-400';
			case 'used':
				// Gray for normal usage
				return 'bg-secondary text-secondary-foreground hover:bg-secondary/80';
			case 'revoked':
			case 'expired':
				// Red for negative events
				return 'bg-secondary text-red-600 hover:bg-secondary/80 dark:text-red-400';
			default:
				return 'border border-input bg-background hover:bg-accent hover:text-accent-foreground';
		}
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString(undefined, {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatPeriodEnd(dateStr: string | null): string {
		if (!dateStr) return t('tickets.noPeriodEnd');
		return new Date(dateStr).toLocaleDateString(undefined, {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	onMount(() => {
		loadTransactions();
	});
</script>

<svelte:head>
	<title>{t('tickets.pageTitle')} | Loafy Club</title>
</svelte:head>

<div class="flex min-h-screen flex-col bg-background">
<Navigation />

<main class="flex-1">
	<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
		<!-- Header -->
		<div class="mb-8">
			<h1 class="text-3xl font-bold tracking-tight">{t('tickets.title')}</h1>
			<p class="mt-2 text-muted-foreground">{t('tickets.description')}</p>
		</div>

		<!-- Balance Card -->
		<Card.Root class="mb-8 overflow-hidden">
			<div class="bg-gradient-to-r from-orange-500 to-amber-500 p-6 text-white">
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm font-medium text-orange-100">{t('tickets.currentBalance')}</p>
						<div class="mt-2 flex items-baseline gap-2">
							<span class="text-5xl font-bold">{authStore.ticketsRemaining}</span>
							<span class="text-xl text-orange-100">{t('tickets.ticketsLabel')}</span>
						</div>
					</div>
					<div class="rounded-full bg-white/20 p-4">
						<Ticket class="size-12" />
					</div>
				</div>
				{#if authStore.ticketBalance?.hasActiveSubscription}
					<div class="mt-4 flex items-center gap-2 text-sm text-orange-100">
						<CalendarClock class="size-4" />
						<span>{t('tickets.validUntil')}: {formatPeriodEnd(authStore.ticketBalance.currentPeriodEnd)}</span>
					</div>
				{:else}
					<div class="mt-4 text-sm text-orange-100">
						{t('tickets.noActiveSubscription')}
					</div>
				{/if}
			</div>
		</Card.Root>

		<!-- Transaction History -->
		<Card.Root>
			<Card.Header class="flex flex-row items-center justify-between">
				<div>
					<Card.Title class="flex items-center gap-2">
						<History class="size-5" />
						{t('tickets.history')}
					</Card.Title>
					<Card.Description>{t('tickets.historyDescription')}</Card.Description>
				</div>
				<Button variant="outline" size="sm" onclick={() => loadTransactions(currentPage)}>
					<RefreshCw class="mr-2 size-4" />
					{t('common.refresh')}
				</Button>
			</Card.Header>
			<Card.Content>
				{#if loading}
					<div class="flex items-center justify-center py-8">
						<RefreshCw class="size-6 animate-spin text-muted-foreground" />
					</div>
				{:else if transactions.length === 0}
					<div class="flex flex-col items-center justify-center py-12 text-center">
						<Ticket class="size-12 text-muted-foreground/50" />
						<p class="mt-4 text-muted-foreground">{t('tickets.noHistory')}</p>
					</div>
				{:else}
					<Table.Root>
						<Table.Header>
							<Table.Row>
								<Table.Head>{t('tickets.table.date')}</Table.Head>
								<Table.Head>{t('tickets.table.type')}</Table.Head>
								<Table.Head class="text-center">{t('tickets.table.amount')}</Table.Head>
								<Table.Head class="text-center">{t('tickets.table.balance')}</Table.Head>
								<Table.Head>{t('tickets.table.details')}</Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each transactions as tx}
								{@const IconComponent = getTransactionIcon(tx.transaction_type)}
								<Table.Row>
									<Table.Cell class="text-sm text-muted-foreground">
										{formatDate(tx.created_at)}
									</Table.Cell>
									<Table.Cell>
										<Badge class={getTransactionBadgeClass(tx.transaction_type)}>
											<svelte:component this={IconComponent} class="mr-1 size-3" />
											{t(`tickets.types.${tx.transaction_type}`)}
										</Badge>
									</Table.Cell>
									<Table.Cell class="text-center">
										<span class={`font-medium ${getTransactionColor(tx.amount)}`}>
											{tx.amount > 0 ? '+' : ''}{tx.amount}
										</span>
									</Table.Cell>
									<Table.Cell class="text-center font-medium">
										{tx.balance_after}
									</Table.Cell>
									<Table.Cell class="text-sm text-muted-foreground">
										{#if tx.booking_code}
											<a href="/bookings" class="text-primary hover:underline">
												{tx.booking_code}
											</a>
										{:else if tx.notes}
											{tx.notes}
										{:else}
											-
										{/if}
									</Table.Cell>
								</Table.Row>
							{/each}
						</Table.Body>
					</Table.Root>

					<!-- Pagination -->
					{#if pageInfo && pageInfo.total_pages > 1}
						<div class="mt-4 flex items-center justify-between border-t pt-4">
							<p class="text-sm text-muted-foreground">
								{t('common.pagination.showing', {
									from: (currentPage - 1) * perPage + 1,
									to: Math.min(currentPage * perPage, pageInfo.total),
									total: pageInfo.total
								})}
							</p>
							<div class="flex items-center gap-2">
								<Button
									variant="outline"
									size="sm"
									disabled={currentPage === 1}
									onclick={() => loadTransactions(currentPage - 1)}
								>
									<ChevronLeft class="size-4" />
								</Button>
								<span class="text-sm">
									{currentPage} / {pageInfo.total_pages}
								</span>
								<Button
									variant="outline"
									size="sm"
									disabled={currentPage >= pageInfo.total_pages}
									onclick={() => loadTransactions(currentPage + 1)}
								>
									<ChevronRight class="size-4" />
								</Button>
							</div>
						</div>
					{/if}
				{/if}
			</Card.Content>
		</Card.Root>
	</div>
</main>

<Footer />
</div>

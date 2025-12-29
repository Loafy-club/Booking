<script lang="ts">
	import { api } from '$lib/api/client';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Badge } from '$lib/components/ui/badge';
	import { Ticket, Plus, Minus, RefreshCw, Gift, TrendingDown, TrendingUp, AlertCircle } from 'lucide-svelte';
	import type { AdminUserTicketsResponse, TicketTransactionResponse } from '$lib/types';

	interface Props {
		userId: string;
		onUpdate?: () => void;
	}

	let { userId, onUpdate }: Props = $props();

	const t = useTranslation();

	let ticketData = $state<AdminUserTicketsResponse | null>(null);
	let loading = $state(true);
	let actionLoading = $state(false);
	let error = $state<string | null>(null);

	// Grant/revoke form state
	let grantAmount = $state(1);
	let grantReason = $state('');
	let revokeAmount = $state(1);
	let revokeReason = $state('');

	async function loadTicketData() {
		loading = true;
		error = null;
		try {
			const response = await api.admin.getUserTickets(userId);
			ticketData = response.data;
		} catch (err) {
			console.error('Failed to load ticket data:', err);
			error = t('admin.tickets.loadError');
		} finally {
			loading = false;
		}
	}

	async function handleGrant() {
		if (grantAmount < 1 || grantAmount > 100) return;
		actionLoading = true;
		error = null;
		try {
			await api.admin.grantTickets(userId, {
				amount: grantAmount,
				reason: grantReason || undefined
			});
			grantAmount = 1;
			grantReason = '';
			await loadTicketData();
			onUpdate?.();
		} catch (err) {
			console.error('Failed to grant tickets:', err);
			error = t('admin.tickets.grantError');
		} finally {
			actionLoading = false;
		}
	}

	async function handleRevoke() {
		if (revokeAmount < 1 || revokeAmount > 100) return;
		actionLoading = true;
		error = null;
		try {
			await api.admin.revokeTickets(userId, {
				amount: revokeAmount,
				reason: revokeReason || undefined
			});
			revokeAmount = 1;
			revokeReason = '';
			await loadTicketData();
			onUpdate?.();
		} catch (err) {
			console.error('Failed to revoke tickets:', err);
			error = t('admin.tickets.revokeError');
		} finally {
			actionLoading = false;
		}
	}

	function getTransactionIcon(type: string) {
		switch (type) {
			case 'subscription_grant':
			case 'bonus_referral':
			case 'bonus_birthday':
			case 'bonus_manual':
				return Gift;
			case 'used':
			case 'revoked':
				return TrendingDown;
			case 'restored':
				return TrendingUp;
			default:
				return Ticket;
		}
	}

	function getTransactionColor(amount: number): string {
		return amount > 0
			? 'text-green-600 dark:text-green-400'
			: 'text-red-600 dark:text-red-400';
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString(undefined, {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	$effect(() => {
		if (userId) {
			loadTicketData();
		}
	});
</script>

<Card.Root>
	<Card.Header>
		<Card.Title class="flex items-center gap-2">
			<Ticket class="size-5" />
			{t('admin.tickets.title')}
		</Card.Title>
	</Card.Header>
	<Card.Content>
		{#if loading}
			<div class="flex items-center justify-center py-8">
				<RefreshCw class="size-6 animate-spin text-muted-foreground" />
			</div>
		{:else if error}
			<div class="flex items-center gap-2 rounded-lg border border-destructive/20 bg-destructive/10 p-4 text-destructive">
				<AlertCircle class="size-5" />
				<span>{error}</span>
			</div>
		{:else if ticketData}
			<!-- Balance Display -->
			<div class="mb-6 flex items-center gap-4 rounded-lg bg-gradient-to-r from-orange-50 to-amber-50 dark:from-orange-900/20 dark:to-amber-900/20 p-4">
				<div class="flex items-center justify-center size-12 rounded-full bg-orange-100 dark:bg-orange-900/50">
					<Ticket class="size-6 text-orange-600 dark:text-orange-400" />
				</div>
				<div>
					<p class="text-sm text-muted-foreground">{t('admin.tickets.currentBalance')}</p>
					<p class="text-3xl font-bold text-orange-700 dark:text-orange-300">{ticketData.tickets_remaining}</p>
				</div>
				<div class="ml-auto">
					{#if ticketData.has_active_subscription}
						<Badge variant="default">{t('admin.tickets.activeSubscription')}</Badge>
					{:else}
						<Badge variant="outline">{t('admin.tickets.noSubscription')}</Badge>
					{/if}
				</div>
			</div>

			<!-- Grant/Revoke Actions -->
			<div class="mb-6 grid gap-4 md:grid-cols-2">
				<!-- Grant Tickets -->
				<div class="rounded-lg border p-4">
					<h4 class="mb-3 flex items-center gap-2 font-medium text-green-600 dark:text-green-400">
						<Plus class="size-4" />
						{t('admin.tickets.grant')}
					</h4>
					<div class="space-y-3">
						<div>
							<Label for="grant-amount">{t('admin.tickets.amount')}</Label>
							<Input
								id="grant-amount"
								type="number"
								min="1"
								max="100"
								bind:value={grantAmount}
								class="mt-1"
							/>
						</div>
						<div>
							<Label for="grant-reason">{t('admin.tickets.reason')}</Label>
							<Textarea
								id="grant-reason"
								bind:value={grantReason}
								placeholder={t('admin.tickets.reasonPlaceholder')}
								rows={2}
								class="mt-1"
							/>
						</div>
						<Button
							onclick={handleGrant}
							disabled={actionLoading || grantAmount < 1}
							class="w-full bg-green-600 hover:bg-green-700"
						>
							{#if actionLoading}
								<RefreshCw class="mr-2 size-4 animate-spin" />
							{:else}
								<Plus class="mr-2 size-4" />
							{/if}
							{t('admin.tickets.grantButton', { amount: grantAmount })}
						</Button>
					</div>
				</div>

				<!-- Revoke Tickets -->
				<div class="rounded-lg border p-4">
					<h4 class="mb-3 flex items-center gap-2 font-medium text-red-600 dark:text-red-400">
						<Minus class="size-4" />
						{t('admin.tickets.revoke')}
					</h4>
					<div class="space-y-3">
						<div>
							<Label for="revoke-amount">{t('admin.tickets.amount')}</Label>
							<Input
								id="revoke-amount"
								type="number"
								min="1"
								max={ticketData.tickets_remaining}
								bind:value={revokeAmount}
								class="mt-1"
							/>
						</div>
						<div>
							<Label for="revoke-reason">{t('admin.tickets.reason')}</Label>
							<Textarea
								id="revoke-reason"
								bind:value={revokeReason}
								placeholder={t('admin.tickets.reasonPlaceholder')}
								rows={2}
								class="mt-1"
							/>
						</div>
						<Button
							onclick={handleRevoke}
							disabled={actionLoading || revokeAmount < 1 || revokeAmount > ticketData.tickets_remaining}
							variant="destructive"
							class="w-full"
						>
							{#if actionLoading}
								<RefreshCw class="mr-2 size-4 animate-spin" />
							{:else}
								<Minus class="mr-2 size-4" />
							{/if}
							{t('admin.tickets.revokeButton', { amount: revokeAmount })}
						</Button>
					</div>
				</div>
			</div>

			<!-- Recent Transactions -->
			{#if ticketData.recent_transactions.length > 0}
				<div>
					<h4 class="mb-3 font-medium">{t('admin.tickets.recentTransactions')}</h4>
					<Table.Root>
						<Table.Header>
							<Table.Row>
								<Table.Head>{t('tickets.table.date')}</Table.Head>
								<Table.Head>{t('tickets.table.type')}</Table.Head>
								<Table.Head class="text-center">{t('tickets.table.amount')}</Table.Head>
								<Table.Head>{t('tickets.table.details')}</Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each ticketData.recent_transactions as tx}
								{@const IconComponent = getTransactionIcon(tx.transaction_type)}
								<Table.Row>
									<Table.Cell class="text-sm text-muted-foreground">
										{formatDate(tx.created_at)}
									</Table.Cell>
									<Table.Cell>
										<span class="flex items-center gap-1 text-sm">
											<svelte:component this={IconComponent} class="size-3" />
											{t(`tickets.types.${tx.transaction_type}`)}
										</span>
									</Table.Cell>
									<Table.Cell class="text-center">
										<span class={`font-medium ${getTransactionColor(tx.amount)}`}>
											{tx.amount > 0 ? '+' : ''}{tx.amount}
										</span>
									</Table.Cell>
									<Table.Cell class="text-sm text-muted-foreground">
										{tx.booking_code || tx.notes || '-'}
									</Table.Cell>
								</Table.Row>
							{/each}
						</Table.Body>
					</Table.Root>
				</div>
			{/if}
		{/if}
	</Card.Content>
</Card.Root>

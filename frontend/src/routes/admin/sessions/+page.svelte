<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate, extractErrorMessage } from '$lib/utils';
	import { requireRole } from '$lib/guards/auth';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { SectionHeader } from '$lib/components/ui/section-header';
	import { LoadingSpinner } from '$lib/components/ui/loading-spinner';
	import { ErrorState } from '$lib/components/ui/error-state';
	import { EmptyState } from '$lib/components/ui/empty-state';
	import { StatusBadge } from '$lib/components/ui/status-badge';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import type { Session } from '$lib/types';

	const t = useTranslation();

	let sessions = $state<Session[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		if (!requireRole('admin')) return;
		await loadSessions();
	});

	async function loadSessions() {
		loading = true;
		error = null;

		try {
			const response = await api.sessions.list();
			sessions = response.data;
		} catch (err: any) {
			error = extractErrorMessage(err, 'Failed to load sessions');
		} finally {
			loading = false;
		}
	}

	async function toggleCancelled(sessionId: string, cancelled: boolean) {
		try {
			// Note: Backend would need to support this - for now just reload
			await loadSessions();
		} catch (err: any) {
			alert(extractErrorMessage(err, t('admin.sessions.updateFailed')));
		}
	}

	async function deleteSession(sessionId: string, title: string) {
		if (!confirm(t('admin.sessions.confirmDelete', { title }))) {
			return;
		}

		try {
			await api.sessions.delete(sessionId);
			await loadSessions();
		} catch (err: any) {
			alert(extractErrorMessage(err, t('admin.sessions.deleteFailed')));
		}
	}
</script>

<svelte:head>
	<title>{t('admin.sessions.pageTitle')} - Loafy Club</title>
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<AnimatedContainer animation="fade-up">
			<SectionHeader title={t('admin.sessions.title')} subtitle={t('admin.sessions.subtitle')}>
				{#snippet actions()}
					<Button
						variant="gradient"
						onclick={() => goto('/organizer/sessions/create')}
					>
						{t('admin.sessions.createSession')}
					</Button>
				{/snippet}
			</SectionHeader>
		</AnimatedContainer>

		{#if loading}
			<LoadingSpinner text={t('admin.sessions.loadingText')} />
		{:else if error}
			<ErrorState message={error} onRetry={loadSessions} />
		{:else if sessions.length === 0}
			<EmptyState
				title={t('admin.sessions.noSessions')}
				description={t('admin.sessions.noSessionsDesc')}
				actionText={t('admin.sessions.createSession')}
				onAction={() => goto('/organizer/sessions/create')}
			/>
		{:else}
			<AnimatedContainer animation="fade-up" delay={100}>
				<GlassCard class="overflow-hidden p-0">
					<div class="overflow-x-auto">
						<table class="min-w-full divide-y divide-gray-200">
							<thead class="bg-gray-50/80">
								<tr>
									<th class="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
										{t('admin.sessions.table.session')}
									</th>
									<th class="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
										{t('admin.sessions.table.dateTime')}
									</th>
									<th class="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
										{t('admin.sessions.table.slots')}
									</th>
									<th class="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
										{t('admin.sessions.table.price')}
									</th>
									<th class="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
										{t('admin.sessions.table.status')}
									</th>
									<th class="px-6 py-4 text-right text-xs font-medium uppercase tracking-wider text-gray-500">
										{t('admin.sessions.table.actions')}
									</th>
								</tr>
							</thead>
							<tbody class="divide-y divide-gray-200 bg-white/50">
								{#each sessions as session}
									<tr class="hover:bg-gray-50/50 transition-colors">
										<td class="whitespace-nowrap px-6 py-4">
											<div class="text-sm font-medium text-gray-800">{session.title}</div>
											<div class="text-sm text-gray-500">{session.location}</div>
										</td>
										<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-600">
											{session.date} {session.time}
										</td>
										<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-600">
											<span class={session.available_slots > 0 ? 'text-green-600' : 'text-red-600'}>
												{session.available_slots}
											</span>
											/ {session.total_slots}
										</td>
									<td class="whitespace-nowrap px-6 py-4 text-sm font-medium text-gradient-primary">
										{formatCurrency(session.price_vnd)}
									</td>
										<td class="whitespace-nowrap px-6 py-4">
											<span class={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${session.cancelled ? 'bg-red-100 text-red-800' : 'bg-green-100 text-green-800'}`}>
												{session.cancelled ? t('admin.sessions.status.cancelled') : t('admin.sessions.status.active')}
											</span>
										</td>
										<td class="whitespace-nowrap px-6 py-4 text-right text-sm font-medium">
											<div class="flex justify-end gap-2">
												<Button
													variant="ghost"
													size="sm"
													onclick={() => goto(`/sessions/${session.id}`)}
												>
													{t('admin.sessions.actions.view')}
												</Button>
												<Button
													variant="ghost"
													size="sm"
													onclick={() => deleteSession(session.id, session.title)}
													class="text-red-600 hover:text-red-700 hover:bg-red-50"
												>
													{t('admin.sessions.actions.delete')}
												</Button>
											</div>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</GlassCard>
			</AnimatedContainer>
		{/if}
	</div>
</PageBackground>

<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate } from '$lib/utils';
	import { requireRole } from '$lib/guards/auth';
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
			error = err.response?.data?.message || err.message || 'Failed to load sessions';
		} finally {
			loading = false;
		}
	}

	async function updateSessionStatus(sessionId: string, status: string) {
		try {
			await api.sessions.update(sessionId, { status });
			await loadSessions();
		} catch (err: any) {
			alert(err.response?.data?.message || err.message || 'Failed to update session');
		}
	}

	async function deleteSession(sessionId: string, title: string) {
		if (!confirm(`Are you sure you want to delete "${title}"?`)) {
			return;
		}

		try {
			await api.sessions.delete(sessionId);
			await loadSessions();
		} catch (err: any) {
			alert(err.response?.data?.message || err.message || 'Failed to delete session');
		}
	}
</script>

<svelte:head>
	<title>Admin - Sessions - Loafy Club</title>
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
	<link href="https://fonts.googleapis.com/css2?family=Baloo+2:wght@400;500;600;700;800&display=swap" rel="stylesheet">
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<AnimatedContainer animation="fade-up">
			<SectionHeader title="Manage Sessions" subtitle="View and manage all pickleball sessions">
				{#snippet actions()}
					<Button
						class="bg-gradient-to-r from-orange-500 to-pink-500 border-0"
						onclick={() => goto('/organizer/sessions/create')}
					>
						Create Session
					</Button>
				{/snippet}
			</SectionHeader>
		</AnimatedContainer>

		{#if loading}
			<LoadingSpinner text="Loading sessions..." />
		{:else if error}
			<ErrorState message={error} onRetry={loadSessions} />
		{:else if sessions.length === 0}
			<EmptyState
				title="No Sessions Found"
				description="Create your first pickleball session"
				actionText="Create Session"
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
										Session
									</th>
									<th class="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
										Date/Time
									</th>
									<th class="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
										Slots
									</th>
									<th class="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
										Price
									</th>
									<th class="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
										Status
									</th>
									<th class="px-6 py-4 text-right text-xs font-medium uppercase tracking-wider text-gray-500">
										Actions
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
											{formatDate(session.start_time)}
										</td>
										<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-600">
											<span class={session.available_slots > 0 ? 'text-green-600' : 'text-red-600'}>
												{session.available_slots}
											</span>
											/ {session.max_slots}
										</td>
										<td class="whitespace-nowrap px-6 py-4 text-sm font-medium text-transparent bg-clip-text bg-gradient-to-r from-orange-500 to-pink-500">
											{formatCurrency(session.price_vnd)}
										</td>
										<td class="whitespace-nowrap px-6 py-4">
											<select
												value={session.status}
												onchange={(e) => updateSessionStatus(session.id, e.currentTarget.value)}
												class="rounded-lg border-2 border-gray-200 px-3 py-1.5 text-xs font-medium focus:border-orange-400 focus:outline-none focus:ring-2 focus:ring-orange-200 transition-colors"
											>
												<option value="draft">Draft</option>
												<option value="published">Published</option>
												<option value="in_progress">In Progress</option>
												<option value="completed">Completed</option>
												<option value="cancelled">Cancelled</option>
											</select>
										</td>
										<td class="whitespace-nowrap px-6 py-4 text-right text-sm font-medium">
											<div class="flex justify-end gap-2">
												<Button
													variant="ghost"
													size="sm"
													onclick={() => goto(`/sessions/${session.id}`)}
												>
													View
												</Button>
												<Button
													variant="ghost"
													size="sm"
													onclick={() => deleteSession(session.id, session.title)}
													class="text-red-600 hover:text-red-700 hover:bg-red-50"
												>
													Delete
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

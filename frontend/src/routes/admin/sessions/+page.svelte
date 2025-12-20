<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, formatDate } from '$lib/utils';
	import { requireRole } from '$lib/guards/auth';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
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

	function getStatusColor(status: string): string {
		switch (status) {
			case 'draft':
				return 'bg-gray-100 text-gray-800';
			case 'published':
				return 'bg-green-100 text-green-800';
			case 'in_progress':
				return 'bg-blue-100 text-blue-800';
			case 'completed':
				return 'bg-purple-100 text-purple-800';
			case 'cancelled':
				return 'bg-red-100 text-red-800';
			default:
				return 'bg-gray-100 text-gray-800';
		}
	}
</script>

<svelte:head>
	<title>Admin - Sessions - Loafy Club</title>
</svelte:head>

<Navigation />

<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
	<div class="mb-8 flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-gray-900">Manage Sessions</h1>
			<p class="mt-2 text-gray-600">View and manage all pickleball sessions</p>
		</div>

		<Button onclick={() => goto('/organizer/sessions/create')}>Create Session</Button>
	</div>

	{#if loading}
		<div class="flex justify-center py-12">
			<div class="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
		</div>
	{:else if error}
		<div class="rounded-lg bg-destructive/10 p-6 text-center">
			<p class="text-destructive">{error}</p>
			<Button class="mt-4" onclick={loadSessions}>Try Again</Button>
		</div>
	{:else if sessions.length === 0}
		<div class="rounded-lg bg-gray-50 p-12 text-center">
			<p class="text-lg text-gray-600">No sessions found</p>
			<Button class="mt-4" onclick={() => goto('/organizer/sessions/create')}>
				Create First Session
			</Button>
		</div>
	{:else}
		<div class="overflow-hidden rounded-lg border bg-white shadow">
			<table class="min-w-full divide-y divide-gray-200">
				<thead class="bg-gray-50">
					<tr>
						<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
							Session
						</th>
						<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
							Date/Time
						</th>
						<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
							Slots
						</th>
						<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
							Price
						</th>
						<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
							Status
						</th>
						<th class="px-6 py-3 text-right text-xs font-medium uppercase tracking-wider text-gray-500">
							Actions
						</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-200 bg-white">
					{#each sessions as session}
						<tr>
							<td class="whitespace-nowrap px-6 py-4">
								<div class="text-sm font-medium text-gray-900">{session.title}</div>
								<div class="text-sm text-gray-500">{session.location}</div>
							</td>
							<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-500">
								{formatDate(session.start_time)}
							</td>
							<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-500">
								{session.available_slots} / {session.max_slots}
							</td>
							<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-500">
								{formatCurrency(session.price_vnd)}
							</td>
							<td class="whitespace-nowrap px-6 py-4">
								<select
									value={session.status}
									onchange={(e) => updateSessionStatus(session.id, e.currentTarget.value)}
									class={`rounded-full px-2 py-1 text-xs font-medium ${getStatusColor(session.status)}`}
								>
									<option value="draft">Draft</option>
									<option value="published">Published</option>
									<option value="in_progress">In Progress</option>
									<option value="completed">Completed</option>
									<option value="cancelled">Cancelled</option>
								</select>
							</td>
							<td class="whitespace-nowrap px-6 py-4 text-right text-sm font-medium">
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
									class="text-destructive hover:text-destructive"
								>
									Delete
								</Button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

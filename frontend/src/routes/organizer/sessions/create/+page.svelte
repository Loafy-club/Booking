<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { requireRole } from '$lib/guards/auth';
	import Navigation from '$lib/components/Navigation.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';

	let loading = $state(false);
	let error = $state<string | null>(null);

	let formData = $state({
		title: '',
		description: '',
		location: '',
		start_time: '',
		end_time: '',
		max_slots: 8,
		price_vnd: 100000,
		early_access_ends_at: ''
	});

	onMount(() => {
		if (!requireRole('organizer')) return;
	});

	async function handleSubmit(e: Event) {
		e.preventDefault();
		loading = true;
		error = null;

		try {
			// Validate dates
			const startTime = new Date(formData.start_time);
			const endTime = new Date(formData.end_time);

			if (startTime >= endTime) {
				throw new Error('End time must be after start time');
			}

			if (startTime < new Date()) {
				throw new Error('Start time must be in the future');
			}

			const payload: any = {
				title: formData.title,
				location: formData.location,
				start_time: formData.start_time,
				end_time: formData.end_time,
				max_slots: formData.max_slots,
				price_vnd: formData.price_vnd
			};

			if (formData.description) {
				payload.description = formData.description;
			}

			if (formData.early_access_ends_at) {
				payload.early_access_ends_at = formData.early_access_ends_at;
			}

			const response = await api.sessions.create(payload);

			// Redirect to session detail
			goto(`/sessions/${response.data.id}`);
		} catch (err: any) {
			error = err.response?.data?.message || err.message || 'Failed to create session';
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Create Session - Loafy Club</title>
</svelte:head>

<Navigation />

<div class="mx-auto max-w-3xl px-4 py-8 sm:px-6 lg:px-8">
	<div class="mb-6">
		<Button variant="ghost" onclick={() => goto('/sessions')}>
			← Back to Sessions
		</Button>
	</div>

	<Card class="p-6">
		<h1 class="text-3xl font-bold text-gray-900">Create New Session</h1>
		<p class="mt-2 text-gray-600">Schedule a new pickleball session</p>

		<form onsubmit={handleSubmit} class="mt-8 space-y-6">
			<div>
				<label for="title" class="block text-sm font-medium text-gray-700">
					Title <span class="text-destructive">*</span>
				</label>
				<input
					id="title"
					type="text"
					bind:value={formData.title}
					required
					class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
					placeholder="Sunday Morning Pickleball"
				/>
			</div>

			<div>
				<label for="description" class="block text-sm font-medium text-gray-700">
					Description
				</label>
				<textarea
					id="description"
					bind:value={formData.description}
					rows="3"
					class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
					placeholder="Optional description of the session..."
				></textarea>
			</div>

			<div>
				<label for="location" class="block text-sm font-medium text-gray-700">
					Location <span class="text-destructive">*</span>
				</label>
				<input
					id="location"
					type="text"
					bind:value={formData.location}
					required
					class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
					placeholder="Hanoi Sports Center"
				/>
			</div>

			<div class="grid gap-4 sm:grid-cols-2">
				<div>
					<label for="start_time" class="block text-sm font-medium text-gray-700">
						Start Time <span class="text-destructive">*</span>
					</label>
					<input
						id="start_time"
						type="datetime-local"
						bind:value={formData.start_time}
						required
						class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
					/>
				</div>

				<div>
					<label for="end_time" class="block text-sm font-medium text-gray-700">
						End Time <span class="text-destructive">*</span>
					</label>
					<input
						id="end_time"
						type="datetime-local"
						bind:value={formData.end_time}
						required
						class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
					/>
				</div>
			</div>

			<div class="grid gap-4 sm:grid-cols-2">
				<div>
					<label for="max_slots" class="block text-sm font-medium text-gray-700">
						Maximum Slots <span class="text-destructive">*</span>
					</label>
					<input
						id="max_slots"
						type="number"
						bind:value={formData.max_slots}
						required
						min="1"
						max="100"
						class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
					/>
				</div>

				<div>
					<label for="price_vnd" class="block text-sm font-medium text-gray-700">
						Price (VND) <span class="text-destructive">*</span>
					</label>
					<input
						id="price_vnd"
						type="number"
						bind:value={formData.price_vnd}
						required
						min="0"
						step="1000"
						class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
					/>
				</div>
			</div>

			<div>
				<label for="early_access" class="block text-sm font-medium text-gray-700">
					Early Access Ends At (Optional)
				</label>
				<input
					id="early_access"
					type="datetime-local"
					bind:value={formData.early_access_ends_at}
					class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
				/>
				<p class="mt-1 text-xs text-gray-500">
					Subscribers get early access until this time (Phase 2 feature)
				</p>
			</div>

			{#if error}
				<div class="rounded-md bg-destructive/10 p-4">
					<p class="text-sm text-destructive">{error}</p>
				</div>
			{/if}

			<div class="flex gap-4">
				<Button type="submit" size="lg" disabled={loading}>
					{loading ? 'Creating...' : 'Create Session'}
				</Button>
				<Button type="button" variant="outline" size="lg" onclick={() => goto('/sessions')}>
					Cancel
				</Button>
			</div>
		</form>
	</Card>
</div>

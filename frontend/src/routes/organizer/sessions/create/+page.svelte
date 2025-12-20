<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { requireRole } from '$lib/guards/auth';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { BackButton } from '$lib/components/ui/back-button';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';

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
			goto(`/sessions/${response.data.id}`);
		} catch (err: any) {
			error = err.response?.data?.message || err.message || 'Failed to create session';
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Create Session - Loafy Club</title>
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
	<link href="https://fonts.googleapis.com/css2?family=Baloo+2:wght@400;500;600;700;800&display=swap" rel="stylesheet">
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<div class="mx-auto max-w-3xl px-4 py-8 sm:px-6 lg:px-8">
		<AnimatedContainer animation="fade-up">
			<BackButton href="/sessions" label="Back to Sessions" />
		</AnimatedContainer>

		<AnimatedContainer animation="fade-up" delay={100}>
			<GlassCard>
				<h1 class="text-3xl font-bold text-gray-800" style="font-family: 'Baloo 2', sans-serif;">
					Create New Session
				</h1>
				<p class="mt-2 text-gray-600">Schedule a new pickleball session</p>

				<form onsubmit={handleSubmit} class="mt-8 space-y-6">
					<div>
						<Label for="title">
							Title <span class="text-red-500">*</span>
						</Label>
						<Input
							id="title"
							type="text"
							bind:value={formData.title}
							required
							placeholder="Sunday Morning Pickleball"
							class="mt-1 rounded-xl border-2 border-gray-200 focus:border-orange-400 focus:ring-orange-200"
						/>
					</div>

					<div>
						<Label for="description">Description</Label>
						<textarea
							id="description"
							bind:value={formData.description}
							rows="3"
							class="mt-1 block w-full rounded-xl border-2 border-gray-200 px-4 py-3 focus:border-orange-400 focus:outline-none focus:ring-2 focus:ring-orange-200 transition-colors"
							placeholder="Optional description of the session..."
						></textarea>
					</div>

					<div>
						<Label for="location">
							Location <span class="text-red-500">*</span>
						</Label>
						<Input
							id="location"
							type="text"
							bind:value={formData.location}
							required
							placeholder="Hanoi Sports Center"
							class="mt-1 rounded-xl border-2 border-gray-200 focus:border-orange-400 focus:ring-orange-200"
						/>
					</div>

					<div class="grid gap-4 sm:grid-cols-2">
						<div>
							<Label for="start_time">
								Start Time <span class="text-red-500">*</span>
							</Label>
							<Input
								id="start_time"
								type="datetime-local"
								bind:value={formData.start_time}
								required
								class="mt-1 rounded-xl border-2 border-gray-200 focus:border-orange-400 focus:ring-orange-200"
							/>
						</div>

						<div>
							<Label for="end_time">
								End Time <span class="text-red-500">*</span>
							</Label>
							<Input
								id="end_time"
								type="datetime-local"
								bind:value={formData.end_time}
								required
								class="mt-1 rounded-xl border-2 border-gray-200 focus:border-orange-400 focus:ring-orange-200"
							/>
						</div>
					</div>

					<div class="grid gap-4 sm:grid-cols-2">
						<div>
							<Label for="max_slots">
								Maximum Slots <span class="text-red-500">*</span>
							</Label>
							<Input
								id="max_slots"
								type="number"
								bind:value={formData.max_slots}
								required
								min="1"
								max="100"
								class="mt-1 rounded-xl border-2 border-gray-200 focus:border-orange-400 focus:ring-orange-200"
							/>
						</div>

						<div>
							<Label for="price_vnd">
								Price (VND) <span class="text-red-500">*</span>
							</Label>
							<Input
								id="price_vnd"
								type="number"
								bind:value={formData.price_vnd}
								required
								min="0"
								step="1000"
								class="mt-1 rounded-xl border-2 border-gray-200 focus:border-orange-400 focus:ring-orange-200"
							/>
						</div>
					</div>

					<div>
						<Label for="early_access">Early Access Ends At (Optional)</Label>
						<Input
							id="early_access"
							type="datetime-local"
							bind:value={formData.early_access_ends_at}
							class="mt-1 rounded-xl border-2 border-gray-200 focus:border-orange-400 focus:ring-orange-200"
						/>
						<p class="mt-1 text-xs text-gray-500">
							Subscribers get early access until this time (Phase 2 feature)
						</p>
					</div>

					{#if error}
						<Alert variant="destructive">
							<AlertDescription>{error}</AlertDescription>
						</Alert>
					{/if}

					<div class="flex gap-4 pt-4">
						<Button
							type="submit"
							size="lg"
							class="bg-gradient-to-r from-orange-500 to-pink-500 border-0"
							disabled={loading}
						>
							{loading ? 'Creating...' : 'Create Session'}
						</Button>
						<Button type="button" variant="outline" size="lg" onclick={() => goto('/sessions')}>
							Cancel
						</Button>
					</div>
				</form>
			</GlassCard>
		</AnimatedContainer>
	</div>
</PageBackground>

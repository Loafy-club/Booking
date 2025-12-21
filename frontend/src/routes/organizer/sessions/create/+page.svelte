<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { extractErrorMessage } from '$lib/utils';
	import { requireRole } from '$lib/guards/auth';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import { Button } from '$lib/components/ui/button';
	import { GlassCard } from '$lib/components/ui/glass-card';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { BackButton } from '$lib/components/ui/back-button';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';

	const t = useTranslation();

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
				throw new Error(t('organizer.createSession.errors.endTimeAfterStart'));
			}

			if (startTime < new Date()) {
				throw new Error(t('organizer.createSession.errors.startTimeInFuture'));
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
			error = extractErrorMessage(err, t('organizer.createSession.errors.createFailed'));
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>{t('organizer.createSession.pageTitle')} - Loafy Club</title>
</svelte:head>

<PageBackground variant="subtle">
	<Navigation />

	<div class="mx-auto max-w-3xl px-4 py-8 sm:px-6 lg:px-8">
		<AnimatedContainer animation="fade-up">
			<BackButton href="/sessions" label={t('organizer.createSession.backToSessions')} />
		</AnimatedContainer>

		<AnimatedContainer animation="fade-up" delay={100}>
			<GlassCard>
				<h1 class="text-3xl font-bold text-gray-800 font-display">
					{t('organizer.createSession.title')}
				</h1>
				<p class="mt-2 text-gray-600">{t('organizer.createSession.subtitle')}</p>

				<form onsubmit={handleSubmit} class="mt-8 space-y-6">
					<div>
						<Label for="title">
							{t('organizer.createSession.form.title')} <span class="text-red-500">{t('organizer.createSession.form.required')}</span>
						</Label>
						<Input
							id="title"
							type="text"
							bind:value={formData.title}
							required
							placeholder={t('organizer.createSession.form.titlePlaceholder')}
							variant="styled" class="mt-1"
						/>
					</div>

					<div>
						<Label for="description">{t('organizer.createSession.form.description')}</Label>
						<Textarea
							id="description"
							bind:value={formData.description}
							rows={3}
							variant="styled"
							class="mt-1"
							placeholder={t('organizer.createSession.form.descriptionPlaceholder')}
						/>
					</div>

					<div>
						<Label for="location">
							{t('organizer.createSession.form.location')} <span class="text-red-500">{t('organizer.createSession.form.required')}</span>
						</Label>
						<Input
							id="location"
							type="text"
							bind:value={formData.location}
							required
							placeholder={t('organizer.createSession.form.locationPlaceholder')}
							variant="styled" class="mt-1"
						/>
					</div>

					<div class="grid gap-4 sm:grid-cols-2">
						<div>
							<Label for="start_time">
								{t('organizer.createSession.form.startTime')} <span class="text-red-500">{t('organizer.createSession.form.required')}</span>
							</Label>
							<Input
								id="start_time"
								type="datetime-local"
								bind:value={formData.start_time}
								required
								variant="styled" class="mt-1"
							/>
						</div>

						<div>
							<Label for="end_time">
								{t('organizer.createSession.form.endTime')} <span class="text-red-500">{t('organizer.createSession.form.required')}</span>
							</Label>
							<Input
								id="end_time"
								type="datetime-local"
								bind:value={formData.end_time}
								required
								variant="styled" class="mt-1"
							/>
						</div>
					</div>

					<div class="grid gap-4 sm:grid-cols-2">
						<div>
							<Label for="max_slots">
								{t('organizer.createSession.form.maxSlots')} <span class="text-red-500">{t('organizer.createSession.form.required')}</span>
							</Label>
							<Input
								id="max_slots"
								type="number"
								bind:value={formData.max_slots}
								required
								min="1"
								max="100"
								variant="styled" class="mt-1"
							/>
						</div>

						<div>
							<Label for="price_vnd">
								{t('organizer.createSession.form.priceVnd')} <span class="text-red-500">{t('organizer.createSession.form.required')}</span>
							</Label>
							<Input
								id="price_vnd"
								type="number"
								bind:value={formData.price_vnd}
								required
								min="0"
								step="1000"
								variant="styled" class="mt-1"
							/>
						</div>
					</div>

					<div>
						<Label for="early_access">{t('organizer.createSession.form.earlyAccess')}</Label>
						<Input
							id="early_access"
							type="datetime-local"
							bind:value={formData.early_access_ends_at}
							variant="styled" class="mt-1"
						/>
						<p class="mt-1 text-xs text-gray-500">
							{t('organizer.createSession.form.earlyAccessNote')}
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
							variant="gradient"
							size="lg"
							disabled={loading}
						>
							{loading ? t('organizer.createSession.buttons.creating') : t('organizer.createSession.buttons.create')}
						</Button>
						<Button type="button" variant="outline" size="lg" onclick={() => goto('/sessions')}>
							{t('organizer.createSession.buttons.cancel')}
						</Button>
					</div>
				</form>
			</GlassCard>
		</AnimatedContainer>
	</div>
</PageBackground>

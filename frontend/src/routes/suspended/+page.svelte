<script lang="ts">
	import { page } from '$app/stores';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { supabase } from '$lib/auth/supabase';
	import { AppLayout } from '$lib/components/layouts';
	import { Card } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { SectionHeader } from '$lib/components/ui/section-header';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Ban } from 'lucide-svelte';

	const t = useTranslation();

	const reason = $page.url.searchParams.get('reason') || t('suspended.defaultReason');
	const until = $page.url.searchParams.get('until');

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString(undefined, {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	async function handleSignOut() {
		await supabase.auth.signOut();
		window.location.href = '/';
	}
</script>

<svelte:head>
	<title>{t('suspended.title')} - Loafy Club</title>
</svelte:head>

<AppLayout>
	<AnimatedContainer animation="fade-up">
		<SectionHeader
			title={t('suspended.title')}
			subtitle={t('suspended.description')}
			gradient
			centered
		/>
	</AnimatedContainer>

	<AnimatedContainer animation="fade-up" delay={100}>
		<Card variant="glass" class="mx-auto max-w-md text-center">
			<div
				class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-destructive/10"
			>
				<Ban class="h-8 w-8 text-destructive" />
			</div>

			<div class="bg-muted rounded-lg p-4 mb-6 text-left">
				<p class="text-sm font-medium mb-1">{t('suspended.reason')}:</p>
				<p class="text-sm text-muted-foreground">{reason}</p>

				{#if until}
					<p class="text-sm font-medium mt-3 mb-1">{t('suspended.until')}:</p>
					<p class="text-sm text-muted-foreground">
						{formatDate(until)}
					</p>
				{:else}
					<p class="mt-3 text-sm text-muted-foreground italic">
						{t('suspended.indefinite')}
					</p>
				{/if}
			</div>

			<p class="text-sm text-muted-foreground mb-6">
				{t('suspended.contactSupportPrefix')}
				<a href="/contact" class="text-primary hover:underline">{t('suspended.contactSupportLink')}</a>.
			</p>

			<Button variant="outline" onclick={handleSignOut}>
				{t('common.signOut')}
			</Button>
		</Card>
	</AnimatedContainer>
</AppLayout>

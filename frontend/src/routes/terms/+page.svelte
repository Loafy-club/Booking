<script lang="ts">
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { AppLayout } from '$lib/components/layouts';
	import { Card } from '$lib/components/ui/card';
	import { SectionHeader } from '$lib/components/ui/section-header';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Scale } from 'lucide-svelte';

	const t = useTranslation();

	const sections = [
		'acceptance',
		'accounts',
		'bookings',
		'cancellation',
		'waitlist',
		'conduct',
		'liability',
		'changes',
		'contact'
	] as const;
</script>

<svelte:head>
	<title>{t('terms.title')} - Loafy Club</title>
</svelte:head>

<AppLayout>
	<!-- Header -->
	<AnimatedContainer animation="fade-up">
		<div class="flex flex-col items-center mb-8">
			<div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-primary/10">
				<Scale class="h-8 w-8 text-primary" />
			</div>
			<SectionHeader title={t('terms.title')} gradient centered />
			<p class="text-sm text-muted-foreground mt-2">
				{t('terms.lastUpdated', { date: 'December 2025' })}
			</p>
		</div>
	</AnimatedContainer>

	<!-- Sections -->
	<div class="max-w-3xl mx-auto space-y-6">
		{#each sections as section, index}
			<AnimatedContainer animation="fade-up" delay={100 + index * 50}>
				<Card class="p-6">
					<h2 class="text-lg font-semibold text-foreground mb-3">
						{t(`terms.sections.${section}.title`)}
					</h2>
					<p class="text-muted-foreground leading-relaxed">
						{t(`terms.sections.${section}.content`)}
					</p>
				</Card>
			</AnimatedContainer>
		{/each}
	</div>

	<!-- Back to Home Link -->
	<AnimatedContainer animation="fade-up" delay={600}>
		<div class="text-center mt-8">
			<a href="/" class="text-sm text-primary hover:underline">
				{t('error.goHome')}
			</a>
		</div>
	</AnimatedContainer>
</AppLayout>

<script lang="ts">
	import { Card } from '$lib/components/ui/card';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { generateDonutArcs } from '$lib/utils/chart-utils';

	interface Props {
		pending: number;
		confirmed: number;
		cancelled: number;
	}

	let { pending, confirmed, cancelled }: Props = $props();
	const t = useTranslation();

	const total = $derived(pending + confirmed + cancelled);

	const segments = $derived(
		[
			{
				value: confirmed,
				color: 'var(--color-chart-4)',
				label: t('admin.charts.confirmed'),
				id: 'confirmed'
			},
			{
				value: pending,
				color: 'var(--color-chart-1)',
				label: t('admin.charts.pending'),
				id: 'pending'
			},
			{
				value: cancelled,
				color: 'var(--color-destructive)',
				label: t('admin.charts.cancelled'),
				id: 'cancelled'
			}
		].filter((d) => d.value > 0)
	);

	const arcs = $derived(generateDonutArcs(segments, total));
</script>

<Card variant="glass" class="h-full">
	<div class="flex h-full flex-col">
		<h3 class="mb-4 text-sm font-medium text-muted-foreground">{t('admin.charts.bookingStatus')}</h3>

		{#if total === 0}
			<div class="flex flex-1 items-center justify-center text-muted-foreground">
				{t('admin.charts.noData')}
			</div>
		{:else}
			<div class="flex flex-1 flex-col items-center justify-center">
				<!-- Donut Chart -->
				<div class="relative">
					<svg width="180" height="180" viewBox="-100 -100 200 200">
						<!-- Donut segments -->
						{#each arcs as arc}
							<path
								d={arc.path}
								fill={arc.color}
								class="transition-opacity duration-200 hover:opacity-80"
							/>
						{/each}

						<!-- Center text -->
						<text
							x="0"
							y="-5"
							text-anchor="middle"
							dominant-baseline="middle"
							class="fill-foreground"
							style="font-size: 28px; font-weight: bold;"
						>
							{total}
						</text>
						<text
							x="0"
							y="18"
							text-anchor="middle"
							dominant-baseline="middle"
							class="fill-muted-foreground"
							style="font-size: 12px;"
						>
							{t('admin.charts.total')}
						</text>
					</svg>
				</div>

				<!-- Legend -->
				<div class="mt-4 flex flex-wrap justify-center gap-4">
					{#each segments as segment}
						<div class="flex items-center gap-2">
							<div
								class="h-3 w-3 rounded-full"
								style="background-color: {segment.color}"
							></div>
							<span class="text-xs text-muted-foreground">
								{segment.label} ({segment.value})
							</span>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>
</Card>

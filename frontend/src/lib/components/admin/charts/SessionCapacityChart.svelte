<script lang="ts">
	import { Card } from '$lib/components/ui/card';
	import { useTranslation } from '$lib/i18n/index.svelte';

	interface Session {
		id: string;
		title: string;
		date: string;
		time: string;
		available_slots: number;
		total_slots: number;
	}

	interface Props {
		sessions: Session[];
	}

	let { sessions }: Props = $props();
	const t = useTranslation();

	// Calculate totals
	const totals = $derived.by(() => {
		const totalSlots = sessions.reduce((sum, s) => sum + s.total_slots, 0);
		const bookedSlots = sessions.reduce((sum, s) => sum + (s.total_slots - s.available_slots), 0);
		const availableSlots = totalSlots - bookedSlots;
		const fillRate = totalSlots > 0 ? Math.round((bookedSlots / totalSlots) * 100) : 0;
		return { totalSlots, bookedSlots, availableSlots, fillRate };
	});

	// Donut chart segments (same order as BookingStatusChart for consistency)
	const segments = $derived([
		{ label: t('admin.charts.booked'), count: totals.bookedSlots, color: 'var(--color-chart-4)' },
		{ label: t('admin.charts.available'), count: totals.availableSlots, color: 'var(--color-chart-3)' }
	].filter(d => d.count > 0));

	// Calculate SVG arc paths for donut chart (same as BookingStatusChart)
	function getArcPath(startAngle: number, endAngle: number, innerRadius: number, outerRadius: number): string {
		const startOuter = polarToCartesian(0, 0, outerRadius, endAngle);
		const endOuter = polarToCartesian(0, 0, outerRadius, startAngle);
		const startInner = polarToCartesian(0, 0, innerRadius, endAngle);
		const endInner = polarToCartesian(0, 0, innerRadius, startAngle);

		const largeArcFlag = endAngle - startAngle <= 180 ? 0 : 1;

		return [
			'M', startOuter.x, startOuter.y,
			'A', outerRadius, outerRadius, 0, largeArcFlag, 0, endOuter.x, endOuter.y,
			'L', endInner.x, endInner.y,
			'A', innerRadius, innerRadius, 0, largeArcFlag, 1, startInner.x, startInner.y,
			'Z'
		].join(' ');
	}

	function polarToCartesian(cx: number, cy: number, radius: number, angleInDegrees: number) {
		const angleInRadians = (angleInDegrees - 90) * Math.PI / 180;
		return {
			x: cx + radius * Math.cos(angleInRadians),
			y: cy + radius * Math.sin(angleInRadians)
		};
	}

	const arcs = $derived.by(() => {
		if (totals.totalSlots === 0) return [];

		const result: Array<{ path: string; color: string; label: string }> = [];
		let currentAngle = 0;
		const gap = 2; // Gap between segments in degrees

		for (const segment of segments) {
			const segmentAngle = (segment.count / totals.totalSlots) * 360 - gap;
			if (segmentAngle > 0) {
				result.push({
					path: getArcPath(currentAngle, currentAngle + segmentAngle, 50, 80),
					color: segment.color,
					label: segment.label
				});
			}
			currentAngle += segmentAngle + gap;
		}

		return result;
	});
</script>

<Card variant="glass" class="h-full">
	<div class="flex h-full flex-col">
		<h3 class="mb-4 text-sm font-medium text-muted-foreground">{t('admin.charts.sessionCapacity')}</h3>

		{#if sessions.length === 0}
			<div class="flex flex-1 items-center justify-center text-muted-foreground">
				{t('admin.charts.noSessions')}
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
							{totals.fillRate}%
						</text>
						<text
							x="0"
							y="18"
							text-anchor="middle"
							dominant-baseline="middle"
							class="fill-muted-foreground"
							style="font-size: 12px;"
						>
							{t('admin.charts.filled')}
						</text>
					</svg>
				</div>

			<!-- Legend -->
			<div class="mt-4 flex flex-wrap justify-center gap-4">
				{#each segments as segment}
					<div class="flex items-center gap-2">
						<div class="h-3 w-3 rounded-full" style="background-color: {segment.color}"></div>
						<span class="text-xs text-muted-foreground">{segment.label} ({segment.count})</span>
					</div>
				{/each}
			</div>
		</div>
		{/if}
	</div>
</Card>

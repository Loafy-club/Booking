<script lang="ts">
	import { Card } from '$lib/components/ui/card';
	import { useTranslation } from '$lib/i18n/index.svelte';

	interface ExpenseCategory {
		category: string;
		total_vnd: number;
		percentage: number;
	}

	interface Props {
		expenses: ExpenseCategory[];
	}

	let { expenses }: Props = $props();
	const t = useTranslation();

	const categoryColors: Record<string, string> = {
		court_rental: 'var(--color-chart-1)',
		equipment: 'var(--color-chart-2)',
		instructor: 'var(--color-chart-3)',
		custom: 'var(--color-chart-4)'
	};

	const total = $derived(expenses.reduce((sum, e) => sum + e.total_vnd, 0));

	const segments = $derived(
		expenses
			.filter(e => e.total_vnd > 0)
			.map(e => ({
				category: e.category,
				amount: e.total_vnd,
				percentage: e.percentage,
				color: categoryColors[e.category] || 'var(--color-chart-5)',
				label: t(`admin.profit.categories.${e.category}`)
			}))
	);

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
		if (total === 0) return [];

		const result: Array<{ path: string; color: string; category: string }> = [];
		let currentAngle = 0;
		const gap = 2;

		for (const segment of segments) {
			const segmentAngle = (segment.amount / total) * 360 - gap;
			if (segmentAngle > 0) {
				result.push({
					path: getArcPath(currentAngle, currentAngle + segmentAngle, 50, 80),
					color: segment.color,
					category: segment.category
				});
			}
			currentAngle += segmentAngle + gap;
		}

		return result;
	});

	function formatCurrency(amount: number): string {
		if (amount >= 1000000) {
			return `${(amount / 1000000).toFixed(1)}M`;
		} else if (amount >= 1000) {
			return `${Math.round(amount / 1000)}K`;
		}
		return amount.toLocaleString();
	}
</script>

<Card variant="glass" class="h-full">
	<div class="flex h-full flex-col">
		<h3 class="mb-4 text-sm font-medium text-muted-foreground">{t('admin.profit.expenseBreakdown')}</h3>

		{#if total === 0}
			<div class="flex flex-1 items-center justify-center text-muted-foreground">
				{t('admin.profit.noExpenses')}
			</div>
		{:else}
			<div class="flex flex-1 flex-col items-center justify-center py-4">
				<div class="relative flex-1 flex items-center justify-center">
					<svg width="200" height="200" viewBox="-100 -100 200 200" class="max-w-full">
						{#each arcs as arc}
							<path
								d={arc.path}
								fill={arc.color}
								class="transition-opacity duration-200 hover:opacity-80"
							/>
						{/each}

						<text
							x="0"
							y="-5"
							text-anchor="middle"
							dominant-baseline="middle"
							class="fill-foreground"
							style="font-size: 20px; font-weight: bold;"
						>
							{formatCurrency(total)}
						</text>
						<text
							x="0"
							y="18"
							text-anchor="middle"
							dominant-baseline="middle"
							class="fill-muted-foreground"
							style="font-size: 11px;"
						>
							VND
						</text>
					</svg>
				</div>

				<div class="mt-4 flex flex-wrap justify-center gap-x-4 gap-y-2">
					{#each segments as segment}
						<div class="flex items-center gap-1.5">
							<div
								class="h-2.5 w-2.5 rounded-full"
								style="background-color: {segment.color}"
							></div>
							<span class="text-xs text-muted-foreground">
								{segment.label} ({Math.round(segment.percentage)}%)
							</span>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>
</Card>

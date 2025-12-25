<script lang="ts">
	import { Card } from '$lib/components/ui/card';
	import { useTranslation } from '$lib/i18n/index.svelte';

	interface DailyDataPoint {
		date: string;
		revenue_vnd: number;
		expenses_vnd: number;
		profit_vnd: number;
	}

	interface Props {
		data: DailyDataPoint[];
	}

	let { data }: Props = $props();
	const t = useTranslation();

	const chartWidth = 400;
	const chartHeight = 200;
	const paddingX = 40;
	const paddingY = 30;
	const paddingTop = 20;

	const displayData = $derived(data.length > 0 ? data : []);

	const maxValue = $derived.by(() => {
		if (displayData.length === 0) return 100;
		const max = Math.max(...displayData.flatMap(d => [d.revenue_vnd, d.expenses_vnd]));
		return max || 100;
	});

	function getXPos(index: number): number {
		if (displayData.length <= 1) return paddingX;
		return paddingX + (index / (displayData.length - 1)) * (chartWidth - paddingX * 2);
	}

	function getYPos(value: number): number {
		const usableHeight = chartHeight - paddingY - paddingTop;
		return paddingTop + usableHeight - (value / maxValue) * usableHeight;
	}

	const revenuePath = $derived.by(() => {
		if (displayData.length < 2) return '';
		return displayData.map((d, i) =>
			`${i === 0 ? 'M' : 'L'} ${getXPos(i)},${getYPos(d.revenue_vnd)}`
		).join(' ');
	});

	const expensesPath = $derived.by(() => {
		if (displayData.length < 2) return '';
		return displayData.map((d, i) =>
			`${i === 0 ? 'M' : 'L'} ${getXPos(i)},${getYPos(d.expenses_vnd)}`
		).join(' ');
	});

	const revenueAreaPath = $derived.by(() => {
		if (displayData.length < 2) return '';
		const linePath = displayData.map((d, i) =>
			`${i === 0 ? 'M' : 'L'} ${getXPos(i)},${getYPos(d.revenue_vnd)}`
		).join(' ');
		const bottomY = chartHeight - paddingY;
		return `${linePath} L ${getXPos(displayData.length - 1)},${bottomY} L ${getXPos(0)},${bottomY} Z`;
	});

	const expensesAreaPath = $derived.by(() => {
		if (displayData.length < 2) return '';
		const linePath = displayData.map((d, i) =>
			`${i === 0 ? 'M' : 'L'} ${getXPos(i)},${getYPos(d.expenses_vnd)}`
		).join(' ');
		const bottomY = chartHeight - paddingY;
		return `${linePath} L ${getXPos(displayData.length - 1)},${bottomY} L ${getXPos(0)},${bottomY} Z`;
	});

	const yAxisLabels = $derived.by(() => {
		const labels = [];
		const steps = 4;
		for (let i = 0; i <= steps; i++) {
			const value = (maxValue / steps) * i;
			labels.push({
				value,
				y: getYPos(value),
				label: formatCompact(value)
			});
		}
		return labels;
	});

	const xAxisLabels = $derived.by(() => {
		if (displayData.length === 0) return [];
		const step = Math.max(1, Math.floor(displayData.length / 5));
		return displayData
			.filter((_, i) => i === 0 || i === displayData.length - 1 || i % step === 0)
			.map((d, _, arr) => {
				const idx = displayData.indexOf(d);
				return {
					x: getXPos(idx),
					label: formatDate(d.date)
				};
			});
	});

	function formatCompact(value: number): string {
		if (value >= 1000000) {
			return `${(value / 1000000).toFixed(1)}M`;
		} else if (value >= 1000) {
			return `${Math.round(value / 1000)}K`;
		}
		return value.toString();
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

	const totalRevenue = $derived(displayData.reduce((sum, d) => sum + d.revenue_vnd, 0));
	const totalExpenses = $derived(displayData.reduce((sum, d) => sum + d.expenses_vnd, 0));
</script>

<Card variant="glass" class="h-full">
	<div class="flex h-full flex-col">
		<div class="mb-4 flex items-center justify-between">
			<h3 class="text-sm font-medium text-muted-foreground">{t('admin.profit.revenueVsExpenses')}</h3>
			<div class="flex items-center gap-4 text-xs">
				<div class="flex items-center gap-1.5">
					<div class="h-2 w-2 rounded-full" style="background-color: var(--color-chart-4)"></div>
					<span class="text-muted-foreground">{t('admin.profit.revenue')}</span>
				</div>
				<div class="flex items-center gap-1.5">
					<div class="h-2 w-2 rounded-full" style="background-color: var(--color-chart-2)"></div>
					<span class="text-muted-foreground">{t('admin.profit.expenses')}</span>
				</div>
			</div>
		</div>

		{#if displayData.length < 2}
			<div class="flex flex-1 items-center justify-center text-muted-foreground">
				{t('admin.charts.noData')}
			</div>
		{:else}
			<div class="flex-1">
				<svg viewBox="0 0 {chartWidth} {chartHeight}" class="h-full w-full" preserveAspectRatio="xMidYMid meet">
					<defs>
						<linearGradient id="revenue-gradient" x1="0" y1="0" x2="0" y2="1">
							<stop offset="0%" style="stop-color: var(--color-chart-4); stop-opacity: 0.3" />
							<stop offset="100%" style="stop-color: var(--color-chart-4); stop-opacity: 0.05" />
						</linearGradient>
						<linearGradient id="expenses-gradient" x1="0" y1="0" x2="0" y2="1">
							<stop offset="0%" style="stop-color: var(--color-chart-2); stop-opacity: 0.2" />
							<stop offset="100%" style="stop-color: var(--color-chart-2); stop-opacity: 0.05" />
						</linearGradient>
					</defs>

					<!-- Grid lines -->
					{#each yAxisLabels as label}
						<line
							x1={paddingX}
							y1={label.y}
							x2={chartWidth - paddingX}
							y2={label.y}
							stroke="currentColor"
							stroke-opacity="0.1"
							stroke-dasharray="4 4"
						/>
						<text
							x={paddingX - 8}
							y={label.y}
							text-anchor="end"
							dominant-baseline="middle"
							class="fill-muted-foreground"
							style="font-size: 10px;"
						>
							{label.label}
						</text>
					{/each}

					<!-- X axis labels -->
					{#each xAxisLabels as label}
						<text
							x={label.x}
							y={chartHeight - 10}
							text-anchor="middle"
							class="fill-muted-foreground"
							style="font-size: 10px;"
						>
							{label.label}
						</text>
					{/each}

					<!-- Area fills -->
					<path d={revenueAreaPath} fill="url(#revenue-gradient)" />
					<path d={expensesAreaPath} fill="url(#expenses-gradient)" />

					<!-- Lines -->
					<path
						d={revenuePath}
						fill="none"
						stroke="var(--color-chart-4)"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
					<path
						d={expensesPath}
						fill="none"
						stroke="var(--color-chart-2)"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>

					<!-- Data points -->
					{#each displayData as d, i}
						<circle
							cx={getXPos(i)}
							cy={getYPos(d.revenue_vnd)}
							r="3"
							fill="var(--color-chart-4)"
							class="opacity-0 hover:opacity-100 transition-opacity"
						/>
						<circle
							cx={getXPos(i)}
							cy={getYPos(d.expenses_vnd)}
							r="3"
							fill="var(--color-chart-2)"
							class="opacity-0 hover:opacity-100 transition-opacity"
						/>
					{/each}
				</svg>
			</div>
		{/if}
	</div>
</Card>

<script lang="ts">
	import { Card } from '$lib/components/ui/card';
	import { ArrowUpRight, TrendingUp, TrendingDown, Minus } from 'lucide-svelte';

	interface DataPoint {
		date: string;
		value: number;
	}

	interface Props {
		title: string;
		value: string | number;
		icon: any;
		iconColor?: string;
		href?: string;
		variant?: 'default' | 'compact' | 'action';
		previousValue?: number;
		currentValue?: number;
		chartData?: DataPoint[];
	}

	let {
		title,
		value,
		icon: Icon,
		iconColor = 'text-muted-foreground',
		href,
		variant = 'default',
		previousValue,
		currentValue,
		chartData = []
	}: Props = $props();

	const isClickable = !!href;
	const cardClass = isClickable
		? '[&>div:last-child]:overflow-hidden [&>div:last-child]:transition-colors [&>div:last-child]:hover:bg-muted/30'
		: '[&>div:last-child]:overflow-hidden';

	// Calculate percentage change
	const percentChange = $derived.by(() => {
		if (previousValue === undefined || currentValue === undefined) return null;
		if (previousValue === 0) {
			return currentValue > 0 ? 100 : 0;
		}
		return Math.round(((currentValue - previousValue) / previousValue) * 100);
	});

	const trend = $derived.by(() => {
		if (percentChange === null) return 'neutral';
		if (percentChange > 0) return 'up';
		if (percentChange < 0) return 'down';
		return 'neutral';
	});

	// Generate sparkline path from chart data (uses viewBox coordinates)
	const sparklinePath = $derived.by(() => {
		if (chartData.length < 2) return '';

		const values = chartData.map(d => d.value);
		const maxVal = Math.max(...values, 1);
		const minVal = Math.min(...values, 0);
		const range = maxVal - minVal || 1;

		const width = 100; // viewBox width
		const height = 28;
		const paddingY = 4;

		const points = chartData.map((d, i) => {
			const x = (i / (chartData.length - 1)) * width;
			const y = height - paddingY - ((d.value - minVal) / range) * (height - paddingY * 2);
			return `${x},${y}`;
		});

		return `M ${points.join(' L ')}`;
	});

	// Area path for filled sparkline
	const sparklineAreaPath = $derived.by(() => {
		if (chartData.length < 2) return '';

		const values = chartData.map(d => d.value);
		const maxVal = Math.max(...values, 1);
		const minVal = Math.min(...values, 0);
		const range = maxVal - minVal || 1;

		const width = 100; // viewBox width
		const height = 28;
		const paddingY = 4;

		const points = chartData.map((d, i) => {
			const x = (i / (chartData.length - 1)) * width;
			const y = height - paddingY - ((d.value - minVal) / range) * (height - paddingY * 2);
			return `${x},${y}`;
		});

		return `M 0,${height} L ${points.join(' L ')} L ${width},${height} Z`;
	});

	// Sparkline color based on trend
	const sparklineColor = $derived.by(() => {
		if (trend === 'up') return 'var(--color-chart-4)'; // green
		if (trend === 'down') return 'var(--color-chart-2)'; // red
		return 'var(--color-chart-3)'; // neutral
	});
</script>

{#snippet content()}
	{#if variant === 'action'}
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-3">
				<Icon class="h-5 w-5 {iconColor}" />
				<span class="font-medium">{title}</span>
			</div>
			{#if isClickable}
				<ArrowUpRight class="h-4 w-4 text-muted-foreground" />
			{/if}
		</div>
	{:else if variant === 'compact'}
		<div class="flex items-center gap-4">
			<div class="rounded-lg bg-muted/50 p-3">
				<Icon class="h-5 w-5 {iconColor}" />
			</div>
			<div>
				<p class="text-sm font-medium text-muted-foreground">{title}</p>
				<p class="text-2xl font-bold text-foreground">{value}</p>
			</div>
		</div>
	{:else}
		<div class="relative pb-6">
			<div class="flex items-start justify-between">
				<div class="flex-1">
					<p class="text-sm font-medium text-muted-foreground">{title}</p>
					<div class="mt-1">
						<p class="text-3xl font-bold text-foreground">{value}</p>
						{#if percentChange !== null}
							<div class="mt-1 flex w-fit items-center gap-0.5 rounded-full px-1.5 py-0.5 {trend === 'up' ? 'bg-emerald-500/10' : trend === 'down' ? 'bg-red-500/10' : 'bg-muted'}">
								{#if trend === 'up'}
									<TrendingUp class="h-3 w-3 text-emerald-600 dark:text-emerald-400" />
									<span class="text-xs font-semibold text-emerald-600 dark:text-emerald-400">+{percentChange}%</span>
								{:else if trend === 'down'}
									<TrendingDown class="h-3 w-3 text-red-600 dark:text-red-400" />
									<span class="text-xs font-semibold text-red-600 dark:text-red-400">{percentChange}%</span>
								{:else}
									<Minus class="h-3 w-3 text-muted-foreground" />
									<span class="text-xs font-semibold text-muted-foreground">0%</span>
								{/if}
							</div>
						{/if}
					</div>
				</div>
				<div class="rounded-xl bg-muted/50 p-2.5">
					<Icon class="h-5 w-5 {iconColor}" />
				</div>
			</div>
			<!-- Sparkline chart -->
			{#if chartData.length >= 2}
				<div class="absolute -bottom-8 -left-8 -right-8">
					<svg viewBox="0 0 100 24" preserveAspectRatio="none" class="h-6 w-full">
						<defs>
							<linearGradient id="sparkline-gradient-{title.replace(/\s/g, '')}" x1="0" y1="0" x2="0" y2="1">
								<stop offset="0%" style="stop-color: {sparklineColor}; stop-opacity: 0.25" />
								<stop offset="100%" style="stop-color: {sparklineColor}; stop-opacity: 0.05" />
							</linearGradient>
						</defs>
						<path
							d={sparklineAreaPath}
							fill="url(#sparkline-gradient-{title.replace(/\s/g, '')})"
						/>
						<path
							d={sparklinePath}
							fill="none"
							stroke={sparklineColor}
							stroke-width="1.5"
							stroke-linecap="round"
							stroke-linejoin="round"
							vector-effect="non-scaling-stroke"
						/>
					</svg>
				</div>
			{/if}
		</div>
	{/if}
{/snippet}

{#if href}
	<a {href}>
		<Card variant="glass" class={cardClass}>
			{@render content()}
		</Card>
	</a>
{:else}
	<Card variant="glass" class={cardClass}>
		{@render content()}
	</Card>
{/if}

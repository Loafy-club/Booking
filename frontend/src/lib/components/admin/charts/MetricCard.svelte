<script lang="ts">
	import { Card } from '$lib/components/ui/card';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Info, TrendingUp, TrendingDown, Minus } from 'lucide-svelte';
	import {
		generateSparklinePaths,
		getSparklineColor,
		calculateTrend,
		type DataPoint
	} from '$lib/utils/sparkline';

	interface Props {
		label: string;
		value: string | number;
		suffix?: string;
		icon?: any;
		iconColor?: string;
		tooltip?: string;
		previousValue?: number;
		currentValue?: number;
		chartData?: DataPoint[];
	}

	let {
		label,
		value,
		suffix = '',
		icon: Icon,
		iconColor = 'text-muted-foreground',
		tooltip,
		previousValue,
		currentValue,
		chartData = []
	}: Props = $props();

	// Calculate trend and percentage change using utility
	const trendData = $derived(calculateTrend(previousValue, currentValue));
	const percentChange = $derived(trendData.percentChange);
	const trend = $derived(trendData.trend);

	// Generate sparkline paths using utility
	const sparklinePaths = $derived(generateSparklinePaths(chartData));
	const sparklinePath = $derived(sparklinePaths.linePath);
	const sparklineAreaPath = $derived(sparklinePaths.areaPath);

	// Sparkline color based on trend
	const sparklineColor = $derived(getSparklineColor(trend));
</script>

<Card variant="glass" class="[&>div:last-child]:overflow-hidden relative">
	<div class="relative z-10 flex items-center gap-3 pb-5">
		{#if Icon}
			<div class="rounded-xl bg-muted/50 p-2.5">
				<Icon class="h-4 w-4 {iconColor}" />
			</div>
		{/if}
		<div class="min-w-0 flex-1">
			<div class="flex items-center gap-1">
				<p class="truncate text-xs font-medium text-muted-foreground">{label}</p>
				{#if tooltip}
					<Tooltip.Root>
						<Tooltip.Trigger>
							<Info class="h-3 w-3 text-muted-foreground/60 hover:text-muted-foreground cursor-help" />
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p class="max-w-[200px] text-xs">{tooltip}</p>
						</Tooltip.Content>
					</Tooltip.Root>
				{/if}
			</div>
			<div class="flex items-baseline gap-2">
				<p class="text-xl font-bold text-foreground">{value}</p>
				{#if suffix}
					<span class="text-sm text-muted-foreground">{suffix}</span>
				{/if}
				{#if percentChange !== null}
					<div class="flex items-center gap-0.5 rounded-full px-1.5 py-0.5 {trend === 'up' ? 'bg-emerald-500/10' : trend === 'down' ? 'bg-red-500/10' : 'bg-muted'}">
						{#if trend === 'up'}
							<TrendingUp class="h-2.5 w-2.5 text-emerald-600 dark:text-emerald-400" />
							<span class="text-[10px] font-semibold text-emerald-600 dark:text-emerald-400">+{percentChange}%</span>
						{:else if trend === 'down'}
							<TrendingDown class="h-2.5 w-2.5 text-red-600 dark:text-red-400" />
							<span class="text-[10px] font-semibold text-red-600 dark:text-red-400">{percentChange}%</span>
						{:else}
							<Minus class="h-2.5 w-2.5 text-muted-foreground" />
							<span class="text-[10px] font-semibold text-muted-foreground">0%</span>
						{/if}
					</div>
				{/if}
			</div>
		</div>
	</div>
	<!-- Sparkline chart -->
	{#if chartData.length >= 2}
		<div class="absolute bottom-0 left-0 right-0">
			<svg viewBox="0 0 100 24" preserveAspectRatio="none" class="h-6 w-full">
				<defs>
					<linearGradient id="metric-sparkline-gradient-{label.replace(/\s/g, '')}" x1="0" y1="0" x2="0" y2="1">
						<stop offset="0%" style="stop-color: {sparklineColor}; stop-opacity: 0.25" />
						<stop offset="100%" style="stop-color: {sparklineColor}; stop-opacity: 0.05" />
					</linearGradient>
				</defs>
				<path
					d={sparklineAreaPath}
					fill="url(#metric-sparkline-gradient-{label.replace(/\s/g, '')})"
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
</Card>

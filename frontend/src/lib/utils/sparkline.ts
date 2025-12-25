/**
 * Sparkline chart utilities for generating SVG paths from data points.
 *
 * These utilities are extracted from MetricCard and StatCard components
 * to provide reusable sparkline path generation.
 */

export interface DataPoint {
	date: string;
	value: number;
}

export interface SparklinePaths {
	linePath: string;
	areaPath: string;
}

export interface SparklineOptions {
	width?: number;
	height?: number;
	paddingY?: number;
}

const DEFAULT_OPTIONS: Required<SparklineOptions> = {
	width: 100,
	height: 24,
	paddingY: 3
};

/**
 * Generates sparkline SVG paths from chart data.
 *
 * @param data - Array of data points with date and value
 * @param options - Optional configuration for dimensions
 * @returns Object containing linePath and areaPath for SVG rendering
 *
 * @example
 * ```ts
 * const { linePath, areaPath } = generateSparklinePaths(chartData);
 * // Use in SVG:
 * // <path d={linePath} fill="none" stroke="currentColor" />
 * // <path d={areaPath} fill="url(#gradient)" />
 * ```
 */
export function generateSparklinePaths(
	data: DataPoint[],
	options: SparklineOptions = {}
): SparklinePaths {
	if (data.length < 2) {
		return { linePath: '', areaPath: '' };
	}

	const { width, height, paddingY } = { ...DEFAULT_OPTIONS, ...options };

	const values = data.map((d) => d.value);
	const maxVal = Math.max(...values, 1);
	const minVal = Math.min(...values, 0);
	const range = maxVal - minVal || 1;

	const points = data.map((d, i) => {
		const x = (i / (data.length - 1)) * width;
		const y = height - paddingY - ((d.value - minVal) / range) * (height - paddingY * 2);
		return { x, y };
	});

	const linePath = `M ${points.map((p) => `${p.x},${p.y}`).join(' L ')}`;
	const areaPath = `M 0,${height} L ${points.map((p) => `${p.x},${p.y}`).join(' L ')} L ${width},${height} Z`;

	return { linePath, areaPath };
}

export type TrendDirection = 'up' | 'down' | 'neutral';

/**
 * Gets the appropriate color variable for a sparkline based on trend direction.
 *
 * @param trend - The trend direction
 * @returns CSS variable string for the color
 */
export function getSparklineColor(trend: TrendDirection): string {
	if (trend === 'up') return 'var(--color-chart-4)';
	if (trend === 'down') return 'var(--color-chart-2)';
	return 'var(--color-chart-3)';
}

/**
 * Calculates trend direction and percentage change from previous and current values.
 *
 * @param previousValue - The previous period's value
 * @param currentValue - The current period's value
 * @returns Object with percentChange and trend direction
 */
export function calculateTrend(
	previousValue: number | undefined,
	currentValue: number | undefined
): { percentChange: number | null; trend: TrendDirection } {
	if (previousValue === undefined || currentValue === undefined) {
		return { percentChange: null, trend: 'neutral' };
	}

	let percentChange: number;
	if (previousValue === 0) {
		percentChange = currentValue > 0 ? 100 : 0;
	} else {
		percentChange = Math.round(((currentValue - previousValue) / previousValue) * 100);
	}

	let trend: TrendDirection;
	if (percentChange > 0) trend = 'up';
	else if (percentChange < 0) trend = 'down';
	else trend = 'neutral';

	return { percentChange, trend };
}

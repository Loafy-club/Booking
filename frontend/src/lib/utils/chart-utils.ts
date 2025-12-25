/**
 * Chart utilities for generating donut/pie chart SVG paths.
 *
 * These utilities are extracted from BookingStatusChart, ExpenseCategoryChart,
 * and SessionCapacityChart components for reusable donut chart generation.
 */

export interface Point {
	x: number;
	y: number;
}

export interface DonutArcOptions {
	/** Inner radius of the donut (default: 50) */
	innerRadius?: number;
	/** Outer radius of the donut (default: 80) */
	outerRadius?: number;
	/** Gap between segments in degrees (default: 2) */
	gap?: number;
}

export interface Segment {
	value: number;
	color: string;
	label?: string;
	id?: string;
}

export interface ArcData {
	path: string;
	color: string;
	label?: string;
	id?: string;
	startAngle: number;
	endAngle: number;
}

const DEFAULT_DONUT_OPTIONS: Required<DonutArcOptions> = {
	innerRadius: 50,
	outerRadius: 80,
	gap: 2
};

/**
 * Converts polar coordinates to Cartesian coordinates.
 *
 * @param cx - Center x coordinate
 * @param cy - Center y coordinate
 * @param radius - Radius from center
 * @param angleInDegrees - Angle in degrees (0 is top, clockwise)
 * @returns Point with x and y coordinates
 */
export function polarToCartesian(
	cx: number,
	cy: number,
	radius: number,
	angleInDegrees: number
): Point {
	const angleInRadians = ((angleInDegrees - 90) * Math.PI) / 180;
	return {
		x: cx + radius * Math.cos(angleInRadians),
		y: cy + radius * Math.sin(angleInRadians)
	};
}

/**
 * Generates an SVG arc path for a donut segment.
 *
 * @param startAngle - Starting angle in degrees
 * @param endAngle - Ending angle in degrees
 * @param innerRadius - Inner radius of the donut
 * @param outerRadius - Outer radius of the donut
 * @param cx - Center x coordinate (default: 0)
 * @param cy - Center y coordinate (default: 0)
 * @returns SVG path string for the arc
 */
export function getArcPath(
	startAngle: number,
	endAngle: number,
	innerRadius: number,
	outerRadius: number,
	cx = 0,
	cy = 0
): string {
	const startOuter = polarToCartesian(cx, cy, outerRadius, endAngle);
	const endOuter = polarToCartesian(cx, cy, outerRadius, startAngle);
	const startInner = polarToCartesian(cx, cy, innerRadius, endAngle);
	const endInner = polarToCartesian(cx, cy, innerRadius, startAngle);

	const largeArcFlag = endAngle - startAngle <= 180 ? 0 : 1;

	return [
		'M',
		startOuter.x,
		startOuter.y,
		'A',
		outerRadius,
		outerRadius,
		0,
		largeArcFlag,
		0,
		endOuter.x,
		endOuter.y,
		'L',
		endInner.x,
		endInner.y,
		'A',
		innerRadius,
		innerRadius,
		0,
		largeArcFlag,
		1,
		startInner.x,
		startInner.y,
		'Z'
	].join(' ');
}

/**
 * Generates donut chart arc data from segments.
 *
 * @param segments - Array of segments with value and color
 * @param total - Total value for calculating proportions (if not provided, sums segments)
 * @param options - Optional configuration for radii and gap
 * @returns Array of arc data with paths and colors
 *
 * @example
 * ```ts
 * const segments = [
 *   { value: 10, color: 'red', label: 'Pending' },
 *   { value: 30, color: 'green', label: 'Confirmed' }
 * ];
 * const arcs = generateDonutArcs(segments);
 * // Use in SVG:
 * // {#each arcs as arc}
 * //   <path d={arc.path} fill={arc.color} />
 * // {/each}
 * ```
 */
export function generateDonutArcs(
	segments: Segment[],
	total?: number,
	options: DonutArcOptions = {}
): ArcData[] {
	const filteredSegments = segments.filter((s) => s.value > 0);

	if (filteredSegments.length === 0) {
		return [];
	}

	const { innerRadius, outerRadius, gap } = { ...DEFAULT_DONUT_OPTIONS, ...options };
	const calculatedTotal = total ?? filteredSegments.reduce((sum, s) => sum + s.value, 0);

	if (calculatedTotal === 0) {
		return [];
	}

	const result: ArcData[] = [];
	let currentAngle = 0;

	for (const segment of filteredSegments) {
		const segmentAngle = (segment.value / calculatedTotal) * 360 - gap;
		if (segmentAngle > 0) {
			result.push({
				path: getArcPath(currentAngle, currentAngle + segmentAngle, innerRadius, outerRadius),
				color: segment.color,
				label: segment.label,
				id: segment.id,
				startAngle: currentAngle,
				endAngle: currentAngle + segmentAngle
			});
		}
		currentAngle += segmentAngle + gap;
	}

	return result;
}

/**
 * Calculates the percentage of a value relative to total.
 *
 * @param value - The value to calculate percentage for
 * @param total - The total value
 * @param decimals - Number of decimal places (default: 1)
 * @returns Formatted percentage string (e.g., "25.5%")
 */
export function calculatePercentage(value: number, total: number, decimals = 1): string {
	if (total === 0) return '0%';
	const percentage = (value / total) * 100;
	return `${percentage.toFixed(decimals)}%`;
}

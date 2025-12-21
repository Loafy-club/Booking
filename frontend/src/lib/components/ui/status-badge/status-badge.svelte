<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const statusBadgeVariants = tv({
		base: "inline-flex items-center whitespace-nowrap rounded-full px-2.5 py-0.5 text-xs font-medium",
		variants: {
			status: {
				// Booking statuses
				pending: "bg-yellow-100 text-yellow-800",
				confirmed: "bg-green-100 text-green-800",
				failed: "bg-red-100 text-red-800",
				cancelled: "bg-red-100 text-red-800",
				// Session statuses
				draft: "bg-gray-100 text-gray-800",
				published: "bg-green-100 text-green-800",
				upcoming: "bg-green-100 text-green-800",
				in_progress: "bg-blue-100 text-blue-800",
				completed: "bg-purple-100 text-purple-800",
				full: "bg-orange-100 text-orange-800",
				// Default
				default: "bg-gray-100 text-gray-800",
			},
			size: {
				default: "px-2.5 py-0.5 text-xs",
				sm: "px-2 py-0.5 text-[10px]",
				lg: "px-3 py-1 text-sm",
			},
		},
		defaultVariants: {
			status: "default",
			size: "default",
		},
	});

	export type StatusBadgeStatus = VariantProps<typeof statusBadgeVariants>["status"];
	export type StatusBadgeSize = VariantProps<typeof statusBadgeVariants>["size"];
</script>

<script lang="ts">
	import { cn } from "$lib/utils";

	interface Props {
		status: string;
		statusKey?: string;
		variant?: 'booking' | 'session' | 'custom';
		size?: StatusBadgeSize;
		customColors?: { bg: string; text: string };
		class?: string;
	}

	let { status, statusKey, variant = 'booking', size = 'default', customColors, class: className = '' }: Props = $props();

	function getStatusKey(): StatusBadgeStatus {
		if (customColors) return 'default';
		const key = (statusKey || status).toLowerCase();
		// Check if key is a valid status
		const validStatuses = ['pending', 'confirmed', 'failed', 'cancelled', 'draft', 'published', 'upcoming', 'in_progress', 'completed', 'full'];
		return validStatuses.includes(key) ? key as StatusBadgeStatus : 'default';
	}

	const resolvedStatus = $derived(getStatusKey());
</script>

{#if customColors}
	<span class={cn(statusBadgeVariants({ size }), customColors.bg, customColors.text, className)}>
		{status}
	</span>
{:else}
	<span class={cn(statusBadgeVariants({ status: resolvedStatus, size }), className)}>
		{status}
	</span>
{/if}

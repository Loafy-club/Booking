<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const badgeVariants = tv({
		base: "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2",
		variants: {
			variant: {
				default: "border-transparent bg-primary text-primary-foreground shadow hover:bg-primary/80",
				secondary: "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80",
				destructive: "border-transparent bg-destructive text-destructive-foreground shadow hover:bg-destructive/80",
				outline: "text-foreground",
				success: "border-transparent bg-green-500 text-white shadow hover:bg-green-500/80",
				warning: "border-transparent bg-yellow-500 text-white shadow hover:bg-yellow-500/80",
				gradient: "border-transparent bg-gradient-to-r from-orange-500 to-pink-500 text-white shadow",
				admin: "border-transparent bg-badge-admin-bg text-badge-admin-text",
				organizer: "border-transparent bg-badge-organizer-bg text-badge-organizer-text",
				muted: "border-transparent bg-muted text-muted-foreground",
				// Status variants
				pending: "border-transparent bg-badge-pending-bg text-badge-pending-text",
				confirmed: "border-transparent bg-badge-confirmed-bg text-badge-confirmed-text",
				failed: "border-transparent bg-badge-failed-bg text-badge-failed-text",
				cancelled: "border-transparent bg-badge-failed-bg text-badge-failed-text",
				upcoming: "border-transparent bg-badge-confirmed-bg text-badge-confirmed-text",
				"in-progress": "border-transparent bg-badge-progress-bg text-badge-progress-text",
				completed: "border-transparent bg-badge-completed-bg text-badge-completed-text",
				full: "border-transparent bg-badge-full-bg text-badge-full-text",
			},
			size: {
				default: "px-2.5 py-0.5 text-xs",
				sm: "px-2 py-0.5 text-[10px]",
				lg: "px-3 py-1 text-sm",
			},
		},
		defaultVariants: {
			variant: "default",
			size: "default",
		},
	});

	export type BadgeVariant = VariantProps<typeof badgeVariants>["variant"];
	export type BadgeSize = VariantProps<typeof badgeVariants>["size"];
</script>

<script lang="ts">
	import { cn } from "$lib/utils";
	import type { HTMLAttributes } from "svelte/elements";

	type Props = HTMLAttributes<HTMLDivElement> & {
		variant?: BadgeVariant;
		size?: BadgeSize;
	};

	let { class: className, variant = "default", size = "default", children, ...restProps }: Props = $props();
</script>

<div
	class={cn(badgeVariants({ variant, size }), className)}
	{...restProps}
>
	{@render children?.()}
</div>

<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const cardVariants = tv({
		base: "rounded-xl text-card-foreground",
		variants: {
			variant: {
				default: "border bg-card shadow",
				info: "p-4 bg-gray-50",
				infoSm: "p-3 bg-gray-50",
				ghost: "bg-transparent",
				outline: "border-2 border-gray-200",
			},
			padding: {
				default: "",
				none: "p-0",
				sm: "p-3",
				md: "p-4",
				lg: "p-6",
				xl: "p-8",
			},
		},
		defaultVariants: {
			variant: "default",
			padding: "default",
		},
	});

	export type CardVariant = VariantProps<typeof cardVariants>["variant"];
	export type CardPadding = VariantProps<typeof cardVariants>["padding"];
</script>

<script lang="ts">
	import { cn } from "$lib/utils";
	import type { HTMLAttributes } from "svelte/elements";

	type Props = HTMLAttributes<HTMLDivElement> & {
		variant?: CardVariant;
		padding?: CardPadding;
	};

	let { class: className, variant = "default", padding = "default", children, ...restProps }: Props = $props();
</script>

<div
	class={cn(cardVariants({ variant, padding }), className)}
	{...restProps}
>
	{@render children?.()}
</div>

<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const selectNativeVariants = tv({
		base: "block w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none disabled:cursor-not-allowed disabled:opacity-50 cursor-pointer",
		variants: {
			variant: {
				default: "border-input bg-background selection:bg-primary dark:bg-input/30 selection:text-primary-foreground ring-offset-background placeholder:text-muted-foreground md:text-sm focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
				styled: "rounded-xl border-2 border-gray-200 px-4 py-3 text-sm focus:border-orange-400 focus:ring-2 focus:ring-orange-200 transition-colors",
				compact: "rounded-lg border px-2 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent",
			},
		},
		defaultVariants: {
			variant: "default",
		},
	});

	export type SelectNativeVariant = VariantProps<typeof selectNativeVariants>["variant"];
</script>

<script lang="ts">
	import type { HTMLSelectAttributes } from "svelte/elements";
	import type { Snippet } from "svelte";
	import type { WithElementRef } from "$lib/utils.js";
	import { cn } from "$lib/utils.js";

	type Props = WithElementRef<HTMLSelectAttributes> & {
		variant?: SelectNativeVariant;
		children: Snippet;
	};

	let {
		ref = $bindable(null),
		value = $bindable(),
		class: className,
		variant = "default",
		children,
		...restProps
	}: Props = $props();
</script>

<select
	bind:this={ref}
	class={cn(selectNativeVariants({ variant }), className)}
	bind:value
	{...restProps}
>
	{@render children()}
</select>



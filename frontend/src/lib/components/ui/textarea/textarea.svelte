<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const textareaVariants = tv({
		base: "flex w-full min-w-0 rounded-md border bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none disabled:cursor-not-allowed disabled:opacity-50 resize-y",
		variants: {
			variant: {
				default: "border-input bg-background selection:bg-primary selection:text-primary-foreground ring-offset-background placeholder:text-muted-foreground md:text-sm focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 aria-invalid:border-destructive",
				styled: "rounded-xl border-2 border-border text-sm focus:border-ring focus:ring-2 focus:ring-ring/30 transition-colors",
			},
		},
		defaultVariants: {
			variant: "default",
		},
	});

	export type TextareaVariant = VariantProps<typeof textareaVariants>["variant"];
</script>

<script lang="ts">
	import type { HTMLTextareaAttributes } from "svelte/elements";
	import type { WithElementRef } from "$lib/utils.js";
	import { cn } from "$lib/utils.js";

	type Props = WithElementRef<HTMLTextareaAttributes> & {
		variant?: TextareaVariant;
	};

	let {
		ref = $bindable(null),
		value = $bindable(),
		class: className,
		variant = "default",
		"data-slot": dataSlot = "textarea",
		...restProps
	}: Props = $props();
</script>

<textarea
	bind:this={ref}
	data-slot={dataSlot}
	class={cn(textareaVariants({ variant }), className)}
	bind:value
	{...restProps}
></textarea>








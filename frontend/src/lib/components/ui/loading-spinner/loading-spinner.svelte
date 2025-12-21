<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const loadingSpinnerVariants = tv({
		slots: {
			wrapper: "flex flex-col items-center justify-center py-12",
			spinner: "animate-spin rounded-full border-primary border-t-transparent",
			text: "mt-4 text-muted-foreground",
		},
		variants: {
			size: {
				sm: { spinner: "h-6 w-6 border-2" },
				md: { spinner: "h-12 w-12 border-4" },
				lg: { spinner: "h-16 w-16 border-4" },
			},
		},
		defaultVariants: {
			size: "md",
		},
	});

	export type LoadingSpinnerSize = VariantProps<typeof loadingSpinnerVariants>["size"];
</script>

<script lang="ts">
	import { cn } from "$lib/utils";

	interface Props {
		size?: LoadingSpinnerSize;
		text?: string;
		class?: string;
	}

	let { size = 'md', text, class: className = '' }: Props = $props();

	const styles = $derived(loadingSpinnerVariants({ size }));
</script>

<div class={cn(styles.wrapper(), className)}>
	<div class={styles.spinner()}></div>
	{#if text}
		<p class={styles.text()}>{text}</p>
	{/if}
</div>

<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const pageTitleVariants = tv({
		slots: {
			wrapper: "",
			title: "font-bold text-gray-800 font-display",
			subtitle: "text-gray-600",
		},
		variants: {
			centered: {
				true: { wrapper: "text-center" },
				false: { wrapper: "" },
			},
			size: {
				default: { title: "text-4xl sm:text-5xl", subtitle: "mt-4 text-lg" },
				sm: { title: "text-2xl sm:text-3xl", subtitle: "mt-2 text-base" },
				lg: { title: "text-5xl sm:text-6xl", subtitle: "mt-6 text-xl" },
			},
		},
		defaultVariants: {
			centered: true,
			size: "default",
		},
	});

	export type PageTitleSize = VariantProps<typeof pageTitleVariants>["size"];
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from "$lib/utils";

	interface Props {
		children: Snippet;
		subtitle?: string;
		centered?: boolean;
		size?: PageTitleSize;
		class?: string;
	}

	let { children, subtitle, centered = true, size = 'default', class: className = '' }: Props = $props();

	const styles = $derived(pageTitleVariants({ centered, size }));
</script>

<div class={cn(styles.wrapper(), className)}>
	<h1 class={styles.title()}>
		{@render children()}
	</h1>
	{#if subtitle}
		<p class={styles.subtitle()}>{subtitle}</p>
	{/if}
</div>

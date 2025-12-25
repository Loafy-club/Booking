<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const sectionHeaderVariants = tv({
		slots: {
			wrapper: "mb-8",
			title: "font-bold font-display",
			subtitle: "mt-2",
			actions: "flex items-center gap-3",
		},
		variants: {
			centered: {
				true: { wrapper: "text-center" },
				false: { wrapper: "flex items-center justify-between" },
			},
			size: {
				default: { title: "text-3xl sm:text-4xl", subtitle: "text-lg" },
				sm: { title: "text-2xl sm:text-3xl", subtitle: "text-base" },
				lg: { title: "text-4xl sm:text-5xl", subtitle: "text-xl" },
			},
		},
		defaultVariants: {
			centered: false,
			size: "default",
		},
	});

	export type SectionHeaderSize = VariantProps<typeof sectionHeaderVariants>["size"];
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import { GradientText } from '$lib/components/ui/gradient-text';
	import { cn } from "$lib/utils";

	interface Props {
		title: string;
		subtitle?: string;
		gradient?: boolean;
		centered?: boolean;
		size?: SectionHeaderSize;
		actions?: Snippet;
		class?: string;
	}

	let {
		title,
		subtitle,
		gradient = false,
		centered = false,
		size = 'default',
		actions,
		class: className = ''
	}: Props = $props();

	const styles = $derived(sectionHeaderVariants({ centered, size }));
</script>

<div class={cn(styles.wrapper(), className)}>
	<div>
		<h1 class={cn(styles.title(), "text-foreground")}>
			{#if gradient}
				<GradientText>{title}</GradientText>
			{:else}
				{title}
			{/if}
		</h1>
		{#if subtitle}
			<p class={cn(styles.subtitle(), "text-muted-foreground")}>{subtitle}</p>
		{/if}
	</div>
	{#if actions && !centered}
		<div class={styles.actions()}>
			{@render actions()}
		</div>
	{/if}
</div>

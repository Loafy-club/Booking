<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const gradientTextVariants = tv({
		base: "text-transparent bg-clip-text bg-gradient-to-r font-display",
		variants: {
			variant: {
				default: "from-orange-500 to-pink-500",
				hero: "from-orange-400 via-pink-500 to-rose-500",
				subtle: "from-orange-400 to-orange-600",
				primary: "from-orange-500 to-pink-500",
			},
			size: {
				default: "",
				xs: "text-xs",
				sm: "text-sm",
				base: "text-base",
				lg: "text-lg",
				xl: "text-xl",
				"2xl": "text-2xl",
				"3xl": "text-3xl",
				"4xl": "text-4xl",
				"5xl": "text-5xl",
			},
			weight: {
				default: "",
				normal: "font-normal",
				medium: "font-medium",
				semibold: "font-semibold",
				bold: "font-bold",
			},
		},
		defaultVariants: {
			variant: "default",
			size: "default",
			weight: "default",
		},
	});

	export type GradientTextVariant = VariantProps<typeof gradientTextVariants>["variant"];
	export type GradientTextSize = VariantProps<typeof gradientTextVariants>["size"];
	export type GradientTextWeight = VariantProps<typeof gradientTextVariants>["weight"];
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from "$lib/utils";

	interface Props {
		children: Snippet;
		variant?: GradientTextVariant;
		size?: GradientTextSize;
		weight?: GradientTextWeight;
		as?: 'span' | 'h1' | 'h2' | 'h3' | 'h4' | 'p';
		class?: string;
	}

	let { children, variant = 'default', size = 'default', weight = 'default', as = 'span', class: className = '' }: Props = $props();
</script>

{#if as === 'h1'}
	<h1 class={cn(gradientTextVariants({ variant, size, weight }), className)}>
		{@render children()}
	</h1>
{:else if as === 'h2'}
	<h2 class={cn(gradientTextVariants({ variant, size, weight }), className)}>
		{@render children()}
	</h2>
{:else if as === 'h3'}
	<h3 class={cn(gradientTextVariants({ variant, size, weight }), className)}>
		{@render children()}
	</h3>
{:else if as === 'h4'}
	<h4 class={cn(gradientTextVariants({ variant, size, weight }), className)}>
		{@render children()}
	</h4>
{:else if as === 'p'}
	<p class={cn(gradientTextVariants({ variant, size, weight }), className)}>
		{@render children()}
	</p>
{:else}
	<span class={cn(gradientTextVariants({ variant, size, weight }), className)}>
		{@render children()}
	</span>
{/if}

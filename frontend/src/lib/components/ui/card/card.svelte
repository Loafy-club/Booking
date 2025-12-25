<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const cardVariants = tv({
		slots: {
			wrapper: "relative",
			glow: "absolute inset-0 rounded-[2rem] blur-xl transition-all duration-500 pointer-events-none",
			card: "rounded-xl text-card-foreground overflow-visible",
		},
		variants: {
			variant: {
				default: {
					wrapper: "",
					glow: "hidden",
					card: "border bg-card shadow",
				},
				glass: {
					wrapper: "group",
					glow: "bg-gradient-to-br from-orange-300/40 to-pink-300/40 opacity-40",
					card: "relative rounded-[2rem] shadow-md backdrop-blur-sm bg-card/85 p-8",
				},
				glassYellow: {
					wrapper: "group",
					glow: "bg-gradient-to-br from-pink-300 to-rose-400 opacity-40",
					card: "relative rounded-[2rem] shadow-md backdrop-blur-sm bg-card/85 p-8",
				},
				glassOrange: {
					wrapper: "group",
					glow: "bg-gradient-to-br from-yellow-300 to-orange-400 opacity-40",
					card: "relative rounded-[2rem] shadow-md backdrop-blur-sm bg-card/85 p-8",
				},
				glassPink: {
					wrapper: "group",
					glow: "bg-gradient-to-br from-orange-300 to-pink-400 opacity-40",
					card: "relative rounded-[2rem] shadow-md backdrop-blur-sm bg-card/85 p-8",
				},
				info: {
					wrapper: "",
					glow: "hidden",
					card: "p-4 bg-muted rounded-xl",
				},
				infoSm: {
					wrapper: "",
					glow: "hidden",
					card: "p-3 bg-muted rounded-xl",
				},
				ghost: {
					wrapper: "",
					glow: "hidden",
					card: "bg-transparent",
				},
				outline: {
					wrapper: "",
					glow: "hidden",
					card: "border-2 border-border",
				},
			},
			hover: {
				true: {},
				false: {},
			},
		},
		compoundVariants: [
			{
				variant: ["glass", "glassYellow", "glassOrange", "glassPink"],
				hover: true,
				class: {
					glow: "group-hover:opacity-60 group-hover:scale-105",
					card: "transition-all duration-500 hover:shadow-2xl hover:-translate-y-2",
				},
			},
		],
		defaultVariants: {
			variant: "default",
			hover: false,
		},
	});

	export type CardVariant = VariantProps<typeof cardVariants>["variant"];
</script>

<script lang="ts">
	import { cn } from "$lib/utils";
	import type { HTMLAttributes } from "svelte/elements";

	type Props = HTMLAttributes<HTMLDivElement> & {
		variant?: CardVariant;
		hover?: boolean;
	};

	let { class: className, variant = "default", hover = false, children, ...restProps }: Props = $props();

	const styles = $derived(cardVariants({ variant, hover }));
	const isGlass = $derived(variant?.startsWith('glass'));
</script>

{#if isGlass}
	<div class={cn(styles.wrapper(), className)}>
		<div class={styles.glow()}></div>
		<div class={cn(styles.card())} {...restProps}>
			{@render children?.()}
		</div>
	</div>
{:else}
	<div class={cn(styles.card(), className)} {...restProps}>
		{@render children?.()}
	</div>
{/if}

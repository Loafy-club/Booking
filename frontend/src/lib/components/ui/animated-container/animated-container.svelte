<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const animatedContainerVariants = tv({
		base: "transition-all ease-out",
		variants: {
			animation: {
				"fade-up": "",
				"fade-left": "",
				"fade-right": "",
				"scale": "",
			},
			visible: {
				true: "",
				false: "",
			},
		},
		compoundVariants: [
			{ animation: "fade-up", visible: false, class: "opacity-0 translate-y-8" },
			{ animation: "fade-up", visible: true, class: "opacity-100 translate-y-0" },
			{ animation: "fade-left", visible: false, class: "opacity-0 -translate-x-8" },
			{ animation: "fade-left", visible: true, class: "opacity-100 translate-x-0" },
			{ animation: "fade-right", visible: false, class: "opacity-0 translate-x-8" },
			{ animation: "fade-right", visible: true, class: "opacity-100 translate-x-0" },
			{ animation: "scale", visible: false, class: "opacity-0 scale-95" },
			{ animation: "scale", visible: true, class: "opacity-100 scale-100" },
		],
		defaultVariants: {
			animation: "fade-up",
			visible: false,
		},
	});

	export type AnimatedContainerAnimation = VariantProps<typeof animatedContainerVariants>["animation"];
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import { onMount } from 'svelte';
	import { cn } from "$lib/utils";

	interface Props {
		children: Snippet;
		animation?: AnimatedContainerAnimation;
		delay?: number;
		duration?: number;
		trigger?: 'load' | 'scroll';
		class?: string;
	}

	let {
		children,
		animation = 'fade-up',
		delay = 0,
		duration = 600,
		trigger = 'load',
		class: className = ''
	}: Props = $props();

	let visible = $state(false);
	let element: HTMLDivElement;

	onMount(() => {
		if (trigger === 'load') {
			setTimeout(() => {
				visible = true;
			}, delay);
		} else if (trigger === 'scroll') {
			const observer = new IntersectionObserver(
				(entries) => {
					entries.forEach((entry) => {
						if (entry.isIntersecting) {
							setTimeout(() => {
								visible = true;
							}, delay);
							observer.unobserve(entry.target);
						}
					});
				},
				{ threshold: 0.1, rootMargin: '0px 0px -50px 0px' }
			);
			observer.observe(element);
			return () => observer.disconnect();
		}
	});
</script>

<div
	bind:this={element}
	class={cn(animatedContainerVariants({ animation, visible }), className)}
	style="transition-duration: {duration}ms;"
>
	{@render children()}
</div>

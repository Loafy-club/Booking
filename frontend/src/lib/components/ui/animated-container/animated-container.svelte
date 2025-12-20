<script lang="ts">
	import type { Snippet } from 'svelte';
	import { onMount } from 'svelte';

	interface Props {
		children: Snippet;
		animation?: 'fade-up' | 'fade-left' | 'fade-right' | 'scale';
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

	const animations = {
		'fade-up': {
			initial: 'opacity-0 translate-y-8',
			visible: 'opacity-100 translate-y-0'
		},
		'fade-left': {
			initial: 'opacity-0 -translate-x-8',
			visible: 'opacity-100 translate-x-0'
		},
		'fade-right': {
			initial: 'opacity-0 translate-x-8',
			visible: 'opacity-100 translate-x-0'
		},
		'scale': {
			initial: 'opacity-0 scale-95',
			visible: 'opacity-100 scale-100'
		}
	};
</script>

<div
	bind:this={element}
	class="transition-all ease-out {visible ? animations[animation].visible : animations[animation].initial} {className}"
	style="transition-duration: {duration}ms;"
>
	{@render children()}
</div>

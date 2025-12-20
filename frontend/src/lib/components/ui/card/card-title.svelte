<script lang="ts">
	import { cn } from "$lib/utils";
	import type { HTMLAttributes } from "svelte/elements";

	type Props = HTMLAttributes<HTMLHeadingElement> & {
		tag?: "h1" | "h2" | "h3" | "h4" | "h5" | "h6";
	};

	let { class: className, tag = "h3", children, ...restProps }: Props = $props();
</script>

<svelte:element
	this={tag}
	class={cn("font-semibold leading-none tracking-tight", className)}
	{...restProps}
>
	{@render children?.()}
</svelte:element>

<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		placeholder,
		children,
		...restProps
	}: HTMLAttributes<HTMLSpanElement> & {
		ref?: HTMLSpanElement | null;
		placeholder?: string;
	} = $props();
</script>

<span
	bind:this={ref}
	data-slot="select-value"
	class={cn("pointer-events-none", className)}
	{...restProps}
>
	{#if children}
		{@render children()}
	{:else if placeholder}
		<span class="text-muted-foreground">{placeholder}</span>
	{/if}
</span>

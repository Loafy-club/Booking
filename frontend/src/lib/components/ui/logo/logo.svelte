<script lang="ts">
	interface Props {
		size?: 'sm' | 'md' | 'lg' | 'xl';
		showText?: boolean;
		showRing?: boolean;
		href?: string;
		class?: string;
	}

	let { size = 'md', showText = true, showRing = false, href = '/', class: className = '' }: Props = $props();

	const sizes = {
		sm: { image: 'h-8 w-8', text: 'text-xl', ring: 'w-12 h-12' },
		md: { image: 'h-12 w-12', text: 'text-2xl', ring: 'w-16 h-16' },
		lg: { image: 'h-16 w-16', text: 'text-3xl', ring: 'w-24 h-24' },
		xl: { image: 'h-24 w-24', text: 'text-4xl', ring: 'w-32 h-32' }
	};
</script>

<a {href} class="inline-flex items-center gap-3 group {className}">
	<div class="relative">
		{#if showRing}
			<div class="absolute inset-0 flex items-center justify-center">
				<div class="{sizes[size].ring} rounded-full border-4 border-dashed border-orange-200/60 animate-spin-very-slow"></div>
			</div>
		{/if}
		<img
			src="/mascot.png"
			alt="Loafy the Corgi"
			class="relative {sizes[size].image} object-contain drop-shadow-lg group-hover:scale-110 transition-transform duration-300"
		/>
	</div>
	{#if showText}
		<span
			class="{sizes[size].text} font-bold text-transparent bg-clip-text bg-gradient-to-r from-orange-500 to-pink-500"
			style="font-family: 'Baloo 2', sans-serif;"
		>
			Loafy Club
		</span>
	{/if}
</a>

<style>
	@keyframes spin-very-slow {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
	:global(.animate-spin-very-slow) {
		animation: spin-very-slow 30s linear infinite;
	}
</style>

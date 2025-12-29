<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { cn } from '$lib/utils';
	import { useTranslation } from '$lib/i18n/index.svelte';

	interface Props {
		deadline: string | Date;
		class?: string;
		onExpire?: () => void;
		showSeconds?: boolean;
		compact?: boolean;
	}

	let { deadline, class: className, onExpire, showSeconds = true, compact = false }: Props = $props();

	const t = useTranslation();

	let timeLeft = $state({ days: 0, hours: 0, minutes: 0, seconds: 0 });
	let expired = $state(false);
	let interval: ReturnType<typeof setInterval> | null = null;

	function calculateTimeLeft() {
		const deadlineDate = typeof deadline === 'string' ? new Date(deadline) : deadline;
		const now = new Date();
		const diff = deadlineDate.getTime() - now.getTime();

		if (diff <= 0) {
			expired = true;
			timeLeft = { days: 0, hours: 0, minutes: 0, seconds: 0 };
			if (interval) {
				clearInterval(interval);
				interval = null;
			}
			onExpire?.();
			return;
		}

		const days = Math.floor(diff / (1000 * 60 * 60 * 24));
		const hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
		const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
		const seconds = Math.floor((diff % (1000 * 60)) / 1000);

		timeLeft = { days, hours, minutes, seconds };
	}

	onMount(() => {
		calculateTimeLeft();
		interval = setInterval(calculateTimeLeft, 1000);
	});

	onDestroy(() => {
		if (interval) {
			clearInterval(interval);
		}
	});

	function pad(n: number): string {
		return n.toString().padStart(2, '0');
	}
</script>

{#if expired}
	<span class={cn('text-destructive font-medium', className)}>
		{t('countdown.expired')}
	</span>
{:else if compact}
	<span class={cn('font-mono font-medium tabular-nums', className)}>
		{#if timeLeft.days > 0}
			{timeLeft.days}d {pad(timeLeft.hours)}:{pad(timeLeft.minutes)}{#if showSeconds}:{pad(timeLeft.seconds)}{/if}
		{:else if timeLeft.hours > 0}
			{pad(timeLeft.hours)}:{pad(timeLeft.minutes)}{#if showSeconds}:{pad(timeLeft.seconds)}{/if}
		{:else}
			{pad(timeLeft.minutes)}{#if showSeconds}:{pad(timeLeft.seconds)}{/if}
		{/if}
	</span>
{:else}
	<span class={cn('inline-flex items-center gap-1 font-medium', className)}>
		{#if timeLeft.days > 0}
			<span class="font-mono tabular-nums">{timeLeft.days}</span>
			<span class="text-muted-foreground">{t('countdown.days')}</span>
		{/if}
		{#if timeLeft.days > 0 || timeLeft.hours > 0}
			<span class="font-mono tabular-nums">{pad(timeLeft.hours)}</span>
			<span class="text-muted-foreground">{t('countdown.hours')}</span>
		{/if}
		<span class="font-mono tabular-nums">{pad(timeLeft.minutes)}</span>
		<span class="text-muted-foreground">{t('countdown.minutes')}</span>
		{#if showSeconds}
			<span class="font-mono tabular-nums">{pad(timeLeft.seconds)}</span>
			<span class="text-muted-foreground">{t('countdown.seconds')}</span>
		{/if}
	</span>
{/if}

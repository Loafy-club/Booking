<script lang="ts">
	import { ParticipantAvatars } from '$lib/components/ui/participant-avatars';
	import { Users } from 'lucide-svelte';
	import { cn } from '$lib/utils';

	interface SessionData {
		id: string;
		total_slots: number;
		available_slots: number;
		participants_preview?: Array<{
			id: string;
			name: string | null;
			avatar_url: string | null;
			guest_count: number;
		}> | null;
		confirmed_count?: number | null;
	}

	interface Props {
		/** Session data object */
		session: SessionData;
		/** Display variant */
		variant?: 'compact' | 'default' | 'detailed';
		/** Show the Users icon */
		showIcon?: boolean;
		/** Additional class for the container */
		class?: string;
	}

	let {
		session,
		variant = 'default',
		showIcon = false,
		class: className
	}: Props = $props();

	// Variant configurations
	const variantConfig = {
		compact: { maxDisplay: 3, size: 'sm' as const },
		default: { maxDisplay: 4, size: 'sm' as const },
		detailed: { maxDisplay: 6, size: 'md' as const }
	};

	const config = $derived(variantConfig[variant]);
</script>

{#if showIcon}
	<div class={cn('flex items-center gap-2', className)}>
		<Users class="h-4 w-4 flex-shrink-0 text-muted-foreground" />
		<ParticipantAvatars
			sessionId={session.id}
			participants={session.participants_preview ?? []}
			confirmedCount={session.confirmed_count ?? 0}
			maxDisplay={config.maxDisplay}
			size={config.size}
			showCount
			totalSlots={session.total_slots}
			availableSlots={session.available_slots}
		/>
	</div>
{:else}
	<ParticipantAvatars
		sessionId={session.id}
		participants={session.participants_preview ?? []}
		confirmedCount={session.confirmed_count ?? 0}
		maxDisplay={config.maxDisplay}
		size={config.size}
		showCount
		totalSlots={session.total_slots}
		availableSlots={session.available_slots}
		class={className}
	/>
{/if}

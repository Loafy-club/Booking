<script lang="ts">
	import type { ParticipantInfo } from '$lib/types/ParticipantInfo';
	import type { SessionParticipantsResponse } from '$lib/types/SessionParticipantsResponse';
	import { Avatar, AvatarImage, AvatarFallback } from '$lib/components/ui/avatar';
	import { Popover, PopoverContent, PopoverTrigger } from '$lib/components/ui/popover';
	import { Spinner } from '$lib/components/ui/spinner';
	import { api } from '$lib/api/client';
	import { cn } from '$lib/utils';
	import { useTranslation } from '$lib/i18n/index.svelte';

	const t = useTranslation();

	interface Props {
		/** Session ID for fetching full participant list */
		sessionId: string;
		/** Preview participants (max 5) */
		participants: ParticipantInfo[];
		/** Total count of confirmed participants */
		confirmedCount: number;
		/** Maximum avatars to show before +N badge */
		maxDisplay?: number;
		/** Size of each avatar */
		size?: 'sm' | 'md' | 'lg';
		/** Show inline slot count (e.g., "5/8 booked") */
		showCount?: boolean;
		/** Total slots for the session (required if showCount is true) */
		totalSlots?: number;
		/** Available slots remaining (required if showCount is true) */
		availableSlots?: number;
		/** Additional class for the container */
		class?: string;
	}

	let {
		sessionId,
		participants,
		confirmedCount,
		maxDisplay = 5,
		size = 'sm',
		showCount = false,
		totalSlots = 0,
		availableSlots = 0,
		class: className
	}: Props = $props();

	// Derived values for count display
	const bookedSlots = $derived(totalSlots - availableSlots);
	const isFull = $derived(availableSlots === 0);

	// State for popover
	let isOpen = $state(false);
	let fullParticipants = $state<ParticipantInfo[]>([]);
	let isLoading = $state(false);
	let hasLoadedFull = $state(false);

	// Size classes
	const sizeClasses = {
		sm: 'size-6',
		md: 'size-8',
		lg: 'size-10'
	};

	const textSizeClasses = {
		sm: 'text-[10px]',
		md: 'text-xs',
		lg: 'text-sm'
	};

	// Get initials from name
	function getInitials(name: string | null): string {
		if (!name) return '?';
		return name
			.split(' ')
			.map((n) => n[0])
			.join('')
			.toUpperCase()
			.slice(0, 2);
	}

	// Calculate overflow count
	const overflowCount = $derived(Math.max(0, confirmedCount - maxDisplay));
	const displayParticipants = $derived(participants.slice(0, maxDisplay));

	// Fetch full participants list when popover opens
	async function handleOpenChange(open: boolean) {
		isOpen = open;
		if (open && !hasLoadedFull && confirmedCount > 0) {
			isLoading = true;
			try {
				const response = await api.sessions.getParticipants(sessionId);
				const data = response.data as SessionParticipantsResponse;
				fullParticipants = data.participants;
				hasLoadedFull = true;
			} catch (error) {
				console.error('Failed to load participants:', error);
				// Fall back to preview participants
				fullParticipants = participants;
			} finally {
				isLoading = false;
			}
		}
	}

	// Display list (preview or full based on load state)
	const displayList = $derived(hasLoadedFull ? fullParticipants : participants);
</script>

{#if confirmedCount === 0 && !showCount}
	<span class="text-muted-foreground text-xs">-</span>
{:else if confirmedCount === 0 && showCount}
	<!-- No participants yet, just show count -->
	<div class={cn('flex items-center gap-2', className)}>
		<span class="text-muted-foreground text-xs">-</span>
		<span class={cn('text-sm font-medium', isFull ? 'text-error-text' : 'text-success-text')}>
			{bookedSlots} / {totalSlots}
		</span>
		<span class="text-xs text-muted-foreground">{t('sessions.booked')}</span>
	</div>
{:else}
	<Popover bind:open={isOpen} onOpenChange={handleOpenChange}>
		<PopoverTrigger>
			<button
				type="button"
				class={cn(
					'flex items-center gap-3 cursor-pointer hover:opacity-80 transition-opacity',
					className
				)}
			>
				<!-- Avatar stack -->
				<div class="flex items-center -space-x-2">
					{#each displayParticipants as participant, i (participant.id)}
						<Avatar
							class={cn(
								sizeClasses[size],
								'border-2 border-background ring-0',
								'transition-transform hover:scale-110 hover:z-10'
							)}
							style="z-index: {displayParticipants.length - i}"
						>
							{#if participant.avatar_url}
								<AvatarImage src={participant.avatar_url} alt={participant.name || 'Participant'} />
							{/if}
							<AvatarFallback class={cn(textSizeClasses[size], 'bg-primary text-primary-foreground')}>
								{getInitials(participant.name)}
							</AvatarFallback>
						</Avatar>
					{/each}
					{#if overflowCount > 0}
						<div
							class={cn(
								sizeClasses[size],
								textSizeClasses[size],
								'flex items-center justify-center rounded-full',
								'border-2 border-background bg-muted text-muted-foreground font-medium'
							)}
							style="z-index: 0"
						>
							+{overflowCount}
						</div>
					{/if}
				</div>
				<!-- Inline count display -->
				{#if showCount}
					<div class="flex items-center gap-1.5">
						<span class={cn('text-sm font-medium', isFull ? 'text-error-text' : 'text-success-text')}>
							{bookedSlots} / {totalSlots}
						</span>
						<span class="text-xs text-muted-foreground">{t('sessions.booked')}</span>
					</div>
				{/if}
			</button>
		</PopoverTrigger>
		<PopoverContent class="w-64 p-0" align="start">
			<div class="p-3 border-b">
				<h4 class="font-medium text-sm">
					Participants ({confirmedCount})
				</h4>
			</div>
			<div class="max-h-64 overflow-y-auto">
				{#if isLoading}
					<div class="flex items-center justify-center py-6">
						<Spinner class="size-5" />
					</div>
				{:else}
					<div class="divide-y">
						{#each displayList as participant (participant.id)}
							<div class="flex items-center gap-3 px-3 py-2">
								<Avatar class="size-8">
									{#if participant.avatar_url}
										<AvatarImage
											src={participant.avatar_url}
											alt={participant.name || 'Participant'}
										/>
									{/if}
									<AvatarFallback class="text-xs bg-primary text-primary-foreground">
										{getInitials(participant.name)}
									</AvatarFallback>
								</Avatar>
								<div class="flex-1 min-w-0">
									<p class="text-sm font-medium truncate">
										{participant.name || 'Anonymous'}
									</p>
									{#if participant.guest_count > 0}
										<p class="text-xs text-muted-foreground">
											+{participant.guest_count} guest{participant.guest_count > 1 ? 's' : ''}
										</p>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</PopoverContent>
	</Popover>
{/if}

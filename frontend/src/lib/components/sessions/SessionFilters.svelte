<script lang="ts">
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Select from '$lib/components/ui/select';
	import * as Popover from '$lib/components/ui/popover';
	import { Calendar } from '$lib/components/ui/calendar';
	import { Switch } from '$lib/components/ui/switch';
	import { Label } from '$lib/components/ui/label';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { SlidersHorizontal, X, Check } from 'lucide-svelte';
	import type { DateValue } from '@internationalized/date';
	import { DateFormatter, getLocalTimeZone } from '@internationalized/date';

	const t = useTranslation();
	const df = new DateFormatter('en-US', { month: 'short', day: 'numeric' });

	interface Props {
		dateRange: 'this' | 'next' | 'pick' | 'past';
		pickedDate: DateValue | undefined;
		timeOfDay: ('morning' | 'afternoon' | 'evening')[];
		location: string;
		locations: string[];
		showFull: boolean;
		onDateRangeChange: (range: 'this' | 'next' | 'pick' | 'past') => void;
		onPickedDateChange: (date: DateValue | undefined) => void;
		onTimeOfDayChange: (times: ('morning' | 'afternoon' | 'evening')[]) => void;
		onLocationChange: (location: string) => void;
		onShowFullChange: (show: boolean) => void;
		onClearFilters: () => void;
	}

	let {
		dateRange,
		pickedDate,
		timeOfDay,
		location,
		locations,
		showFull,
		onDateRangeChange,
		onPickedDateChange,
		onTimeOfDayChange,
		onLocationChange,
		onShowFullChange,
		onClearFilters
	}: Props = $props();

	let filterPopoverOpen = $state(false);
	let showCalendar = $state(false);

	// Count active filters
	const activeFilterCount = $derived(
		(dateRange !== 'this' ? 1 : 0) +
		(timeOfDay.length > 0 ? 1 : 0) +
		(location ? 1 : 0) +
		(showFull ? 1 : 0)
	);

	const hasActiveFilters = $derived(activeFilterCount > 0);

	function toggleTimeOfDay(time: 'morning' | 'afternoon' | 'evening') {
		if (timeOfDay.includes(time)) {
			onTimeOfDayChange(timeOfDay.filter((t) => t !== time));
		} else {
			onTimeOfDayChange([...timeOfDay, time]);
		}
	}

	function handleDateSelect(date: DateValue | undefined) {
		onPickedDateChange(date);
		if (date) {
			onDateRangeChange('pick');
			showCalendar = false;
		}
	}

	function handleLocationSelect(value: string | undefined) {
		onLocationChange(value || '');
	}

	function clearAllFilters() {
		onClearFilters();
		filterPopoverOpen = false;
	}

	function selectDateRange(range: 'this' | 'next' | 'past') {
		onDateRangeChange(range);
		showCalendar = false;
	}

	const dateRangeLabel = $derived(() => {
		switch (dateRange) {
			case 'this': return t('sessions.date.thisWeek');
			case 'next': return t('sessions.date.nextWeek');
			case 'past': return t('sessions.date.past');
			case 'pick': return pickedDate ? df.format(pickedDate.toDate(getLocalTimeZone())) : t('sessions.date.pickDate');
			default: return t('sessions.date.thisWeek');
		}
	});

	const timeOptions = [
		{ value: 'morning', label: () => t('sessions.time.morning'), desc: () => t('sessions.time.morningHours') },
		{ value: 'afternoon', label: () => t('sessions.time.afternoon'), desc: () => t('sessions.time.afternoonHours') },
		{ value: 'evening', label: () => t('sessions.time.evening'), desc: () => t('sessions.time.eveningHours') }
	] as const;

	const dateOptions = [
		{ value: 'this', label: () => t('sessions.date.thisWeek') },
		{ value: 'next', label: () => t('sessions.date.nextWeek') },
		{ value: 'past', label: () => t('sessions.date.past') }
	] as const;
</script>

<div class="space-y-3">
	<!-- Filter Button -->
	<div class="flex items-center gap-2">
		<Popover.Root bind:open={filterPopoverOpen}>
			<Popover.Trigger>
				{#snippet child({ props })}
					<Button
						{...props}
						variant={activeFilterCount > 0 ? 'default' : 'outline'}
						size="sm"
						class="h-9 {activeFilterCount > 0 ? 'bg-primary text-primary-foreground' : '!bg-white hover:!bg-gray-50 text-gray-700 border-gray-200 shadow-sm'}"
					>
						<SlidersHorizontal class="h-4 w-4 mr-1.5" />
						{t('common.filter')}
						{#if activeFilterCount > 0}
							<Badge variant="secondary" class="ml-1.5 h-5 w-5 p-0 justify-center text-xs">
								{activeFilterCount}
							</Badge>
						{/if}
					</Button>
				{/snippet}
			</Popover.Trigger>
			<Popover.Content class="w-80" align="start">
				<div class="space-y-4">
					<div class="flex items-center justify-between">
						<h4 class="font-medium">{t('common.filter')}</h4>
						{#if hasActiveFilters}
							<Button variant="ghost" size="sm" class="h-7 text-xs" onclick={clearAllFilters}>
								{t('sessions.filters.clearAll')}
							</Button>
						{/if}
					</div>

					<Separator />

					<!-- Date Range -->
					<div class="space-y-2">
						<Label class="text-sm font-medium">{t('sessions.date.label')}</Label>
						<div class="grid grid-cols-2 gap-2">
							{#each dateOptions as option}
								<button
									class="flex items-center justify-center rounded-md border px-3 py-2 text-sm transition-colors hover:bg-muted {dateRange === option.value ? 'border-primary bg-primary/5 font-medium' : 'border-border'}"
									onclick={() => selectDateRange(option.value)}
								>
									{option.label()}
									{#if dateRange === option.value}
										<Check class="h-3 w-3 ml-1.5 text-primary" />
									{/if}
								</button>
							{/each}
							<button
								class="flex items-center justify-center rounded-md border px-3 py-2 text-sm transition-colors hover:bg-muted {dateRange === 'pick' ? 'border-primary bg-primary/5 font-medium' : 'border-border'}"
								onclick={() => showCalendar = !showCalendar}
							>
								{dateRange === 'pick' && pickedDate ? df.format(pickedDate.toDate(getLocalTimeZone())) : t('sessions.date.pickDate')}
								{#if dateRange === 'pick'}
									<Check class="h-3 w-3 ml-1.5 text-primary" />
								{/if}
							</button>
						</div>
						{#if showCalendar}
							<div class="pt-2">
								<Calendar
									type="single"
									value={pickedDate}
									onValueChange={(date) => handleDateSelect(date)}
								/>
							</div>
						{/if}
					</div>

					<Separator />

					<!-- Time of Day -->
					<div class="space-y-2">
						<Label class="text-sm font-medium">{t('sessions.time.label')}</Label>
						<div class="grid grid-cols-1 gap-2">
							{#each timeOptions as option}
								<button
									class="flex items-center justify-between rounded-md border px-3 py-2 text-sm transition-colors hover:bg-muted {timeOfDay.includes(option.value) ? 'border-primary bg-primary/5' : 'border-border'}"
									onclick={() => toggleTimeOfDay(option.value)}
								>
									<div class="flex flex-col items-start">
										<span class="font-medium">{option.label()}</span>
										<span class="text-xs text-muted-foreground">{option.desc()}</span>
									</div>
									{#if timeOfDay.includes(option.value)}
										<Check class="h-4 w-4 text-primary" />
									{/if}
								</button>
							{/each}
						</div>
					</div>

					<!-- Location -->
					{#if locations.length > 0}
						<Separator />
						<div class="space-y-2">
							<Label class="text-sm font-medium">{t('sessions.location.label')}</Label>
							<Select.Root type="single" value={location || undefined} onValueChange={handleLocationSelect}>
								<Select.Trigger class="w-full">
									{location || t('sessions.location.all')}
								</Select.Trigger>
								<Select.Content>
									<Select.Item value="">{t('sessions.location.all')}</Select.Item>
									{#each locations as loc}
										<Select.Item value={loc}>{loc}</Select.Item>
									{/each}
								</Select.Content>
							</Select.Root>
						</div>
					{/if}

					<Separator />

					<!-- Show Full Sessions -->
					<div class="flex items-center justify-between">
						<Label for="show-full-filter" class="text-sm cursor-pointer">
							{t('sessions.filters.showFull')}
						</Label>
						<Switch
							id="show-full-filter"
							checked={showFull}
							onCheckedChange={onShowFullChange}
						/>
					</div>
				</div>
			</Popover.Content>
		</Popover.Root>

		<!-- Results count will be shown by parent -->
	</div>

	<!-- Active Filter Chips (removable) -->
	{#if hasActiveFilters}
		<div class="flex flex-wrap items-center gap-2">
			{#if dateRange !== 'this'}
				<Badge variant="secondary" class="gap-1 pr-1">
					{dateRangeLabel()}
					<button
						class="ml-1 rounded-full hover:bg-muted-foreground/20 p-0.5"
						onclick={() => onDateRangeChange('this')}
					>
						<X class="h-3 w-3" />
					</button>
				</Badge>
			{/if}
			{#if timeOfDay.length > 0}
				{#each timeOfDay as time}
					<Badge variant="secondary" class="gap-1 pr-1">
						{time === 'morning' ? t('sessions.time.morning') : time === 'afternoon' ? t('sessions.time.afternoon') : t('sessions.time.evening')}
						<button
							class="ml-1 rounded-full hover:bg-muted-foreground/20 p-0.5"
							onclick={() => toggleTimeOfDay(time)}
						>
							<X class="h-3 w-3" />
						</button>
					</Badge>
				{/each}
			{/if}
			{#if location}
				<Badge variant="secondary" class="gap-1 pr-1">
					{location}
					<button
						class="ml-1 rounded-full hover:bg-muted-foreground/20 p-0.5"
						onclick={() => onLocationChange('')}
					>
						<X class="h-3 w-3" />
					</button>
				</Badge>
			{/if}
			{#if showFull}
				<Badge variant="secondary" class="gap-1 pr-1">
					{t('sessions.filters.showFull')}
					<button
						class="ml-1 rounded-full hover:bg-muted-foreground/20 p-0.5"
						onclick={() => onShowFullChange(false)}
					>
						<X class="h-3 w-3" />
					</button>
				</Badge>
			{/if}
		</div>
	{/if}
</div>

<script lang="ts">
	import { CalendarDateTime, getLocalTimeZone, parseDateTime, today, type DateValue } from "@internationalized/date";
	import { Calendar } from "$lib/components/ui/calendar";
	import * as Popover from "$lib/components/ui/popover";
	import { Button } from "$lib/components/ui/button";
	import { cn } from "$lib/utils";
	import { Calendar as CalendarIcon, Clock } from "lucide-svelte";

	interface Props {
		value?: string;
		placeholder?: string;
		disabled?: boolean;
		required?: boolean;
		id?: string;
		class?: string;
		onchange?: (value: string) => void;
	}

	let {
		value = $bindable(""),
		placeholder = "Select date and time",
		disabled = false,
		required = false,
		id,
		class: className,
		onchange
	}: Props = $props();

	let open = $state(false);

	// Parse the ISO string to DateValue for the calendar
	let dateValue = $derived.by(() => {
		if (!value) return undefined;
		try {
			// Parse datetime-local format (YYYY-MM-DDTHH:MM)
			const parsed = parseDateTime(value);
			return parsed;
		} catch {
			return undefined;
		}
	});

	// Extract time from the value
	let timeValue = $derived.by(() => {
		if (!value) return "";
		const parts = value.split("T");
		return parts[1] || "";
	});

	// Format date for display
	let displayValue = $derived.by(() => {
		if (!dateValue) return "";
		const date = dateValue.toDate(getLocalTimeZone());
		return new Intl.DateTimeFormat("en-US", {
			weekday: "short",
			month: "short",
			day: "numeric",
			year: "numeric",
			hour: "numeric",
			minute: "2-digit",
			hour12: true
		}).format(date);
	});

	function handleDateSelect(newDate: DateValue | undefined) {
		if (!newDate) return;

		// Get current time or default to current time
		let hours = 9;
		let minutes = 0;

		if (timeValue) {
			const [h, m] = timeValue.split(":").map(Number);
			hours = h || 0;
			minutes = m || 0;
		}

		// Create CalendarDateTime with the selected date and time
		const datetime = new CalendarDateTime(
			newDate.year,
			newDate.month,
			newDate.day,
			hours,
			minutes
		);

		// Format as datetime-local string (YYYY-MM-DDTHH:MM)
		const newValue = `${datetime.year}-${String(datetime.month).padStart(2, "0")}-${String(datetime.day).padStart(2, "0")}T${String(datetime.hour).padStart(2, "0")}:${String(datetime.minute).padStart(2, "0")}`;
		value = newValue;
		onchange?.(newValue);
	}

	function handleTimeChange(e: Event) {
		const target = e.target as HTMLInputElement;
		const newTime = target.value;

		if (!dateValue) {
			// If no date selected, use today
			const todayDate = today(getLocalTimeZone());
			const [hours, minutes] = newTime.split(":").map(Number);
			const datetime = new CalendarDateTime(
				todayDate.year,
				todayDate.month,
				todayDate.day,
				hours || 0,
				minutes || 0
			);
			const newValue = `${datetime.year}-${String(datetime.month).padStart(2, "0")}-${String(datetime.day).padStart(2, "0")}T${String(datetime.hour).padStart(2, "0")}:${String(datetime.minute).padStart(2, "0")}`;
			value = newValue;
			onchange?.(newValue);
		} else {
			const [hours, minutes] = newTime.split(":").map(Number);
			const datetime = new CalendarDateTime(
				dateValue.year,
				dateValue.month,
				dateValue.day,
				hours || 0,
				minutes || 0
			);
			const newValue = `${datetime.year}-${String(datetime.month).padStart(2, "0")}-${String(datetime.day).padStart(2, "0")}T${String(datetime.hour).padStart(2, "0")}:${String(datetime.minute).padStart(2, "0")}`;
			value = newValue;
			onchange?.(newValue);
		}
	}
</script>

<Popover.Root bind:open>
	<Popover.Trigger>
		{#snippet child({ props })}
			<Button
				{id}
				variant="outline"
				{disabled}
				class={cn(
					"w-full justify-start text-left font-normal rounded-xl border-2 border-border h-auto py-3",
					!value && "text-muted-foreground",
					className
				)}
				{...props}
			>
				<CalendarIcon class="mr-2 size-4" />
				{displayValue || placeholder}
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content class="w-auto p-0" align="start">
		<Calendar
			type="single"
			value={dateValue}
			onValueChange={handleDateSelect}
			captionLayout="dropdown"
			class="rounded-md"
		/>
		<div class="border-t border-border p-3">
			<div class="flex items-center gap-2">
				<Clock class="size-4 text-muted-foreground" />
				<input
					type="time"
					value={timeValue}
					onchange={handleTimeChange}
					class="flex-1 rounded-md border border-input bg-background px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
				/>
			</div>
		</div>
	</Popover.Content>
</Popover.Root>

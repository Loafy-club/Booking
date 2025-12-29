<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Card } from '$lib/components/ui/card';

	type Props = {
		rows?: number;
		cols?: number;
		/** Show avatar in first column (for user tables) */
		showAvatar?: boolean;
		/** Show dropdown select in second-to-last column (for role selects) */
		showRoleSelect?: boolean;
	};

	let { rows = 5, cols = 7, showAvatar = true, showRoleSelect = true }: Props = $props();
</script>

<Card variant="glass" class="overflow-hidden !p-0 [&>div:first-child]:hidden [&>div:last-child]:shadow-none">
	<div class="overflow-x-auto">
		<table class="w-full">
			<thead class="border-b border-border">
				<tr>
					{#each Array(cols) as _, i}
						<th class="h-12 px-4 text-left align-middle">
							{#if i === cols - 1}
								<!-- Empty header for actions column -->
							{:else}
								<Skeleton class="h-4 w-16" />
							{/if}
						</th>
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each Array(rows) as _}
					<tr class="border-b border-border last:border-0">
						{#each Array(cols) as _, i}
							<td class="p-4 align-middle">
								{#if i === 0 && showAvatar}
									<!-- User column with avatar -->
									<div class="flex items-center gap-3">
										<Skeleton class="h-8 w-8 rounded-full" />
										<Skeleton class="h-4 w-24" />
									</div>
								{:else if i === 0}
									<!-- First column without avatar -->
									<Skeleton class="h-5 w-32 mb-1" />
									<Skeleton class="h-3 w-24" />
								{:else if i === cols - 1}
									<!-- Actions column -->
									<div class="flex justify-end">
										<Skeleton class="h-8 w-8 rounded-md" />
									</div>
								{:else if i === cols - 2 && showRoleSelect}
									<!-- Role select column -->
									<Skeleton class="h-8 w-28 rounded-md" />
								{:else if i === 4 || i === 2}
									<!-- Badge columns (status, provider) -->
									<Skeleton class="h-5 w-16 rounded-full" />
								{:else}
									<!-- Regular text column -->
									<Skeleton class="h-4 w-24" />
								{/if}
							</td>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</Card>

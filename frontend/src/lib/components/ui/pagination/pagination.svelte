<script lang="ts">
	import { cn } from '$lib/utils.js';
	import { ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight } from 'lucide-svelte';
	import type { PageInfo } from '$lib/types';

	interface Props {
		pageInfo: PageInfo;
		onPageChange: (page: number) => void;
		class?: string;
		showPageNumbers?: boolean;
		maxVisiblePages?: number;
	}

	let {
		pageInfo,
		onPageChange,
		class: className,
		showPageNumbers = true,
		maxVisiblePages = 5
	}: Props = $props();

	const getVisiblePages = $derived.by(() => {
		const { page, total_pages } = pageInfo;
		const pages: (number | 'ellipsis')[] = [];

		if (total_pages <= maxVisiblePages) {
			for (let i = 1; i <= total_pages; i++) {
				pages.push(i);
			}
		} else {
			const half = Math.floor(maxVisiblePages / 2);
			let start = Math.max(1, page - half);
			let end = Math.min(total_pages, start + maxVisiblePages - 1);

			if (end - start < maxVisiblePages - 1) {
				start = Math.max(1, end - maxVisiblePages + 1);
			}

			if (start > 1) {
				pages.push(1);
				if (start > 2) pages.push('ellipsis');
			}

			for (let i = start; i <= end; i++) {
				if (i !== 1 && i !== total_pages) {
					pages.push(i);
				}
			}

			if (end < total_pages) {
				if (end < total_pages - 1) pages.push('ellipsis');
				pages.push(total_pages);
			}
		}

		return pages;
	});

	function goToPage(page: number) {
		if (page >= 1 && page <= pageInfo.total_pages && page !== pageInfo.page) {
			onPageChange(page);
		}
	}
</script>

<nav
	class={cn('flex items-center justify-between gap-4', className)}
	aria-label="Pagination"
>
	<div class="text-sm text-muted-foreground">
		Showing {Math.min((pageInfo.page - 1) * pageInfo.per_page + 1, pageInfo.total)} to {Math.min(
			pageInfo.page * pageInfo.per_page,
			pageInfo.total
		)} of {pageInfo.total} results
	</div>

	<div class="flex items-center gap-1">
		<!-- First page -->
		<button
			onclick={() => goToPage(1)}
			disabled={pageInfo.page === 1}
			class="inline-flex h-8 w-8 items-center justify-center rounded-md border border-input bg-background text-sm font-medium hover:bg-accent hover:text-accent-foreground disabled:pointer-events-none disabled:opacity-50"
			aria-label="First page"
		>
			<ChevronsLeft class="h-4 w-4" />
		</button>

		<!-- Previous page -->
		<button
			onclick={() => goToPage(pageInfo.page - 1)}
			disabled={pageInfo.page === 1}
			class="inline-flex h-8 w-8 items-center justify-center rounded-md border border-input bg-background text-sm font-medium hover:bg-accent hover:text-accent-foreground disabled:pointer-events-none disabled:opacity-50"
			aria-label="Previous page"
		>
			<ChevronLeft class="h-4 w-4" />
		</button>

		{#if showPageNumbers}
			{#each getVisiblePages as item}
				{#if item === 'ellipsis'}
					<span class="px-2 text-muted-foreground">...</span>
				{:else}
					<button
						onclick={() => goToPage(item)}
						class={cn(
							'inline-flex h-8 min-w-8 items-center justify-center rounded-md px-2 text-sm font-medium transition-colors',
							item === pageInfo.page
								? 'bg-primary text-primary-foreground'
								: 'border border-input bg-background hover:bg-accent hover:text-accent-foreground'
						)}
						aria-current={item === pageInfo.page ? 'page' : undefined}
					>
						{item}
					</button>
				{/if}
			{/each}
		{/if}

		<!-- Next page -->
		<button
			onclick={() => goToPage(pageInfo.page + 1)}
			disabled={pageInfo.page >= pageInfo.total_pages}
			class="inline-flex h-8 w-8 items-center justify-center rounded-md border border-input bg-background text-sm font-medium hover:bg-accent hover:text-accent-foreground disabled:pointer-events-none disabled:opacity-50"
			aria-label="Next page"
		>
			<ChevronRight class="h-4 w-4" />
		</button>

		<!-- Last page -->
		<button
			onclick={() => goToPage(pageInfo.total_pages)}
			disabled={pageInfo.page >= pageInfo.total_pages}
			class="inline-flex h-8 w-8 items-center justify-center rounded-md border border-input bg-background text-sm font-medium hover:bg-accent hover:text-accent-foreground disabled:pointer-events-none disabled:opacity-50"
			aria-label="Last page"
		>
			<ChevronsRight class="h-4 w-4" />
		</button>
	</div>
</nav>

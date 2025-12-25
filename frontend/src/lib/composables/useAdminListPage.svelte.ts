/**
 * Composable for managing admin list pages with URL-synced pagination, sorting, and filtering.
 *
 * Provides a consistent pattern for admin pages that display paginated, sortable,
 * and filterable lists of items with URL persistence.
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { useAdminListPage } from '$lib/composables/useAdminListPage.svelte';
 *   import { page } from '$app/stores';
 *
 *   const listPage = useAdminListPage({
 *     basePath: '/admin/users',
 *     page,
 *     filters: ['role', 'status']
 *   });
 *
 *   onMount(() => {
 *     listPage.initFromUrl();
 *     loadData();
 *   });
 *
 *   async function loadData() {
 *     listPage.startLoading();
 *     try {
 *       const response = await api.admin.listUsers({
 *         page: listPage.currentPage,
 *         per_page: listPage.perPage,
 *         search: listPage.searchQuery,
 *         role: listPage.getFilter('role'),
 *         status: listPage.getFilter('status'),
 *         sort_by: listPage.sortBy,
 *         sort_order: listPage.sortOrder
 *       });
 *       users = response.data.data;
 *       pageInfo = response.data.page_info;
 *       listPage.finishLoading();
 *     } catch (err) {
 *       listPage.setError(extractErrorMessage(err));
 *     }
 *   }
 * </script>
 * ```
 */

import { goto } from '$app/navigation';
import { ArrowUpDown, ArrowUp, ArrowDown } from 'lucide-svelte';
import type { Readable } from 'svelte/store';

export interface PageStore {
	url: URL;
}

export interface AdminListPageOptions {
	/** Base path for URL (e.g., '/admin/users') */
	basePath: string;
	/** Svelte page store */
	page: Readable<PageStore>;
	/** Filter keys to track in URL (e.g., ['role', 'status']) */
	filters?: string[];
	/** Default items per page (default: 10) */
	defaultPerPage?: number;
	/** Delay before search triggers in ms (default: 300) */
	searchDebounceMs?: number;
	/** Delay before showing skeleton in ms (default: 200) */
	skeletonDelayMs?: number;
}

export interface AdminListPageState {
	// Pagination
	currentPage: number;
	perPage: number;
	searchQuery: string;
	sortBy: string | undefined;
	sortOrder: 'asc' | 'desc' | undefined;

	// Loading state
	loading: boolean;
	showSkeleton: boolean;
	error: string | null;

	// Methods
	initFromUrl: () => void;
	updateUrl: () => void;
	handleSearchInput: (e: Event) => void;
	clearSearch: () => void;
	handleSort: (column: string, loadFn: () => void) => void;
	handlePageChange: (newPage: number, loadFn: () => void) => void;
	getSortIcon: (column: string) => typeof ArrowUpDown | typeof ArrowUp | typeof ArrowDown;
	getFilter: (key: string) => string | undefined;
	setFilter: (key: string, value: string, loadFn: () => void) => void;
	startLoading: () => void;
	finishLoading: () => void;
	setError: (message: string | null) => void;
	triggerSearch: (loadFn: () => void) => void;
}

/**
 * Creates a state manager for admin list pages.
 *
 * @param options - Configuration options
 * @returns State object with reactive properties and methods
 */
export function useAdminListPage(options: AdminListPageOptions): AdminListPageState {
	const {
		basePath,
		page,
		filters: filterKeys = [],
		defaultPerPage = 10,
		searchDebounceMs = 300,
		skeletonDelayMs = 200
	} = options;

	// Pagination state
	let currentPage = $state(1);
	let perPage = $state(defaultPerPage);
	let searchQuery = $state('');
	let sortBy = $state<string | undefined>(undefined);
	let sortOrder = $state<'asc' | 'desc' | undefined>(undefined);

	// Filters (dynamic based on filterKeys)
	const filterValues = $state<Record<string, string>>({});

	// Loading state
	let loading = $state(true);
	let showSkeleton = $state(false);
	let error = $state<string | null>(null);

	// Debounce timer
	let searchTimeout: ReturnType<typeof setTimeout> | null = null;
	let skeletonTimer: ReturnType<typeof setTimeout> | null = null;

	// Get the current URL from the page store
	function getCurrentParams(): URLSearchParams {
		let params: URLSearchParams;
		page.subscribe((p) => {
			params = p.url.searchParams;
		})();
		return params!;
	}

	function initFromUrl() {
		const params = getCurrentParams();
		currentPage = parseInt(params.get('page') || '1', 10);
		perPage = parseInt(params.get('per_page') || String(defaultPerPage), 10);
		searchQuery = params.get('search') || '';
		sortBy = params.get('sort_by') || undefined;
		sortOrder = (params.get('sort_order') as 'asc' | 'desc') || undefined;

		// Initialize filters
		for (const key of filterKeys) {
			filterValues[key] = params.get(key) || 'all';
		}
	}

	function updateUrl() {
		const params = new URLSearchParams();
		if (currentPage > 1) params.set('page', currentPage.toString());
		if (perPage !== defaultPerPage) params.set('per_page', perPage.toString());
		if (searchQuery) params.set('search', searchQuery);
		if (sortBy) params.set('sort_by', sortBy);
		if (sortOrder) params.set('sort_order', sortOrder);

		// Add filters
		for (const key of filterKeys) {
			const value = filterValues[key];
			if (value && value !== 'all') {
				params.set(key, value);
			}
		}

		const newUrl = params.toString() ? `?${params.toString()}` : basePath;
		goto(newUrl, { replaceState: true, keepFocus: true });
	}

	function handleSearchInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		searchQuery = value;
	}

	function triggerSearch(loadFn: () => void) {
		if (searchTimeout) clearTimeout(searchTimeout);
		searchTimeout = setTimeout(() => {
			currentPage = 1;
			updateUrl();
			loadFn();
		}, searchDebounceMs);
	}

	function clearSearch() {
		searchQuery = '';
		currentPage = 1;
	}

	function handleSort(column: string, loadFn: () => void) {
		if (sortBy === column) {
			if (sortOrder === 'asc') {
				sortOrder = 'desc';
			} else if (sortOrder === 'desc') {
				sortBy = undefined;
				sortOrder = undefined;
			}
		} else {
			sortBy = column;
			sortOrder = 'asc';
		}
		updateUrl();
		loadFn();
	}

	function handlePageChange(newPage: number, loadFn: () => void) {
		currentPage = newPage;
		updateUrl();
		loadFn();
	}

	function getSortIcon(column: string): typeof ArrowUpDown | typeof ArrowUp | typeof ArrowDown {
		if (sortBy !== column) return ArrowUpDown;
		return sortOrder === 'asc' ? ArrowUp : ArrowDown;
	}

	function getFilter(key: string): string | undefined {
		const value = filterValues[key];
		return value === 'all' ? undefined : value;
	}

	function setFilter(key: string, value: string, loadFn: () => void) {
		filterValues[key] = value || 'all';
		currentPage = 1;
		updateUrl();
		loadFn();
	}

	function startLoading() {
		loading = true;
		error = null;

		// Start skeleton timer
		if (skeletonTimer) clearTimeout(skeletonTimer);
		skeletonTimer = setTimeout(() => {
			if (loading) showSkeleton = true;
		}, skeletonDelayMs);
	}

	function finishLoading() {
		loading = false;
		if (skeletonTimer) {
			clearTimeout(skeletonTimer);
			skeletonTimer = null;
		}
	}

	function setError(message: string | null) {
		error = message;
		loading = false;
		if (skeletonTimer) {
			clearTimeout(skeletonTimer);
			skeletonTimer = null;
		}
	}

	return {
		get currentPage() {
			return currentPage;
		},
		set currentPage(value: number) {
			currentPage = value;
		},
		get perPage() {
			return perPage;
		},
		set perPage(value: number) {
			perPage = value;
		},
		get searchQuery() {
			return searchQuery;
		},
		set searchQuery(value: string) {
			searchQuery = value;
		},
		get sortBy() {
			return sortBy;
		},
		get sortOrder() {
			return sortOrder;
		},
		get loading() {
			return loading;
		},
		get showSkeleton() {
			return showSkeleton;
		},
		get error() {
			return error;
		},
		initFromUrl,
		updateUrl,
		handleSearchInput,
		clearSearch,
		handleSort,
		handlePageChange,
		getSortIcon,
		getFilter,
		setFilter,
		startLoading,
		finishLoading,
		setError,
		triggerSearch
	};
}

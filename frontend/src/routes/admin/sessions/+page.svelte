<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, extractErrorMessage } from '$lib/utils';
	import { requireRole } from '$lib/guards/auth';
	import { authStore } from '$lib/stores/auth.svelte';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { AdminLayout } from '$lib/components/admin';
	import { Button } from '$lib/components/ui/button';
	import { Card } from '$lib/components/ui/card';
	import { SectionHeader } from '$lib/components/ui/section-header';
	import { TableSkeleton } from '$lib/components/ui/skeleton';
	import * as Empty from '$lib/components/ui/empty';
	import { Badge } from '$lib/components/ui/badge';
	import { AnimatedContainer } from '$lib/components/ui/animated-container';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Table from '$lib/components/ui/table';
	import * as Select from '$lib/components/ui/select';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import { DateTimePicker } from '$lib/components/ui/datetime-picker';
	import { Pagination } from '$lib/components/ui/pagination';
	import { Calendar, Search, X, ArrowUpDown, ArrowUp, ArrowDown, MoreHorizontal, Pencil, ExternalLink, Trash2, Plus, ChevronDown, ChevronUp } from 'lucide-svelte';
	import { SessionParticipants } from '$lib/components/ui/session-participants';
	import type { AdminSession, PageInfo } from '$lib/types';

	const t = useTranslation();

	type ExpenseCategory = 'court_rental' | 'equipment' | 'instructor' | 'custom';

	interface Expense {
		category: ExpenseCategory;
		description: string;
		amount_vnd: number;
	}

	// Pagination state from URL
	let currentPage = $state(1);
	let perPage = $state(10);
	let searchQuery = $state('');
	let statusFilter = $state<string>('all');
	let sortBy = $state<string | undefined>(undefined);
	let sortOrder = $state<'asc' | 'desc' | undefined>(undefined);

	// Data state
	let sessions = $state<AdminSession[]>([]);
	let pageInfo = $state<PageInfo | null>(null);
	let loading = $state(true);
	let showSkeleton = $state(false);
	let error = $state<string | null>(null);

	// Debounce timer
	let searchTimeout: ReturnType<typeof setTimeout> | null = null;

	// Delete dialog state
	let deleteDialogOpen = $state(false);
	let sessionToDelete = $state<AdminSession | null>(null);
	let deleteError = $state<string | null>(null);
	let deleting = $state(false);

	// Session dialog state (for both create and edit)
	let sessionDialogOpen = $state(false);
	let isCreating = $state(false);
	let editingSession = $state<AdminSession | null>(null);
	let sessionSaving = $state(false);
	let sessionError = $state<string | null>(null);
	let showExpenses = $state(false);

	// Session form values
	let formTitle = $state('');
	let formDescription = $state('');
	let formLocation = $state('');
	let formStartTime = $state('');
	let formEndTime = $state('');
	let formMaxSlots = $state(8);
	let formPriceVnd = $state<number>(100000);
	let formEarlyAccessEndsAt = $state('');
	let formExpenses = $state<Expense[]>([]);

	const categoryOptions = [
		{ value: 'court_rental', label: () => t('organizer.createSession.expenses.courtRental') },
		{ value: 'equipment', label: () => t('organizer.createSession.expenses.equipment') },
		{ value: 'instructor', label: () => t('organizer.createSession.expenses.instructor') },
		{ value: 'custom', label: () => t('organizer.createSession.expenses.custom') }
	];

	// Calculate total expenses
	let totalExpenses = $derived(
		formExpenses.reduce((sum, expense) => sum + (expense.amount_vnd || 0), 0)
	);

	// Initialize from URL params
	function initFromUrl() {
		const params = $page.url.searchParams;
		currentPage = parseInt(params.get('page') || '1', 10);
		perPage = parseInt(params.get('per_page') || '10', 10);
		searchQuery = params.get('search') || '';
		statusFilter = params.get('status') || 'all';
		sortBy = params.get('sort_by') || undefined;
		sortOrder = (params.get('sort_order') as 'asc' | 'desc') || undefined;
	}

	// Update URL with current state
	function updateUrl() {
		const params = new URLSearchParams();
		if (currentPage > 1) params.set('page', currentPage.toString());
		if (perPage !== 10) params.set('per_page', perPage.toString());
		if (searchQuery) params.set('search', searchQuery);
		if (statusFilter && statusFilter !== 'all') params.set('status', statusFilter);
		if (sortBy) params.set('sort_by', sortBy);
		if (sortOrder) params.set('sort_order', sortOrder);

		const newUrl = params.toString() ? `?${params.toString()}` : '/admin/sessions';
		goto(newUrl, { replaceState: true, keepFocus: true });
	}

	// Check if current user can modify a session (admin can modify all, organizer can only modify their own)
	function canModifySession(session: AdminSession): boolean {
		if (authStore.isAdmin) return true;
		return session.organizer_id === authStore.user?.id;
	}

	onMount(async () => {
		if (!requireRole('organizer')) return;

		initFromUrl();

		const skeletonTimer = setTimeout(() => {
			if (loading) showSkeleton = true;
		}, 200);

		await loadSessions();
		clearTimeout(skeletonTimer);

		// Check for action=create query param and open dialog
		const params = $page.url.searchParams;
		if (params.get('action') === 'create') {
			openCreateDialog();
			// Clear the action param from URL
			goto('/admin/sessions', { replaceState: true });
		}
	});

	async function loadSessions() {
		loading = true;
		error = null;

		try {
			const response = await api.admin.listSessions({
				page: currentPage,
				per_page: perPage,
				search: searchQuery || undefined,
				status: statusFilter === 'all' ? undefined : statusFilter,
				sort_by: sortBy,
				sort_order: sortOrder
			});
			sessions = response.data.data;
			pageInfo = response.data.page_info;
		} catch (err: any) {
			error = extractErrorMessage(err, 'Failed to load sessions');
		} finally {
			loading = false;
		}
	}

	// Handle search with debounce
	function handleSearchInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		searchQuery = value;

		if (searchTimeout) clearTimeout(searchTimeout);
		searchTimeout = setTimeout(() => {
			currentPage = 1;
			updateUrl();
			loadSessions();
		}, 300);
	}

	function clearSearch() {
		searchQuery = '';
		currentPage = 1;
		updateUrl();
		loadSessions();
	}

	// Handle filter changes
	function handleStatusFilter(value: string | undefined) {
		statusFilter = value || 'all';
		currentPage = 1;
		updateUrl();
		loadSessions();
	}

	// Handle sorting
	function handleSort(column: string) {
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
		loadSessions();
	}

	function getSortIcon(column: string) {
		if (sortBy !== column) return ArrowUpDown;
		return sortOrder === 'asc' ? ArrowUp : ArrowDown;
	}

	// Handle pagination
	function handlePageChange(newPage: number) {
		currentPage = newPage;
		updateUrl();
		loadSessions();
	}

	function openDeleteDialog(session: AdminSession) {
		sessionToDelete = session;
		deleteError = null;
		deleteDialogOpen = true;
	}

	async function confirmDeleteSession() {
		if (!sessionToDelete) return;

		deleting = true;
		deleteError = null;

		try {
			await api.sessions.delete(sessionToDelete.id);
			deleteDialogOpen = false;
			sessionToDelete = null;
			await loadSessions();
		} catch (err: any) {
			deleteError = extractErrorMessage(err, t('admin.sessions.deleteFailed'));
		} finally {
			deleting = false;
		}
	}

	// Reset form to defaults
	function resetForm() {
		formTitle = '';
		formDescription = '';
		formLocation = '';
		formStartTime = '';
		formEndTime = '';
		formMaxSlots = 8;
		formPriceVnd = 100000;
		formEarlyAccessEndsAt = '';
		formExpenses = [];
		showExpenses = false;
		sessionError = null;
	}

	// Open create dialog
	function openCreateDialog() {
		isCreating = true;
		editingSession = null;
		resetForm();
		sessionDialogOpen = true;
	}

	// Open edit dialog - fetch full session details including expenses
	async function openEditDialog(session: AdminSession) {
		isCreating = false;
		editingSession = session;
		sessionError = null;
		sessionDialogOpen = true;

		// Set initial values from the list data
		formTitle = session.title;
		formDescription = session.description || '';
		formLocation = session.location;
		formStartTime = `${session.date}T${session.time}`;
		const startDate = new Date(`${session.date}T${session.time}`);
		startDate.setHours(startDate.getHours() + 2);
		formEndTime = startDate.toISOString().slice(0, 16);
		formMaxSlots = session.total_slots;
		formPriceVnd = session.price_vnd || 100000;
		formEarlyAccessEndsAt = session.early_access_ends_at || '';
		formExpenses = [];
		showExpenses = false;

		// Fetch full session details to get expenses
		try {
			const response = await api.sessions.get(session.id);
			const fullSession = response.data;

			// Update with full session data
			if (fullSession.description) {
				formDescription = fullSession.description;
			}

			// Load expenses if available
			if (fullSession.expenses && fullSession.expenses.length > 0) {
				formExpenses = fullSession.expenses.map((e: any) => ({
					category: e.category as ExpenseCategory,
					description: e.description || '',
					amount_vnd: e.amount_vnd
				}));
				showExpenses = true;
			}
		} catch (err) {
			console.warn('Could not fetch full session details:', err);
			// Continue with basic data from the list
		}
	}

	function closeSessionDialog() {
		sessionDialogOpen = false;
		editingSession = null;
		isCreating = false;
		sessionError = null;
	}

	// Expense management functions
	function addExpense() {
		formExpenses = [...formExpenses, { category: 'court_rental', description: '', amount_vnd: 0 }];
		showExpenses = true;
	}

	function removeExpense(index: number) {
		formExpenses = formExpenses.filter((_, i) => i !== index);
	}

	async function saveSession() {
		// Validate times
		const startTime = new Date(formStartTime);
		const endTime = new Date(formEndTime);

		if (startTime >= endTime) {
			sessionError = t('organizer.createSession.errors.endTimeAfterStart');
			return;
		}

		if (isCreating && startTime < new Date()) {
			sessionError = t('organizer.createSession.errors.startTimeInFuture');
			return;
		}

		// Validate expenses
		for (const expense of formExpenses) {
			if (expense.category === 'custom' && !expense.description.trim()) {
				sessionError = t('organizer.createSession.errors.customExpenseDescription');
				return;
			}
			if (expense.amount_vnd <= 0) {
				sessionError = t('organizer.createSession.errors.expenseAmountPositive');
				return;
			}
		}

		sessionSaving = true;
		sessionError = null;

		try {
			const payload: any = {
				title: formTitle,
				location: formLocation,
				start_time: formStartTime,
				end_time: formEndTime,
				max_slots: formMaxSlots,
				price_vnd: formPriceVnd
			};

			if (formDescription) {
				payload.description = formDescription;
			}

			if (formEarlyAccessEndsAt) {
				payload.early_access_ends_at = formEarlyAccessEndsAt;
			}

			// Add expenses if any
			if (formExpenses.length > 0) {
				payload.expenses = formExpenses.map(expense => ({
					category: expense.category,
					description: expense.description || undefined,
					cost_type: 'total',
					amount_vnd: expense.amount_vnd
				}));
			}

			if (isCreating) {
				await api.sessions.create(payload);
			} else if (editingSession) {
				await api.sessions.update(editingSession.id, payload);
			}

			await loadSessions();
			closeSessionDialog();
		} catch (err: any) {
			const defaultError = isCreating
				? t('organizer.createSession.errors.createFailed')
				: t('admin.sessions.updateFailed');
			sessionError = extractErrorMessage(err, defaultError);
		} finally {
			sessionSaving = false;
		}
	}
</script>

<svelte:head>
	<title>{t('admin.sessions.pageTitle')} - Loafy Club</title>
</svelte:head>

<AdminLayout>
	<AnimatedContainer animation="fade-up">
		<SectionHeader title={t('admin.sessions.title')} subtitle={t('admin.sessions.subtitle')}>
			{#snippet actions()}
				<Button variant="gradient" onclick={openCreateDialog}>
					<Plus class="size-4 mr-2" />
					{t('admin.sessions.createSession')}
				</Button>
			{/snippet}
		</SectionHeader>
	</AnimatedContainer>

	{#if error}
		<Alert variant="destructive" class="mb-4">
			<AlertDescription>{error}</AlertDescription>
		</Alert>
	{/if}

	<!-- Search and Filters -->
	<AnimatedContainer animation="fade-up" delay={50}>
		<Card variant="glass" class="mb-4">
			<div class="flex flex-wrap items-center gap-4">
				<!-- Search -->
				<div class="relative flex-1 min-w-[200px]">
					<Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
					<Input
						type="text"
						placeholder={t('admin.sessions.searchPlaceholder')}
						value={searchQuery}
						oninput={handleSearchInput}
						class="pl-9 pr-9"
					/>
					{#if searchQuery}
						<button
							onclick={clearSearch}
							class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
						>
							<X class="h-4 w-4" />
						</button>
					{/if}
				</div>

				<!-- Status Filter -->
				<Select.Root
					type="single"
					value={statusFilter}
					onValueChange={handleStatusFilter}
				>
					<Select.Trigger class="w-[150px]">
						<span class="text-muted-foreground">{t('admin.sessions.statusFilter')}:</span>
						<span class="ml-1 capitalize">{statusFilter === 'all' ? t('common.all') : statusFilter}</span>
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="all">{t('common.all')}</Select.Item>
						<Select.Item value="active">{t('admin.sessions.status.active')}</Select.Item>
						<Select.Item value="cancelled">{t('admin.sessions.status.cancelled')}</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>
		</Card>
	</AnimatedContainer>

	{#if loading && showSkeleton}
		<TableSkeleton rows={5} cols={6} />
	{:else if loading}
		<!-- Brief loading -->
	{:else if sessions.length === 0}
		<AnimatedContainer animation="fade-up" delay={100}>
			<Card variant="glass" class="mx-auto max-w-md">
				<Empty.Root>
					<Empty.Header>
						<Empty.Media variant="icon">
							<Calendar class="size-5" />
						</Empty.Media>
						<Empty.Title>{t('admin.sessions.noSessions')}</Empty.Title>
						<Empty.Description>{t('admin.sessions.noSessionsDesc')}</Empty.Description>
					</Empty.Header>
					<Empty.Content>
						<Button onclick={openCreateDialog}>
							<Plus class="size-4 mr-2" />
							{t('admin.sessions.createSession')}
						</Button>
					</Empty.Content>
				</Empty.Root>
			</Card>
		</AnimatedContainer>
	{:else}
		<AnimatedContainer animation="fade-up" delay={100}>
			<Card variant="glass" class="!p-0">
				<div class="overflow-x-auto overflow-y-hidden rounded-[2rem]">
					<Table.Root>
						<Table.Header>
							<Table.Row>
								<Table.Head>
									<button
										onclick={() => handleSort('title')}
										class="flex items-center gap-1 hover:text-foreground"
									>
										{t('admin.sessions.table.session')}
										<svelte:component this={getSortIcon('title')} class="h-4 w-4" />
									</button>
								</Table.Head>
								<Table.Head>
									<button
										onclick={() => handleSort('date')}
										class="flex items-center gap-1 hover:text-foreground"
									>
										{t('admin.sessions.table.dateTime')}
										<svelte:component this={getSortIcon('date')} class="h-4 w-4" />
									</button>
								</Table.Head>
								<Table.Head>{t('admin.sessions.table.participants')}</Table.Head>
								<Table.Head>
									<button
										onclick={() => handleSort('price_vnd')}
										class="flex items-center gap-1 hover:text-foreground"
									>
										{t('admin.sessions.table.price')}
										<svelte:component this={getSortIcon('price_vnd')} class="h-4 w-4" />
									</button>
								</Table.Head>
								<Table.Head>{t('admin.sessions.table.status')}</Table.Head>
								<Table.Head class="text-right">{t('admin.sessions.table.actions')}</Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each sessions as session}
								<Table.Row>
									<Table.Cell>
										<div class="font-medium text-foreground">{session.title}</div>
										<div class="text-sm text-muted-foreground">{session.location}</div>
									</Table.Cell>
									<Table.Cell class="text-muted-foreground">
										{session.date} {session.time}
									</Table.Cell>
									<Table.Cell>
										<SessionParticipants session={session} variant="default" />
									</Table.Cell>
									<Table.Cell class="font-medium text-foreground">
										{session.price_vnd ? formatCurrency(session.price_vnd) : '-'}
									</Table.Cell>
									<Table.Cell>
										<Badge variant={session.cancelled ? 'cancelled' : 'confirmed'}>
											{session.cancelled ? t('admin.sessions.status.cancelled') : t('admin.sessions.status.active')}
										</Badge>
									</Table.Cell>
									<Table.Cell class="text-right">
										<DropdownMenu.Root>
											<DropdownMenu.Trigger>
												<Button variant="ghost" size="icon" class="h-8 w-8">
													<MoreHorizontal class="h-4 w-4" />
												</Button>
											</DropdownMenu.Trigger>
											<DropdownMenu.Content align="end">
												{#if canModifySession(session)}
													<DropdownMenu.Item onclick={() => openEditDialog(session)}>
														<Pencil class="mr-2 h-4 w-4" />
														{t('admin.sessions.actions.edit')}
													</DropdownMenu.Item>
												{/if}
												<DropdownMenu.Item onclick={() => goto(`/sessions/${session.id}`)}>
													<ExternalLink class="mr-2 h-4 w-4" />
													{t('admin.sessions.actions.view')}
												</DropdownMenu.Item>
												{#if canModifySession(session)}
													<DropdownMenu.Separator />
													<DropdownMenu.Item
														class="text-destructive focus:text-destructive"
														onclick={() => openDeleteDialog(session)}
													>
														<Trash2 class="mr-2 h-4 w-4 text-destructive" />
														{t('admin.sessions.actions.delete')}
													</DropdownMenu.Item>
												{/if}
											</DropdownMenu.Content>
										</DropdownMenu.Root>
									</Table.Cell>
								</Table.Row>
							{/each}
						</Table.Body>
					</Table.Root>
				</div>
			</Card>
		</AnimatedContainer>

		<!-- Pagination -->
		{#if pageInfo && pageInfo.total_pages > 1}
			<AnimatedContainer animation="fade-up" delay={150}>
				<div class="mt-4">
					<Pagination {pageInfo} onPageChange={handlePageChange} />
				</div>
			</AnimatedContainer>
		{:else if pageInfo}
			<AnimatedContainer animation="fade-up" delay={150}>
				<p class="mt-4 text-sm text-muted-foreground">
					{t('admin.sessions.totalCount', { count: pageInfo.total })}
				</p>
			</AnimatedContainer>
		{/if}
	{/if}
</AdminLayout>

<!-- Delete Session Dialog -->
<AlertDialog.Root bind:open={deleteDialogOpen}>
	<AlertDialog.Content interactOutsideBehavior="close">
		<AlertDialog.Header>
			<AlertDialog.Title>{t('admin.sessions.deleteDialogTitle')}</AlertDialog.Title>
			<AlertDialog.Description>
				{t('admin.sessions.confirmDelete', { title: sessionToDelete?.title || '' })}
			</AlertDialog.Description>
		</AlertDialog.Header>

		{#if deleteError}
			<Alert variant="destructive">
				<AlertDescription>{deleteError}</AlertDescription>
			</Alert>
		{/if}

		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={deleting}>
				{t('common.cancel')}
			</AlertDialog.Cancel>
			<Button
				variant="destructive"
				onclick={confirmDeleteSession}
				disabled={deleting}
			>
				{deleting ? t('admin.sessions.deleting') : t('admin.sessions.confirmDeleteButton')}
			</Button>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

<!-- Session Create/Edit Dialog -->
<AlertDialog.Root bind:open={sessionDialogOpen}>
	<AlertDialog.Content class="max-w-2xl max-h-[90vh] flex flex-col overflow-hidden" interactOutsideBehavior="close">
		<AlertDialog.Header class="flex-shrink-0">
			<AlertDialog.Title>
				{isCreating ? t('organizer.createSession.title') : t('admin.sessions.edit.title')}
			</AlertDialog.Title>
			<AlertDialog.Description>
				{#if isCreating}
					{t('organizer.createSession.subtitle')}
				{:else if editingSession}
					{t('admin.sessions.edit.description', { title: editingSession.title })}
				{/if}
			</AlertDialog.Description>
		</AlertDialog.Header>

		<div class="flex-1 overflow-y-auto">
			{#if sessionError}
				<Alert variant="destructive" class="mb-4">
					<AlertDescription>{sessionError}</AlertDescription>
				</Alert>
			{/if}

			<div class="space-y-4 py-4">
			<!-- Title -->
			<div class="space-y-2">
				<Label for="form_title">
					{t('organizer.createSession.form.title')} <span class="text-destructive">*</span>
				</Label>
				<Input
					id="form_title"
					bind:value={formTitle}
					placeholder={t('organizer.createSession.form.titlePlaceholder')}
					required
				/>
			</div>

			<!-- Description -->
			<div class="space-y-2">
				<Label for="form_description">
					{t('organizer.createSession.form.description')}
				</Label>
				<Textarea
					id="form_description"
					bind:value={formDescription}
					placeholder={t('organizer.createSession.form.descriptionPlaceholder')}
					rows={2}
				/>
			</div>

			<!-- Location -->
			<div class="space-y-2">
				<Label for="form_location">
					{t('organizer.createSession.form.location')} <span class="text-destructive">*</span>
				</Label>
				<Input
					id="form_location"
					bind:value={formLocation}
					placeholder={t('organizer.createSession.form.locationPlaceholder')}
					required
				/>
			</div>

			<!-- Start Time -->
			<div class="space-y-2">
				<Label for="form_start_time">
					{t('organizer.createSession.form.startTime')} <span class="text-destructive">*</span>
				</Label>
				<DateTimePicker
					id="form_start_time"
					bind:value={formStartTime}
					placeholder={t('organizer.createSession.form.selectStartTime')}
					required
				/>
			</div>

			<!-- End Time -->
			<div class="space-y-2">
				<Label for="form_end_time">
					{t('organizer.createSession.form.endTime')} <span class="text-destructive">*</span>
				</Label>
				<DateTimePicker
					id="form_end_time"
					bind:value={formEndTime}
					placeholder={t('organizer.createSession.form.selectEndTime')}
					required
				/>
			</div>

			<!-- Max Slots and Price Row -->
			<div class="grid grid-cols-2 gap-4">
				<div class="space-y-2">
					<Label for="form_max_slots">
						{t('organizer.createSession.form.maxSlots')} <span class="text-destructive">*</span>
					</Label>
					<Input
						id="form_max_slots"
						type="number"
						min="1"
						max="100"
						bind:value={formMaxSlots}
						required
					/>
				</div>
				<div class="space-y-2">
					<Label for="form_price">
						{t('organizer.createSession.form.priceVnd')} <span class="text-destructive">*</span>
					</Label>
					<Input
						id="form_price"
						type="number"
						min="0"
						step="1000"
						bind:value={formPriceVnd}
						required
					/>
				</div>
			</div>

			<!-- Early Access -->
			<div class="space-y-2">
				<Label for="form_early_access">
					{t('organizer.createSession.form.earlyAccess')}
				</Label>
				<DateTimePicker
					id="form_early_access"
					bind:value={formEarlyAccessEndsAt}
					placeholder={t('organizer.createSession.form.selectEarlyAccess')}
				/>
				<p class="text-xs text-muted-foreground">
					{t('organizer.createSession.form.earlyAccessNote')}
				</p>
			</div>

			<!-- Expenses Section -->
			<div class="border-t border-border pt-4 mt-2">
				<button
					type="button"
					class="flex w-full items-center justify-between text-left"
					onclick={() => showExpenses = !showExpenses}
				>
					<div>
						<span class="text-sm font-medium text-foreground">
							{t('organizer.createSession.expenses.title')}
						</span>
						<span class="ml-2 text-xs text-muted-foreground">
							({t('organizer.createSession.expenses.optional')})
						</span>
					</div>
					{#if showExpenses}
						<ChevronUp class="size-4 text-muted-foreground" />
					{:else}
						<ChevronDown class="size-4 text-muted-foreground" />
					{/if}
				</button>

				{#if showExpenses}
					<div class="mt-4 space-y-3">
						<p class="text-xs text-muted-foreground">
							{t('organizer.createSession.expenses.description')}
						</p>

						{#each formExpenses as expense, index}
							<div class="rounded-lg border border-border bg-muted/30 p-3">
								<div class="flex flex-wrap gap-3 items-end">
									<div class="flex-1 min-w-[120px]">
										<Label class="text-xs">{t('organizer.createSession.expenses.category')}</Label>
										<Select.Root type="single" value={expense.category} onValueChange={(v) => v && (expense.category = v as ExpenseCategory)}>
											<Select.Trigger class="mt-1 h-9">
												{categoryOptions.find(o => o.value === expense.category)?.label() ?? t('organizer.createSession.expenses.selectCategory')}
											</Select.Trigger>
											<Select.Content>
												{#each categoryOptions as option}
													<Select.Item value={option.value}>{option.label()}</Select.Item>
												{/each}
											</Select.Content>
										</Select.Root>
									</div>

									{#if expense.category === 'custom'}
										<div class="flex-1 min-w-[120px]">
											<Label class="text-xs">{t('organizer.createSession.expenses.descriptionLabel')}</Label>
											<Input
												type="text"
												bind:value={expense.description}
												placeholder={t('organizer.createSession.expenses.descriptionPlaceholder')}
												class="mt-1 h-9"
											/>
										</div>
									{/if}

									<div class="flex-1 min-w-[100px]">
										<Label class="text-xs">{t('organizer.createSession.expenses.amount')}</Label>
										<Input
											type="number"
											bind:value={expense.amount_vnd}
											min="0"
											step="1000"
											placeholder="0"
											class="mt-1 h-9"
										/>
									</div>

									<Button
										type="button"
										variant="ghost"
										size="icon"
										class="h-9 w-9 text-destructive hover:bg-destructive/10"
										onclick={() => removeExpense(index)}
									>
										<Trash2 class="size-4" />
									</Button>
								</div>
							</div>
						{/each}

						<Button type="button" variant="outline" size="sm" onclick={addExpense}>
							<Plus class="size-4 mr-2" />
							{t('organizer.createSession.expenses.addExpense')}
						</Button>

						{#if formExpenses.length > 0}
							<div class="rounded-lg bg-muted/50 p-2 text-sm">
								<span class="text-muted-foreground">{t('organizer.createSession.expenses.estimatedTotal')}:</span>
								<span class="ml-2 font-medium text-foreground">{formatCurrency(totalExpenses)}</span>
							</div>
						{/if}
					</div>
				{:else if formExpenses.length === 0}
					<Button type="button" variant="outline" size="sm" class="mt-2" onclick={addExpense}>
						<Plus class="size-4 mr-2" />
						{t('organizer.createSession.expenses.addExpense')}
					</Button>
				{:else}
					<p class="mt-2 text-xs text-muted-foreground">
						{formExpenses.length} {t('organizer.createSession.expenses.expensesAdded')} - {formatCurrency(totalExpenses)}
					</p>
				{/if}
			</div>
			</div>
		</div>

		<AlertDialog.Footer class="flex-shrink-0">
			<AlertDialog.Cancel onclick={closeSessionDialog}>
				{t('common.cancel')}
			</AlertDialog.Cancel>
			<AlertDialog.Action onclick={saveSession} disabled={sessionSaving}>
				{#if sessionSaving}
					{isCreating ? t('organizer.createSession.buttons.creating') : t('common.saving')}
				{:else}
					{isCreating ? t('organizer.createSession.buttons.create') : t('common.save')}
				{/if}
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

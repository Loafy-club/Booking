<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatCurrency, extractErrorMessage } from '$lib/utils';
	import { getBookingBadgeVariant, getBookingStatusKey } from '$lib/utils/status';
	import { requireRole } from '$lib/guards/auth';
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
	import * as Table from '$lib/components/ui/table';
	import * as Select from '$lib/components/ui/select';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Pagination } from '$lib/components/ui/pagination';
	import { Textarea } from '$lib/components/ui/textarea';
	import { CreditCard, Search, X, ArrowUpDown, ArrowUp, ArrowDown, Pencil, MoreHorizontal, ExternalLink } from 'lucide-svelte';
	import type { AdminBooking, PageInfo, UpdateBookingRequest } from '$lib/types';

	const t = useTranslation();

	// Pagination state from URL
	let currentPage = $state(1);
	let perPage = $state(10);
	let searchQuery = $state('');
	let statusFilter = $state<string>('all');
	let sortBy = $state<string | undefined>(undefined);
	let sortOrder = $state<'asc' | 'desc' | undefined>(undefined);

	// Data state
	let bookings = $state<AdminBooking[]>([]);
	let pageInfo = $state<PageInfo | null>(null);
	let loading = $state(true);
	let showSkeleton = $state(false);
	let error = $state<string | null>(null);

	// Debounce timer
	let searchTimeout: ReturnType<typeof setTimeout> | null = null;

	// Edit dialog state
	let editDialogOpen = $state(false);
	let editingBooking = $state<AdminBooking | null>(null);
	let editSaving = $state(false);
	let editError = $state<string | null>(null);

	// Edit form values
	let editGuestCount = $state<number>(0);
	let editPricePaidVnd = $state<number>(0);
	let editGuestPricePaidVnd = $state<number>(0);
	let editPaymentMethod = $state<string>('');
	let editPaymentStatus = $state<string>('');
	let editAdminNotes = $state<string>('');

	// Initialize from URL params
	function initFromUrl() {
		const params = $page.url.searchParams;
		currentPage = parseInt(params.get('page') || '1', 10);
		perPage = parseInt(params.get('per_page') || '10', 10);
		searchQuery = params.get('search') || '';
		statusFilter = params.get('payment_status') || 'all';
		sortBy = params.get('sort_by') || undefined;
		sortOrder = (params.get('sort_order') as 'asc' | 'desc') || undefined;
	}

	// Update URL with current state
	function updateUrl() {
		const params = new URLSearchParams();
		if (currentPage > 1) params.set('page', currentPage.toString());
		if (perPage !== 10) params.set('per_page', perPage.toString());
		if (searchQuery) params.set('search', searchQuery);
		if (statusFilter && statusFilter !== 'all') params.set('payment_status', statusFilter);
		if (sortBy) params.set('sort_by', sortBy);
		if (sortOrder) params.set('sort_order', sortOrder);

		const newUrl = params.toString() ? `?${params.toString()}` : '/admin/bookings';
		goto(newUrl, { replaceState: true, keepFocus: true });
	}

	onMount(async () => {
		if (!requireRole('admin')) return;

		initFromUrl();

		const skeletonTimer = setTimeout(() => {
			if (loading) showSkeleton = true;
		}, 200);

		await loadBookings();
		clearTimeout(skeletonTimer);
	});

	async function loadBookings() {
		loading = true;
		error = null;

		try {
			const response = await api.admin.listBookings({
				page: currentPage,
				per_page: perPage,
				search: searchQuery || undefined,
				payment_status: statusFilter === 'all' ? undefined : statusFilter,
				sort_by: sortBy,
				sort_order: sortOrder
			});
			bookings = response.data.data;
			pageInfo = response.data.page_info;
		} catch (err: any) {
			error = extractErrorMessage(err, t('admin.bookings.loadError'));
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
			loadBookings();
		}, 300);
	}

	function clearSearch() {
		searchQuery = '';
		currentPage = 1;
		updateUrl();
		loadBookings();
	}

	// Handle filter changes
	function handleStatusFilter(value: string | undefined) {
		statusFilter = value || 'all';
		currentPage = 1;
		updateUrl();
		loadBookings();
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
		loadBookings();
	}

	function getSortIcon(column: string) {
		if (sortBy !== column) return ArrowUpDown;
		return sortOrder === 'asc' ? ArrowUp : ArrowDown;
	}

	// Handle pagination
	function handlePageChange(newPage: number) {
		currentPage = newPage;
		updateUrl();
		loadBookings();
	}

	function getStatusLabel(status: string, cancelled: boolean): string {
		return t(`admin.bookings.status.${getBookingStatusKey(status, cancelled)}`);
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString();
	}

	function formatDateTime(dateString: string): string {
		return new Date(dateString).toLocaleString();
	}

	// Edit dialog functions
	function openEditDialog(booking: AdminBooking) {
		editingBooking = booking;
		editGuestCount = booking.guest_count;
		// We need to split total_price_vnd - assume base price is total minus guest portion
		// For now, we'll set base price to total and guest price to 0
		// This could be improved with actual price data from the booking
		editPricePaidVnd = booking.total_price_vnd;
		editGuestPricePaidVnd = 0;
		editPaymentMethod = booking.payment_method;
		editPaymentStatus = booking.payment_status;
		editAdminNotes = '';
		editError = null;
		editDialogOpen = true;
	}

	function closeEditDialog() {
		editDialogOpen = false;
		editingBooking = null;
		editError = null;
	}

	async function saveBookingChanges() {
		if (!editingBooking) return;

		editSaving = true;
		editError = null;

		try {
			const updateData: UpdateBookingRequest = {};

			// Only include changed fields
			if (editGuestCount !== editingBooking.guest_count) {
				updateData.guest_count = editGuestCount;
			}
			if (editPricePaidVnd !== editingBooking.total_price_vnd) {
				updateData.price_paid_vnd = editPricePaidVnd;
				updateData.guest_price_paid_vnd = editGuestPricePaidVnd;
			}
			if (editPaymentMethod !== editingBooking.payment_method) {
				updateData.payment_method = editPaymentMethod;
			}
			if (editPaymentStatus !== editingBooking.payment_status) {
				updateData.payment_status = editPaymentStatus;
			}
			if (editAdminNotes.trim()) {
				updateData.admin_notes = editAdminNotes.trim();
			}

			await api.admin.updateBooking(editingBooking.id, updateData);

			// Reload bookings to show updated data
			await loadBookings();
			closeEditDialog();
		} catch (err: any) {
			editError = extractErrorMessage(err, t('admin.bookings.edit.saveError'));
		} finally {
			editSaving = false;
		}
	}
</script>

<svelte:head>
	<title>{t('admin.bookings.pageTitle')} - Loafy Club</title>
</svelte:head>

<AdminLayout>
	<AnimatedContainer animation="fade-up">
		<SectionHeader title={t('admin.bookings.title')} subtitle={t('admin.bookings.subtitle')} />
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
						placeholder={t('admin.bookings.searchPlaceholder')}
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
					<Select.Trigger class="w-[160px]">
						<span class="text-muted-foreground">{t('admin.bookings.statusFilter')}:</span>
						<span class="ml-1 capitalize">{statusFilter === 'all' ? t('common.all') : statusFilter}</span>
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="all">{t('common.all')}</Select.Item>
						<Select.Item value="pending">{t('admin.bookings.status.pending')}</Select.Item>
						<Select.Item value="confirmed">{t('admin.bookings.status.confirmed')}</Select.Item>
						<Select.Item value="cancelled">{t('admin.bookings.status.cancelled')}</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>
		</Card>
	</AnimatedContainer>

	{#if loading && showSkeleton}
		<TableSkeleton rows={5} cols={8} />
	{:else if loading}
		<!-- Brief loading -->
	{:else if bookings.length === 0}
		<AnimatedContainer animation="fade-up" delay={100}>
			<Card variant="glass" class="mx-auto max-w-md">
				<Empty.Root>
					<Empty.Header>
						<Empty.Media variant="icon">
							<CreditCard class="size-5" />
						</Empty.Media>
						<Empty.Title>{t('admin.bookings.noBookings')}</Empty.Title>
						<Empty.Description>{t('admin.bookings.noBookingsDesc')}</Empty.Description>
					</Empty.Header>
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
										onclick={() => handleSort('booking_code')}
										class="flex items-center gap-1 hover:text-foreground"
									>
										{t('admin.bookings.table.code')}
										<svelte:component this={getSortIcon('booking_code')} class="h-4 w-4" />
									</button>
								</Table.Head>
								<Table.Head>{t('admin.bookings.table.user')}</Table.Head>
								<Table.Head>{t('admin.bookings.table.session')}</Table.Head>
								<Table.Head>{t('admin.bookings.table.guests')}</Table.Head>
								<Table.Head>
									<button
										onclick={() => handleSort('total_price_vnd')}
										class="flex items-center gap-1 hover:text-foreground"
									>
										{t('admin.bookings.table.amount')}
										<svelte:component this={getSortIcon('total_price_vnd')} class="h-4 w-4" />
									</button>
								</Table.Head>
								<Table.Head>{t('admin.bookings.table.status')}</Table.Head>
								<Table.Head>
									<button
										onclick={() => handleSort('created_at')}
										class="flex items-center gap-1 hover:text-foreground"
									>
										{t('admin.bookings.table.created')}
										<svelte:component this={getSortIcon('created_at')} class="h-4 w-4" />
									</button>
								</Table.Head>
								<Table.Head class="text-right">{t('admin.bookings.table.actions')}</Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each bookings as booking}
								<Table.Row>
									<Table.Cell>
										<code class="rounded bg-muted px-2 py-1 text-sm font-mono">
											{booking.booking_code}
										</code>
									</Table.Cell>
									<Table.Cell>
										<div>
											<div class="font-medium text-foreground">{booking.user_name || '-'}</div>
											<div class="text-sm text-muted-foreground">{booking.user_email}</div>
										</div>
									</Table.Cell>
									<Table.Cell>
										<div>
											<div class="font-medium text-foreground">{booking.session_title}</div>
											<div class="text-sm text-muted-foreground">
												{booking.session_date} {booking.session_time}
											</div>
										</div>
									</Table.Cell>
									<Table.Cell class="text-muted-foreground">
										{booking.guest_count + 1}
									</Table.Cell>
									<Table.Cell class="font-medium text-foreground">
										{formatCurrency(booking.total_price_vnd)}
									</Table.Cell>
									<Table.Cell>
										<Badge variant={getBookingBadgeVariant(booking.payment_status, !!booking.cancelled_at)}>
											{getStatusLabel(booking.payment_status, !!booking.cancelled_at)}
										</Badge>
										{#if booking.payment_status === 'pending' && !booking.cancelled_at && booking.payment_deadline}
											<div class="mt-1 text-xs text-muted-foreground">
												{t('admin.bookings.deadline')}: {formatDateTime(booking.payment_deadline)}
											</div>
										{/if}
									</Table.Cell>
									<Table.Cell class="text-muted-foreground">
										{formatDate(booking.created_at)}
									</Table.Cell>
									<Table.Cell class="text-right">
										<DropdownMenu.Root>
											<DropdownMenu.Trigger>
												<Button variant="ghost" size="icon" class="h-8 w-8">
													<MoreHorizontal class="h-4 w-4" />
												</Button>
											</DropdownMenu.Trigger>
											<DropdownMenu.Content align="end">
												<DropdownMenu.Item onclick={() => openEditDialog(booking)}>
													<Pencil class="mr-2 h-4 w-4" />
													{t('admin.bookings.actions.edit')}
												</DropdownMenu.Item>
												<DropdownMenu.Item onclick={() => goto(`/sessions/${booking.session_id}`)}>
													<ExternalLink class="mr-2 h-4 w-4" />
													{t('admin.bookings.actions.viewSession')}
												</DropdownMenu.Item>
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
					{t('admin.bookings.totalCount', { count: pageInfo.total })}
				</p>
			</AnimatedContainer>
		{/if}
	{/if}

	<!-- Edit Booking Dialog -->
	<AlertDialog.Root bind:open={editDialogOpen}>
		<AlertDialog.Content class="max-w-lg" interactOutsideBehavior="close">
			<AlertDialog.Header>
				<AlertDialog.Title>{t('admin.bookings.edit.title')}</AlertDialog.Title>
				<AlertDialog.Description>
					{#if editingBooking}
						{t('admin.bookings.edit.description', { code: editingBooking.booking_code })}
					{/if}
				</AlertDialog.Description>
			</AlertDialog.Header>

			{#if editError}
				<Alert variant="destructive" class="mb-4">
					<AlertDescription>{editError}</AlertDescription>
				</Alert>
			{/if}

			{#if editingBooking}
				<div class="grid gap-4 py-4">
					<!-- Guest Count -->
					<div class="grid grid-cols-4 items-center gap-4">
						<Label for="guest_count" class="text-right">
							{t('admin.bookings.edit.guestCount')}
						</Label>
						<Input
							id="guest_count"
							type="number"
							min="0"
							bind:value={editGuestCount}
							class="col-span-3"
						/>
					</div>

					<!-- Price -->
					<div class="grid grid-cols-4 items-center gap-4">
						<Label for="price_paid" class="text-right">
							{t('admin.bookings.edit.pricePaid')}
						</Label>
						<Input
							id="price_paid"
							type="number"
							min="0"
							bind:value={editPricePaidVnd}
							class="col-span-3"
						/>
					</div>

					<!-- Payment Method -->
					<div class="grid grid-cols-4 items-center gap-4">
						<Label class="text-right">
							{t('admin.bookings.edit.paymentMethod')}
						</Label>
						<div class="col-span-3">
							<Select.Root
								type="single"
								value={editPaymentMethod}
								onValueChange={(v) => (editPaymentMethod = v || '')}
							>
								<Select.Trigger class="w-full">
									<span class="capitalize">{editPaymentMethod || t('common.select')}</span>
								</Select.Trigger>
								<Select.Content>
									<Select.Item value="qr">QR</Select.Item>
									<Select.Item value="stripe">Stripe</Select.Item>
									<Select.Item value="cash">Cash</Select.Item>
									<Select.Item value="free">Free</Select.Item>
								</Select.Content>
							</Select.Root>
						</div>
					</div>

					<!-- Payment Status -->
					<div class="grid grid-cols-4 items-center gap-4">
						<Label class="text-right">
							{t('admin.bookings.edit.paymentStatus')}
						</Label>
						<div class="col-span-3">
							<Select.Root
								type="single"
								value={editPaymentStatus}
								onValueChange={(v) => (editPaymentStatus = v || '')}
							>
								<Select.Trigger class="w-full">
									<span class="capitalize">{editPaymentStatus || t('common.select')}</span>
								</Select.Trigger>
								<Select.Content>
									<Select.Item value="pending">{t('admin.bookings.status.pending')}</Select.Item>
									<Select.Item value="confirmed">{t('admin.bookings.status.confirmed')}</Select.Item>
									<Select.Item value="failed">{t('admin.bookings.edit.statusFailed')}</Select.Item>
									<Select.Item value="refunded">{t('admin.bookings.edit.statusRefunded')}</Select.Item>
								</Select.Content>
							</Select.Root>
						</div>
					</div>

					<!-- Admin Notes -->
					<div class="grid grid-cols-4 items-start gap-4">
						<Label for="admin_notes" class="text-right pt-2">
							{t('admin.bookings.edit.adminNotes')}
						</Label>
						<Textarea
							id="admin_notes"
							bind:value={editAdminNotes}
							placeholder={t('admin.bookings.edit.adminNotesPlaceholder')}
							class="col-span-3"
							rows={3}
						/>
					</div>
				</div>
			{/if}

			<AlertDialog.Footer>
				<AlertDialog.Cancel onclick={closeEditDialog}>
					{t('common.cancel')}
				</AlertDialog.Cancel>
				<AlertDialog.Action onclick={saveBookingChanges} disabled={editSaving}>
					{#if editSaving}
						{t('common.saving')}
					{:else}
						{t('common.save')}
					{/if}
				</AlertDialog.Action>
			</AlertDialog.Footer>
		</AlertDialog.Content>
	</AlertDialog.Root>
</AdminLayout>

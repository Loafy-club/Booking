<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { extractErrorMessage } from '$lib/utils';
	import { getRoleBadgeVariant } from '$lib/utils/status';
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
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Avatar, AvatarImage, AvatarFallback } from '$lib/components/ui/avatar';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Label } from '$lib/components/ui/label';
	import { DateTimePicker } from '$lib/components/ui/datetime-picker';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Pagination } from '$lib/components/ui/pagination';
	import { Users, MoreHorizontal, Ban, UserCheck, Info, Search, ArrowUpDown, ArrowUp, ArrowDown, X, Pencil, Trash2, Ticket } from 'lucide-svelte';
	import type { AdminUser, PageInfo } from '$lib/types';

	const t = useTranslation();

	interface Role {
		id: string;
		name: string;
	}

	// Pagination state from URL
	let currentPage = $state(1);
	let perPage = $state(10);
	let searchQuery = $state('');
	let roleFilter = $state<string>('all');
	let statusFilter = $state<string>('all');
	let sortBy = $state<string | undefined>(undefined);
	let sortOrder = $state<'asc' | 'desc' | undefined>(undefined);

	// Data state
	let users = $state<AdminUser[]>([]);
	let pageInfo = $state<PageInfo | null>(null);
	let roles = $state<Role[]>([]);
	let loading = $state(true);
	let showSkeleton = $state(false);
	let error = $state<string | null>(null);
	let updatingUserId = $state<string | null>(null);

	// Debounce timer
	let searchTimeout: ReturnType<typeof setTimeout> | null = null;

	// Suspension dialog state
	let suspendDialogOpen = $state(false);
	let unsuspendDialogOpen = $state(false);
	let selectedUser = $state<AdminUser | null>(null);
	let suspendReason = $state('');
	let suspendUntil = $state('');
	let suspending = $state(false);

	// Edit dialog state
	let editDialogOpen = $state(false);
	let editingUser = $state<AdminUser | null>(null);
	let editName = $state('');
	let editPhone = $state('');
	let editSaving = $state(false);
	let editError = $state<string | null>(null);

	// Delete dialog state
	let deleteDialogOpen = $state(false);
	let userToDelete = $state<AdminUser | null>(null);
	let deleteError = $state<string | null>(null);
	let deleting = $state(false);

	// Ticket dialog state
	let ticketDialogOpen = $state(false);
	let ticketUser = $state<AdminUser | null>(null);
	let ticketAction = $state<'grant' | 'revoke'>('grant');
	let ticketAmount = $state(1);
	let ticketReason = $state('');
	let ticketProcessing = $state(false);
	let ticketError = $state<string | null>(null);
	let userTicketBalance = $state<number | null>(null);

	// Initialize from URL params
	function initFromUrl() {
		const params = $page.url.searchParams;
		currentPage = parseInt(params.get('page') || '1', 10);
		perPage = parseInt(params.get('per_page') || '10', 10);
		searchQuery = params.get('search') || '';
		roleFilter = params.get('role') || 'all';
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
		if (roleFilter && roleFilter !== 'all') params.set('role', roleFilter);
		if (statusFilter && statusFilter !== 'all') params.set('status', statusFilter);
		if (sortBy) params.set('sort_by', sortBy);
		if (sortOrder) params.set('sort_order', sortOrder);

		const newUrl = params.toString() ? `?${params.toString()}` : '/admin/users';
		goto(newUrl, { replaceState: true, keepFocus: true });
	}

	onMount(async () => {
		if (!requireRole('admin')) return;

		initFromUrl();

		const skeletonTimer = setTimeout(() => {
			if (loading) showSkeleton = true;
		}, 200);

		await Promise.all([loadUsers(), loadRoles()]);
		clearTimeout(skeletonTimer);
	});

	async function loadUsers() {
		loading = true;
		error = null;

		try {
			const response = await api.admin.listUsers({
				page: currentPage,
				per_page: perPage,
				search: searchQuery || undefined,
				role: roleFilter === 'all' ? undefined : roleFilter,
				status: statusFilter === 'all' ? undefined : statusFilter,
				sort_by: sortBy,
				sort_order: sortOrder
			});
			users = response.data.data;
			pageInfo = response.data.page_info;
		} catch (err: any) {
			error = extractErrorMessage(err, t('admin.users.loadError'));
		} finally {
			loading = false;
		}
	}

	async function loadRoles() {
		try {
			const response = await api.admin.listRoles();
			roles = response.data;
		} catch (err: any) {
			console.error('Failed to load roles:', err);
		}
	}

	// Handle search with debounce
	function handleSearchInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		searchQuery = value;

		if (searchTimeout) clearTimeout(searchTimeout);
		searchTimeout = setTimeout(() => {
			currentPage = 1; // Reset to first page on search
			updateUrl();
			loadUsers();
		}, 300);
	}

	function clearSearch() {
		searchQuery = '';
		currentPage = 1;
		updateUrl();
		loadUsers();
	}

	// Handle filter changes
	function handleRoleFilter(value: string | undefined) {
		roleFilter = value || 'all';
		currentPage = 1;
		updateUrl();
		loadUsers();
	}

	function handleStatusFilter(value: string | undefined) {
		statusFilter = value || 'all';
		currentPage = 1;
		updateUrl();
		loadUsers();
	}

	// Handle sorting
	function handleSort(column: string) {
		if (sortBy === column) {
			// Toggle order or clear
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
		loadUsers();
	}

	function getSortIcon(column: string) {
		if (sortBy !== column) return ArrowUpDown;
		return sortOrder === 'asc' ? ArrowUp : ArrowDown;
	}

	// Handle pagination
	function handlePageChange(newPage: number) {
		currentPage = newPage;
		updateUrl();
		loadUsers();
	}

	async function updateRole(userId: string, newRole: string) {
		updatingUserId = userId;
		error = null;

		try {
			const response = await api.admin.updateUserRole(userId, newRole);
			// Update the user in the list
			users = users.map((u) => (u.id === userId ? { ...u, role: response.data.role } : u));
		} catch (err: any) {
			error = extractErrorMessage(err, t('admin.users.updateError'));
		} finally {
			updatingUserId = null;
		}
	}

	function openSuspendDialog(user: AdminUser) {
		selectedUser = user;
		suspendReason = '';
		suspendUntil = '';
		suspendDialogOpen = true;
	}

	function openUnsuspendDialog(user: AdminUser) {
		selectedUser = user;
		unsuspendDialogOpen = true;
	}

	async function handleSuspend() {
		if (!selectedUser || !suspendReason.trim()) return;

		suspending = true;
		error = null;

		try {
			const response = await api.admin.suspendUser(selectedUser.id, {
				reason: suspendReason.trim(),
				until: suspendUntil || undefined
			});
			// Update the user in the list
			users = users.map((u) => (u.id === selectedUser!.id ? response.data : u));
			suspendDialogOpen = false;
		} catch (err: any) {
			error = extractErrorMessage(err, t('admin.users.suspendError'));
		} finally {
			suspending = false;
		}
	}

	async function handleUnsuspend() {
		if (!selectedUser) return;

		suspending = true;
		error = null;

		try {
			const response = await api.admin.unsuspendUser(selectedUser.id);
			// Update the user in the list
			users = users.map((u) => (u.id === selectedUser!.id ? response.data : u));
			unsuspendDialogOpen = false;
		} catch (err: any) {
			error = extractErrorMessage(err, t('admin.users.unsuspendError'));
		} finally {
			suspending = false;
		}
	}

	// Edit functions
	function openEditDialog(user: AdminUser) {
		editingUser = user;
		editName = user.name || '';
		editPhone = user.phone || '';
		editError = null;
		editDialogOpen = true;
	}

	function closeEditDialog() {
		editDialogOpen = false;
		editingUser = null;
		editError = null;
	}

	async function handleEditSave() {
		if (!editingUser) return;

		editSaving = true;
		editError = null;

		try {
			const response = await api.admin.updateUser(editingUser.id, {
				name: editName || undefined,
				phone: editPhone || undefined
			});
			// Update the user in the list
			users = users.map((u) => (u.id === editingUser!.id ? response.data : u));
			closeEditDialog();
		} catch (err: any) {
			editError = extractErrorMessage(err, t('admin.users.editError'));
		} finally {
			editSaving = false;
		}
	}

	// Delete functions
	function openDeleteDialog(user: AdminUser) {
		userToDelete = user;
		deleteError = null;
		deleteDialogOpen = true;
	}

	async function handleDelete() {
		if (!userToDelete) return;

		deleting = true;
		deleteError = null;

		try {
			await api.admin.deleteUser(userToDelete.id);
			// Remove the user from the list
			users = users.filter((u) => u.id !== userToDelete!.id);
			deleteDialogOpen = false;
			userToDelete = null;
			// Reload to get updated pagination
			await loadUsers();
		} catch (err: any) {
			deleteError = extractErrorMessage(err, t('admin.users.deleteError'));
		} finally {
			deleting = false;
		}
	}

	// Ticket functions
	async function openTicketDialog(user: AdminUser, action: 'grant' | 'revoke') {
		ticketUser = user;
		ticketAction = action;
		ticketAmount = 1;
		ticketReason = '';
		ticketError = null;
		ticketDialogOpen = true;

		// Load user's current ticket balance
		try {
			const response = await api.admin.getUserTickets(user.id);
			userTicketBalance = response.data.tickets_remaining ?? 0;
		} catch {
			userTicketBalance = null;
		}
	}

	async function handleTicketAction() {
		if (!ticketUser || ticketAmount < 1) return;

		ticketProcessing = true;
		ticketError = null;

		try {
			if (ticketAction === 'grant') {
				await api.admin.grantTickets(ticketUser.id, {
					amount: ticketAmount,
					reason: ticketReason || undefined
				});
			} else {
				await api.admin.revokeTickets(ticketUser.id, {
					amount: ticketAmount,
					reason: ticketReason || undefined
				});
			}
			ticketDialogOpen = false;
		} catch (err: any) {
			ticketError = extractErrorMessage(err, `Failed to ${ticketAction} tickets`);
		} finally {
			ticketProcessing = false;
		}
	}

	function getInitials(name: string | undefined, email: string): string {
		if (name) {
			return name
				.split(' ')
				.map((n) => n[0])
				.join('')
				.toUpperCase()
				.slice(0, 2);
		}
		return email[0].toUpperCase();
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString();
	}
</script>

<svelte:head>
	<title>{t('admin.users.pageTitle')} - Loafy Club</title>
</svelte:head>

<AdminLayout>
	<AnimatedContainer animation="fade-up">
		<SectionHeader title={t('admin.users.title')} subtitle={t('admin.users.subtitle')} />
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
						placeholder={t('admin.users.searchPlaceholder')}
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

				<!-- Role Filter -->
				<Select.Root
					type="single"
					value={roleFilter}
					onValueChange={handleRoleFilter}
				>
					<Select.Trigger class="w-[140px]">
						<span class="text-muted-foreground">{t('admin.users.roleFilter')}:</span>
						<span class="ml-1 capitalize">{roleFilter === 'all' ? t('common.all') : roleFilter}</span>
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="all">{t('common.all')}</Select.Item>
						{#each roles as role}
							<Select.Item value={role.name} class="capitalize">{role.name}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>

				<!-- Status Filter -->
				<Select.Root
					type="single"
					value={statusFilter}
					onValueChange={handleStatusFilter}
				>
					<Select.Trigger class="w-[150px]">
						<span class="text-muted-foreground">{t('admin.users.statusFilter')}:</span>
						<span class="ml-1 capitalize">
							{#if statusFilter === 'all'}
								{t('common.all')}
							{:else if statusFilter === 'active'}
								{t('admin.users.active')}
							{:else if statusFilter === 'suspended'}
								{t('admin.users.suspended')}
							{:else}
								{statusFilter}
							{/if}
						</span>
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="all">{t('common.all')}</Select.Item>
						<Select.Item value="active">{t('admin.users.active')}</Select.Item>
						<Select.Item value="suspended">{t('admin.users.suspended')}</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>
		</Card>
	</AnimatedContainer>

	{#if loading && showSkeleton}
		<TableSkeleton rows={5} cols={7} />
	{:else if loading}
		<!-- Brief loading -->
	{:else if users.length === 0}
		<AnimatedContainer animation="fade-up" delay={100}>
			<Card variant="glass" class="mx-auto max-w-md">
				<Empty.Root>
					<Empty.Header>
						<Empty.Media variant="icon">
							<Users class="size-5" />
						</Empty.Media>
						<Empty.Title>{t('admin.users.noUsers')}</Empty.Title>
						<Empty.Description>{t('admin.users.noUsersDesc')}</Empty.Description>
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
										onclick={() => handleSort('name')}
										class="flex items-center gap-1 hover:text-foreground"
									>
										{t('admin.users.table.user')}
										<svelte:component this={getSortIcon('name')} class="h-4 w-4" />
									</button>
								</Table.Head>
								<Table.Head>
									<button
										onclick={() => handleSort('email')}
										class="flex items-center gap-1 hover:text-foreground"
									>
										{t('admin.users.table.email')}
										<svelte:component this={getSortIcon('email')} class="h-4 w-4" />
									</button>
								</Table.Head>
								<Table.Head>{t('admin.users.table.provider')}</Table.Head>
								<Table.Head>
									<button
										onclick={() => handleSort('created_at')}
										class="flex items-center gap-1 hover:text-foreground"
									>
										{t('admin.users.table.joined')}
										<svelte:component this={getSortIcon('created_at')} class="h-4 w-4" />
									</button>
								</Table.Head>
								<Table.Head>{t('admin.users.table.status')}</Table.Head>
								<Table.Head>
									<button
										onclick={() => handleSort('role')}
										class="flex items-center gap-1 hover:text-foreground"
									>
										{t('admin.users.table.role')}
										<svelte:component this={getSortIcon('role')} class="h-4 w-4" />
									</button>
								</Table.Head>
								<Table.Head class="w-10"></Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each users as user}
								<Table.Row>
									<Table.Cell>
										<div class="flex items-center gap-3">
											<Avatar class="h-8 w-8">
												{#if user.avatar_url}
													<AvatarImage src={user.avatar_url} alt={user.name || user.email} />
												{/if}
												<AvatarFallback>{getInitials(user.name, user.email)}</AvatarFallback>
											</Avatar>
											<span class="font-medium text-foreground">{user.name || '-'}</span>
										</div>
									</Table.Cell>
									<Table.Cell class="text-muted-foreground">{user.email}</Table.Cell>
									<Table.Cell>
										<div class="flex flex-wrap gap-1">
											{#each user.auth_provider.split(', ') as provider}
												<Badge variant="outline" class="capitalize">{provider}</Badge>
											{/each}
										</div>
									</Table.Cell>
									<Table.Cell class="text-muted-foreground">{formatDate(user.created_at)}</Table.Cell>
									<Table.Cell>
										{#if user.restriction.is_suspended}
											<div class="flex items-center gap-1">
												<Badge variant="destructive">{t('admin.users.suspended')}</Badge>
												<Tooltip.Root>
													<Tooltip.Trigger>
														<Info class="h-4 w-4 text-muted-foreground" />
													</Tooltip.Trigger>
													<Tooltip.Content class="max-w-xs">
														<p class="font-medium">{t('admin.users.suspendReason')}:</p>
														<p class="text-sm">{user.restriction.suspension_reason}</p>
														{#if user.restriction.suspended_until}
															<p class="mt-1 text-sm">
																<span class="font-medium">{t('admin.users.suspendUntil')}:</span>
																{formatDate(user.restriction.suspended_until)}
															</p>
														{:else}
															<p class="mt-1 text-sm text-muted-foreground">{t('admin.users.indefinite')}</p>
														{/if}
													</Tooltip.Content>
												</Tooltip.Root>
											</div>
										{:else}
											<Badge variant="outline">{t('admin.users.active')}</Badge>
										{/if}
									</Table.Cell>
									<Table.Cell>
										<Select.Root
											type="single"
											value={user.role}
											onValueChange={(v) => v && updateRole(user.id, v)}
										>
											<Select.Trigger
												class="w-32"
												disabled={updatingUserId === user.id}
											>
												{#if updatingUserId === user.id}
													<span class="text-muted-foreground">{t('common.loading')}</span>
												{:else}
													<Badge variant={getRoleBadgeVariant(user.role)} class="capitalize">
														{user.role}
													</Badge>
												{/if}
											</Select.Trigger>
											<Select.Content>
												{#each roles as role}
													<Select.Item value={role.name} class="capitalize">{role.name}</Select.Item>
												{/each}
											</Select.Content>
										</Select.Root>
									</Table.Cell>
									<Table.Cell>
										<DropdownMenu.Root>
											<DropdownMenu.Trigger>
												<Button variant="ghost" size="icon" class="h-8 w-8">
													<MoreHorizontal class="h-4 w-4" />
												</Button>
											</DropdownMenu.Trigger>
											<DropdownMenu.Content align="end">
												{#if user.role !== 'admin'}
													<DropdownMenu.Item onclick={() => openEditDialog(user)}>
														<Pencil class="mr-2 h-4 w-4" />
														{t('common.edit')}
													</DropdownMenu.Item>
													<DropdownMenu.Separator />
												{/if}
												<DropdownMenu.Item onclick={() => openTicketDialog(user, 'grant')}>
													<Ticket class="mr-2 h-4 w-4 text-green-500" />
													Grant Tickets
												</DropdownMenu.Item>
												<DropdownMenu.Item onclick={() => openTicketDialog(user, 'revoke')}>
													<Ticket class="mr-2 h-4 w-4 text-red-500" />
													Revoke Tickets
												</DropdownMenu.Item>
												{#if user.role !== 'admin'}
													<DropdownMenu.Separator />
													{#if user.restriction.is_suspended}
														<DropdownMenu.Item
															class="text-success-text focus:text-success-text"
															onclick={() => openUnsuspendDialog(user)}
														>
															<UserCheck class="mr-2 h-4 w-4 text-success-text" />
															{t('admin.users.unsuspend')}
														</DropdownMenu.Item>
													{:else}
														<DropdownMenu.Item
															class="text-destructive focus:text-destructive"
															onclick={() => openSuspendDialog(user)}
														>
															<Ban class="mr-2 h-4 w-4 text-destructive" />
															{t('admin.users.suspend')}
														</DropdownMenu.Item>
													{/if}
													<DropdownMenu.Separator />
													<DropdownMenu.Item
														class="text-destructive focus:text-destructive"
														onclick={() => openDeleteDialog(user)}
													>
														<Trash2 class="mr-2 h-4 w-4 text-destructive" />
														{t('common.delete')}
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
					{t('admin.users.totalCount', { count: pageInfo.total })}
				</p>
			</AnimatedContainer>
		{/if}
	{/if}

	<!-- Suspend Dialog -->
	<AlertDialog.Root bind:open={suspendDialogOpen}>
		<AlertDialog.Content interactOutsideBehavior="close">
			<AlertDialog.Header>
				<AlertDialog.Title>{t('admin.users.suspendDialogTitle')}</AlertDialog.Title>
				<AlertDialog.Description>
					{t('admin.users.suspendDialogDesc', { name: selectedUser?.name || selectedUser?.email || '' })}
				</AlertDialog.Description>
			</AlertDialog.Header>

			<div class="space-y-4 py-4">
				<div class="space-y-2">
					<Label for="reason">{t('admin.users.suspendReason')} *</Label>
					<Textarea
						id="reason"
						bind:value={suspendReason}
						placeholder={t('admin.users.suspendReasonPlaceholder')}
						rows={3}
					/>
				</div>

				<div class="space-y-2">
					<Label for="until">{t('admin.users.suspendUntil')}</Label>
					<DateTimePicker
						id="until"
						bind:value={suspendUntil}
						placeholder={t('admin.users.suspendUntilPlaceholder')}
					/>
					<p class="text-sm text-muted-foreground">
						{t('admin.users.suspendUntilNote')}
					</p>
				</div>
			</div>

			<AlertDialog.Footer>
				<AlertDialog.Cancel>{t('common.cancel')}</AlertDialog.Cancel>
				<Button
					variant="destructive"
					onclick={handleSuspend}
					disabled={!suspendReason.trim() || suspending}
				>
					{suspending ? t('common.loading') : t('admin.users.confirmSuspend')}
				</Button>
			</AlertDialog.Footer>
		</AlertDialog.Content>
	</AlertDialog.Root>

	<!-- Unsuspend Dialog -->
	<AlertDialog.Root bind:open={unsuspendDialogOpen}>
		<AlertDialog.Content interactOutsideBehavior="close">
			<AlertDialog.Header>
				<AlertDialog.Title>{t('admin.users.unsuspendDialogTitle')}</AlertDialog.Title>
				<AlertDialog.Description>
					{t('admin.users.unsuspendDialogDesc', { name: selectedUser?.name || selectedUser?.email || '' })}
				</AlertDialog.Description>
			</AlertDialog.Header>

			<AlertDialog.Footer>
				<AlertDialog.Cancel>{t('common.cancel')}</AlertDialog.Cancel>
				<Button onclick={handleUnsuspend} disabled={suspending}>
					{suspending ? t('common.loading') : t('admin.users.confirmUnsuspend')}
				</Button>
			</AlertDialog.Footer>
		</AlertDialog.Content>
	</AlertDialog.Root>

	<!-- Edit User Dialog -->
	<AlertDialog.Root bind:open={editDialogOpen}>
		<AlertDialog.Content class="max-w-lg" interactOutsideBehavior="close">
			<AlertDialog.Header>
				<AlertDialog.Title>{t('admin.users.editDialogTitle')}</AlertDialog.Title>
				<AlertDialog.Description>
					{#if editingUser}
						{t('admin.users.editDialogDesc', { email: editingUser.email })}
					{/if}
				</AlertDialog.Description>
			</AlertDialog.Header>

			{#if editError}
				<Alert variant="destructive" class="mb-4">
					<AlertDescription>{editError}</AlertDescription>
				</Alert>
			{/if}

			{#if editingUser}
				<div class="grid gap-4 py-4">
					<div class="grid grid-cols-4 items-center gap-4">
						<Label for="edit_name" class="text-right">
							{t('admin.users.editName')}
						</Label>
						<Input
							id="edit_name"
							bind:value={editName}
							placeholder={t('admin.users.editNamePlaceholder')}
							class="col-span-3"
						/>
					</div>

					<div class="grid grid-cols-4 items-center gap-4">
						<Label for="edit_phone" class="text-right">
							{t('admin.users.editPhone')}
						</Label>
						<Input
							id="edit_phone"
							bind:value={editPhone}
							placeholder={t('admin.users.editPhonePlaceholder')}
							class="col-span-3"
						/>
					</div>
				</div>
			{/if}

			<AlertDialog.Footer>
				<AlertDialog.Cancel onclick={closeEditDialog}>
					{t('common.cancel')}
				</AlertDialog.Cancel>
				<AlertDialog.Action onclick={handleEditSave} disabled={editSaving}>
					{#if editSaving}
						{t('common.saving')}
					{:else}
						{t('common.save')}
					{/if}
				</AlertDialog.Action>
			</AlertDialog.Footer>
		</AlertDialog.Content>
	</AlertDialog.Root>

	<!-- Delete User Dialog -->
	<AlertDialog.Root bind:open={deleteDialogOpen}>
		<AlertDialog.Content interactOutsideBehavior="close">
			<AlertDialog.Header>
				<AlertDialog.Title>{t('admin.users.deleteDialogTitle')}</AlertDialog.Title>
				<AlertDialog.Description>
					{t('admin.users.deleteDialogDesc', { name: userToDelete?.name || userToDelete?.email || '' })}
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
					onclick={handleDelete}
					disabled={deleting}
				>
					{deleting ? t('admin.users.deleting') : t('common.delete')}
				</Button>
			</AlertDialog.Footer>
		</AlertDialog.Content>
	</AlertDialog.Root>

	<!-- Ticket Grant/Revoke Dialog -->
	<AlertDialog.Root bind:open={ticketDialogOpen}>
		<AlertDialog.Content class="max-w-md" interactOutsideBehavior="close">
			<AlertDialog.Header>
				<AlertDialog.Title>
					{ticketAction === 'grant' ? 'Grant Tickets' : 'Revoke Tickets'}
				</AlertDialog.Title>
				<AlertDialog.Description>
					{#if ticketUser}
						{ticketAction === 'grant' ? 'Add' : 'Remove'} tickets for {ticketUser.name || ticketUser.email}
						{#if userTicketBalance !== null}
							<br />
							<span class="font-medium">Current balance: {userTicketBalance} tickets</span>
						{/if}
					{/if}
				</AlertDialog.Description>
			</AlertDialog.Header>

			{#if ticketError}
				<Alert variant="destructive" class="mb-4">
					<AlertDescription>{ticketError}</AlertDescription>
				</Alert>
			{/if}

			<div class="grid gap-4 py-4">
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="ticket_amount" class="text-right">Amount</Label>
					<Input
						id="ticket_amount"
						type="number"
						min="1"
						max={ticketAction === 'revoke' && userTicketBalance !== null ? userTicketBalance : 100}
						bind:value={ticketAmount}
						class="col-span-3"
					/>
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="ticket_reason" class="text-right">Reason</Label>
					<Input
						id="ticket_reason"
						bind:value={ticketReason}
						placeholder="Optional reason"
						class="col-span-3"
					/>
				</div>
			</div>

			<AlertDialog.Footer>
				<AlertDialog.Cancel disabled={ticketProcessing}>
					{t('common.cancel')}
				</AlertDialog.Cancel>
				<Button
					variant={ticketAction === 'grant' ? 'default' : 'destructive'}
					onclick={handleTicketAction}
					disabled={ticketProcessing || ticketAmount < 1}
				>
					{ticketProcessing ? 'Processing...' : ticketAction === 'grant' ? 'Grant' : 'Revoke'}
				</Button>
			</AlertDialog.Footer>
		</AlertDialog.Content>
	</AlertDialog.Root>
</AdminLayout>

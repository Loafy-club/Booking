<script lang="ts">
	import type { Snippet } from 'svelte';
	import { page } from '$app/stores';
	import { useTranslation } from '$lib/i18n/index.svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import { PageBackground } from '$lib/components/ui/page-background';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import {
		LayoutDashboard,
		Users,
		Calendar,
		CreditCard,
		ChevronLeft,
		ChevronRight
	} from 'lucide-svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
	const t = useTranslation();

	let collapsed = $state(false);

	interface NavItem {
		href: string;
		icon: typeof LayoutDashboard;
		labelKey: string;
		adminOnly?: boolean;
	}

	const allNavItems: NavItem[] = [
		{ href: '/admin', icon: LayoutDashboard, labelKey: 'admin.nav.dashboard' },
		{ href: '/admin/users', icon: Users, labelKey: 'admin.nav.users', adminOnly: true },
		{ href: '/admin/sessions', icon: Calendar, labelKey: 'admin.nav.sessions' },
		{ href: '/admin/bookings', icon: CreditCard, labelKey: 'admin.nav.bookings', adminOnly: true }
	];

	// Filter nav items based on user role
	let navItems = $derived(
		authStore.isAdmin ? allNavItems : allNavItems.filter(item => !item.adminOnly)
	);

	function isActive(href: string): boolean {
		if (href === '/admin') {
			return $page.url.pathname === '/admin';
		}
		return $page.url.pathname.startsWith(href);
	}
</script>

<PageBackground variant="subtle">
	<Navigation />

	<div class="flex min-h-[calc(100vh-4rem)]">
		<!-- Sidebar -->
		<aside
			class="border-r border-border/50 bg-card/30 backdrop-blur-sm transition-all duration-300 {collapsed
				? 'w-16'
				: 'w-64'}"
		>
			<div class="sticky top-16 flex h-[calc(100vh-4rem)] flex-col">
				<!-- Sidebar Header -->
				<div class="flex items-center justify-between p-4">
					{#if !collapsed}
						<h2 class="text-lg font-semibold text-foreground">{t('admin.title')}</h2>
					{/if}
					<Button
						variant="ghost"
						size="icon"
						class="h-8 w-8"
						onclick={() => (collapsed = !collapsed)}
					>
						{#if collapsed}
							<ChevronRight class="h-4 w-4" />
						{:else}
							<ChevronLeft class="h-4 w-4" />
						{/if}
					</Button>
				</div>

				<Separator />

				<!-- Navigation -->
				<nav class="flex-1 space-y-1 p-2">
					{#each navItems as item}
						{@const active = isActive(item.href)}
						<a
							href={item.href}
							class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors
                {active
								? 'bg-primary/10 text-primary'
								: 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
						>
							<item.icon class="h-5 w-5 shrink-0" />
							{#if !collapsed}
								<span>{t(item.labelKey)}</span>
							{/if}
						</a>
					{/each}
				</nav>

				</div>
		</aside>

		<!-- Main Content -->
		<main class="flex-1 overflow-auto">
			<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
				{@render children()}
			</div>
		</main>
	</div>
</PageBackground>

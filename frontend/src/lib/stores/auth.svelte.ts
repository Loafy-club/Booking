import { supabase } from '$lib/auth/supabase';
import { api } from '$lib/api/client';
import type { User as SupabaseUser, Session, Subscription } from '@supabase/supabase-js';
import type { TicketBalanceResponse } from '$lib/types';

export interface User {
	id: string;
	email: string;
	name?: string;
	phone?: string;
	avatar_url?: string;
	role: 'user' | 'organizer' | 'admin';
	birthday?: string;
}

export interface TicketBalance {
	ticketsRemaining: number;
	hasActiveSubscription: boolean;
	currentPeriodEnd: string | null;
}

// Module-level tracking to survive HMR
let authSubscription: Subscription | null = null;
let cachedSupabaseUser: SupabaseUser | null = null;

class AuthStore {
	user = $state<User | null>(null);
	supabaseUser = $state<SupabaseUser | null>(cachedSupabaseUser);
	ticketBalance = $state<TicketBalance | null>(null);
	loading = $state(true);
	initialized = $state(false);

	async initialize(session?: Session | null) {
		// If we have cached user from previous HMR cycle, restore it
		if (cachedSupabaseUser && !this.supabaseUser) {
			this.supabaseUser = cachedSupabaseUser;
		}

		// Skip if already initialized this instance
		if (this.initialized) {
			this.loading = false;
			return;
		}

		try {
			// Use provided session or get from Supabase
			let currentSession = session;
			if (!currentSession) {
				const { data } = await supabase.auth.getSession();
				currentSession = data.session;
			}

			if (currentSession?.user) {
				this.supabaseUser = currentSession.user as SupabaseUser;
				cachedSupabaseUser = currentSession.user as SupabaseUser;
				// Don't await fetchUser - let it complete in background
				this.fetchUser();
			}

			// Clean up any existing subscription before creating a new one
			if (authSubscription) {
				authSubscription.unsubscribe();
				authSubscription = null;
			}

			// Set up the auth state change listener
			const { data: { subscription } } = supabase.auth.onAuthStateChange(async (event, session) => {
				if (event === 'SIGNED_IN' && session?.user) {
					this.supabaseUser = session.user;
					cachedSupabaseUser = session.user;
					this.fetchUser();
				} else if (event === 'SIGNED_OUT') {
					this.user = null;
					this.supabaseUser = null;
					cachedSupabaseUser = null;
				} else if (event === 'TOKEN_REFRESHED' && session?.user) {
					this.supabaseUser = session.user;
					cachedSupabaseUser = session.user;
				}
			});

			authSubscription = subscription;
			this.initialized = true;
		} catch (error) {
			console.error('Failed to initialize auth:', error);
		} finally {
			this.loading = false;
		}
	}

	async fetchUser() {
		try {
			const response = await api.auth.me();
			this.user = response.data;
			// Fetch ticket balance in background
			this.fetchTicketBalance();
		} catch (error) {
			console.error('Failed to fetch user:', error);
			this.user = null;
		}
	}

	async fetchTicketBalance() {
		try {
			const response = await api.subscriptions.getTicketBalance();
			const data: TicketBalanceResponse = response.data;
			this.ticketBalance = {
				ticketsRemaining: data.tickets_remaining,
				hasActiveSubscription: data.has_active_subscription,
				currentPeriodEnd: data.current_period_end
			};
		} catch (error) {
			console.error('Failed to fetch ticket balance:', error);
			this.ticketBalance = null;
		}
	}

	async refreshTicketBalance() {
		return this.fetchTicketBalance();
	}

	/**
	 * Generic OAuth sign-in method that works with any provider.
	 * Redirects to the provider's OAuth flow.
	 */
	private async signInWithOAuth(provider: 'google' | 'facebook' | 'apple') {
		const { data, error } = await supabase.auth.signInWithOAuth({
			provider,
			options: {
				redirectTo: `${window.location.origin}/auth/callback`
			}
		});

		if (error) {
			console.error(`${provider} sign in error:`, error);
			throw error;
		}

		// Redirect to the OAuth URL
		if (data?.url) {
			window.location.href = data.url;
		}
	}

	async signInWithGoogle() {
		return this.signInWithOAuth('google');
	}

	async signInWithFacebook() {
		return this.signInWithOAuth('facebook');
	}

	async signInWithApple() {
		return this.signInWithOAuth('apple');
	}

	async signOut() {
		try {
			// Call backend logout endpoint
			await api.auth.logout();
		} catch (error) {
			console.error('Backend logout error:', error);
		} finally {
			// Always sign out from Supabase
			await supabase.auth.signOut();
			this.user = null;
			this.supabaseUser = null;
			this.ticketBalance = null;
		}
	}

	/**
	 * Permanently delete the user's account and all associated data.
	 * This action cannot be undone.
	 */
	async deleteAccount() {
		try {
			// Call backend to delete user data
			await api.users.deleteAccount();
		} catch (error) {
			console.error('Failed to delete account from backend:', error);
			throw error;
		}

		// Sign out from Supabase (this will also invalidate the session)
		await supabase.auth.signOut();
		this.user = null;
		this.supabaseUser = null;
		this.ticketBalance = null;
		cachedSupabaseUser = null;
	}

	/**
	 * Link a new OAuth provider to the current account.
	 * This allows users to sign in with multiple providers.
	 */
	async linkIdentity(provider: 'google' | 'facebook' | 'apple') {
		const { data, error } = await supabase.auth.linkIdentity({
			provider,
			options: {
				redirectTo: `${window.location.origin}/account?tab=account&linked=${provider}`
			}
		});

		if (error) {
			console.error(`Failed to link ${provider}:`, error);
			throw error;
		}

		// Redirect to the OAuth URL
		if (data?.url) {
			window.location.href = data.url;
		}
	}

	/**
	 * Unlink an OAuth provider from the current account.
	 * User must have at least one identity remaining.
	 */
	async unlinkIdentity(provider: string) {
		const identity = this.identities.find(id => id.provider === provider);
		if (!identity) {
			throw new Error(`No identity found for provider: ${provider}`);
		}

		const { error } = await supabase.auth.unlinkIdentity(identity);

		if (error) {
			console.error('Failed to unlink identity:', error);
			throw error;
		}

		// Refresh user data
		const { data } = await supabase.auth.getUser();
		if (data?.user) {
			this.supabaseUser = data.user;
			cachedSupabaseUser = data.user;
		}
	}

	/**
	 * Get the list of connected identities for the current user.
	 */
	get identities() {
		return this.supabaseUser?.identities || [];
	}

	/**
	 * Check if a specific provider is connected.
	 */
	isProviderConnected(provider: string): boolean {
		return this.identities.some(id => id.provider === provider);
	}

	get isAuthenticated(): boolean {
		// User is authenticated if we have a Supabase session
		// this.user is optional (backend profile, may fail if backend is down)
		return this.supabaseUser !== null;
	}

	get isAdmin(): boolean {
		return this.user?.role === 'admin';
	}

	get isOrganizer(): boolean {
		return this.user?.role === 'organizer' || this.user?.role === 'admin';
	}

	get ticketsRemaining(): number {
		return this.ticketBalance?.ticketsRemaining ?? 0;
	}

	get hasActiveSubscription(): boolean {
		return this.ticketBalance?.hasActiveSubscription ?? false;
	}

	get hasTickets(): boolean {
		return this.ticketsRemaining > 0;
	}
}

export const authStore = new AuthStore();

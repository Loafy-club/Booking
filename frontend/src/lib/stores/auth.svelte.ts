import { supabase } from '$lib/auth/supabase';
import { api } from '$lib/api/client';
import type { User as SupabaseUser } from '@supabase/supabase-js';

export interface User {
	id: string;
	email: string;
	full_name?: string;
	phone_number?: string;
	role: 'user' | 'organizer' | 'admin';
	created_at: string;
}

class AuthStore {
	user = $state<User | null>(null);
	supabaseUser = $state<SupabaseUser | null>(null);
	loading = $state(true);
	initialized = $state(false);

	async initialize() {
		if (this.initialized) return;

		try {
			// Get current Supabase session
			const {
				data: { session }
			} = await supabase.auth.getSession();

			if (session?.user) {
				this.supabaseUser = session.user;
				await this.fetchUser();
			}

			// Listen to auth changes
			supabase.auth.onAuthStateChange(async (event, session) => {
				console.log('Auth state changed:', event);

				if (event === 'SIGNED_IN' && session?.user) {
					this.supabaseUser = session.user;
					await this.fetchUser();
				} else if (event === 'SIGNED_OUT') {
					this.user = null;
					this.supabaseUser = null;
				} else if (event === 'TOKEN_REFRESHED' && session?.user) {
					this.supabaseUser = session.user;
				}
			});

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
		} catch (error) {
			console.error('Failed to fetch user:', error);
			this.user = null;
		}
	}

	async signInWithGoogle() {
		const { error } = await supabase.auth.signInWithOAuth({
			provider: 'google',
			options: {
				redirectTo: `${window.location.origin}/auth/callback`
			}
		});

		if (error) {
			console.error('Google sign in error:', error);
			throw error;
		}
	}

	async signInWithFacebook() {
		const { error } = await supabase.auth.signInWithOAuth({
			provider: 'facebook',
			options: {
				redirectTo: `${window.location.origin}/auth/callback`
			}
		});

		if (error) {
			console.error('Facebook sign in error:', error);
			throw error;
		}
	}

	async signInWithApple() {
		const { error } = await supabase.auth.signInWithOAuth({
			provider: 'apple',
			options: {
				redirectTo: `${window.location.origin}/auth/callback`
			}
		});

		if (error) {
			console.error('Apple sign in error:', error);
			throw error;
		}
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
		}
	}

	get isAuthenticated(): boolean {
		return this.user !== null && this.supabaseUser !== null;
	}

	get isAdmin(): boolean {
		return this.user?.role === 'admin';
	}

	get isOrganizer(): boolean {
		return this.user?.role === 'organizer' || this.user?.role === 'admin';
	}
}

export const authStore = new AuthStore();

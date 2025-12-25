import axios, { type AxiosInstance, type InternalAxiosRequestConfig } from 'axios';
import { supabase } from '$lib/auth/supabase';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000';

class ApiClient {
	private client: AxiosInstance;

	constructor() {
		this.client = axios.create({
			baseURL: API_BASE_URL,
			headers: {
				'Content-Type': 'application/json'
			}
		});

		// Request interceptor to add auth token
		this.client.interceptors.request.use(
			async (config: InternalAxiosRequestConfig) => {
				try {
					const {
						data: { session }
					} = await supabase.auth.getSession();

					if (session?.access_token) {
						config.headers.Authorization = `Bearer ${session.access_token}`;
					}
				} catch (error) {
					// Supabase not configured or session retrieval failed
					// Continue without auth header for public endpoints
					console.warn('Auth session not available:', error);
				}

				return config;
			},
			(error) => {
				return Promise.reject(error);
			}
		);

		// Response interceptor for error handling
		this.client.interceptors.response.use(
			(response) => response,
			async (error) => {
				// Don't auto-signout on 401 - let components handle auth state
				// This prevents logout loops during OAuth callback or when backend is unavailable
				// The auth store will naturally show unauthenticated state if session is invalid
				if (error.response?.status === 401) {
					console.warn('API returned 401 - authentication may be required');
				}

				// Handle suspended user error
				if (
					error.response?.status === 403 &&
					error.response?.data?.error === 'account_suspended'
				) {
					const { reason, until } = error.response.data;
					const params = new URLSearchParams();
					if (reason) params.set('reason', reason);
					if (until) params.set('until', until);
					window.location.href = `/suspended?${params.toString()}`;
				}

				return Promise.reject(error);
			}
		);
	}

	get axios(): AxiosInstance {
		return this.client;
	}
}

export const apiClient = new ApiClient().axios;

// API helper functions
export const api = {
	// Auth
	auth: {
		me: () => apiClient.get('/api/auth/me'),
		logout: () => apiClient.post('/api/auth/logout'),
		callback: (token: string) => apiClient.post('/api/auth/callback', { token })
	},

	// Users
	users: {
		updateProfile: (data: { name?: string; phone?: string; avatar_url?: string }) =>
			apiClient.put('/api/users/me', data),
		deleteAccount: () => apiClient.delete('/api/users/me')
	},

	// Sessions
	sessions: {
		list: (params?: { from_date?: string; organizer_id?: string; available_only?: boolean }) =>
			apiClient.get('/api/sessions', { params }),
		get: (id: string) => apiClient.get(`/api/sessions/${id}`),
		getParticipants: (id: string) => apiClient.get(`/api/sessions/${id}/participants`),
		create: (data: {
			title: string;
			description?: string;
			location: string;
			start_time: string;
			end_time: string;
			max_slots: number;
			price_vnd: number;
			early_access_ends_at?: string;
			expenses?: Array<{
				category: 'court_rental' | 'equipment' | 'instructor' | 'custom';
				description?: string;
				cost_type: 'per_court' | 'total';
				amount_vnd: number;
			}>;
		}) => apiClient.post('/api/sessions', data),
		update: (id: string, data: Partial<{
			title: string;
			description?: string;
			location: string;
			start_time: string;
			end_time: string;
			max_slots: number;
			price_vnd: number;
			early_access_ends_at?: string;
			status: string;
		}>) => apiClient.put(`/api/sessions/${id}`, data),
		delete: (id: string) => apiClient.delete(`/api/sessions/${id}`)
	},

	// Bookings
	bookings: {
		list: (params?: { page?: number; per_page?: number }) =>
			apiClient.get('/api/bookings', { params }),
		get: (id: string) => apiClient.get(`/api/bookings/${id}`),
		create: (data: {
			session_id: string;
			guest_count: number;
			payment_method: 'stripe' | 'qr';
		}) => apiClient.post('/api/bookings', data),
		cancel: (id: string) => apiClient.delete(`/api/bookings/${id}`)
	},

	// Payments
	payments: {
		createIntent: (booking_id: string) =>
			apiClient.post('/api/payments/stripe/intent', { booking_id })
	},

	// Admin
	admin: {
		getStats: (period: string = '30d') => apiClient.get(`/api/admin/stats?period=${period}`),
		listUsers: (params?: {
			page?: number;
			per_page?: number;
			search?: string;
			role?: string;
			status?: string;
			sort_by?: string;
			sort_order?: string;
		}) => apiClient.get('/api/admin/users', { params }),
		updateUserRole: (userId: string, role: string) =>
			apiClient.put(`/api/admin/users/${userId}/role`, { role }),
		suspendUser: (userId: string, data: { reason: string; until?: string }) =>
			apiClient.post(`/api/admin/users/${userId}/suspend`, data),
		unsuspendUser: (userId: string) => apiClient.post(`/api/admin/users/${userId}/unsuspend`),
		updateUser: (userId: string, data: { name?: string; phone?: string; role?: string }) =>
			apiClient.put(`/api/admin/users/${userId}`, data),
		deleteUser: (userId: string) => apiClient.delete(`/api/admin/users/${userId}`),
		listBookings: (params?: {
			page?: number;
			per_page?: number;
			search?: string;
			payment_status?: string;
			session_id?: string;
			sort_by?: string;
			sort_order?: string;
		}) => apiClient.get('/api/admin/bookings', { params }),
		getBooking: (id: string) => apiClient.get(`/api/admin/bookings/${id}`),
		updateBooking: (id: string, data: {
			guest_count?: number | null;
			price_paid_vnd?: number | null;
			guest_price_paid_vnd?: number | null;
			payment_method?: string | null;
			payment_status?: string | null;
			admin_notes?: string | null;
		}) => apiClient.put(`/api/admin/bookings/${id}`, data),
		listSessions: (params?: {
			page?: number;
			per_page?: number;
			search?: string;
			status?: string;
			organizer_id?: string;
			sort_by?: string;
			sort_order?: string;
		}) => apiClient.get('/api/admin/sessions', { params }),
		listRoles: () => apiClient.get('/api/admin/roles'),
		// Profit endpoints
		getProfitStats: (period: string = '30d') =>
			apiClient.get(`/api/admin/stats/profit?period=${period}`),
		getSessionsProfit: (period: string = '30d', limit: number = 20) =>
			apiClient.get(`/api/admin/sessions/profit?period=${period}&limit=${limit}`),
		getExpensesByCategory: (period: string = '30d') =>
			apiClient.get(`/api/admin/expenses/by-category?period=${period}`),
		getDailyProfitData: (period: string = '30d') =>
			apiClient.get(`/api/admin/profit/daily?period=${period}`)
	}
};

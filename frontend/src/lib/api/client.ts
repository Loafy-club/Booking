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
				if (error.response?.status === 401) {
					// Token expired or invalid, sign out
					await supabase.auth.signOut();
					window.location.href = '/auth/login';
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
		callback: (code: string) => apiClient.post('/api/auth/callback', { code })
	},

	// Sessions
	sessions: {
		list: (params?: { from_date?: string; organizer_id?: string; available_only?: boolean }) =>
			apiClient.get('/api/sessions', { params }),
		get: (id: string) => apiClient.get(`/api/sessions/${id}`),
		create: (data: {
			title: string;
			description?: string;
			location: string;
			start_time: string;
			end_time: string;
			max_slots: number;
			price_vnd: number;
			early_access_ends_at?: string;
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
		list: () => apiClient.get('/api/bookings'),
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
	}
};

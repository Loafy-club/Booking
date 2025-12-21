import { createClient } from '@supabase/supabase-js';

const supabaseUrl = import.meta.env.VITE_SUPABASE_URL || '';
const supabaseAnonKey = import.meta.env.VITE_SUPABASE_ANON_KEY || '';

// Check if we have valid Supabase credentials (not placeholders)
const hasValidCredentials =
	supabaseUrl &&
	supabaseAnonKey &&
	!supabaseUrl.includes('your-project') &&
	!supabaseAnonKey.includes('your-');

// Create standard Supabase client with localStorage for PKCE storage
// This works better for client-side OAuth than cookie-based SSR storage
export const supabase = hasValidCredentials
	? createClient(supabaseUrl, supabaseAnonKey, {
			auth: {
				flowType: 'implicit',
				persistSession: true,
				autoRefreshToken: true,
				detectSessionInUrl: true
			}
		})
	: createClient('https://placeholder.supabase.co', 'placeholder-key');

// Log warning if using placeholder credentials
if (!hasValidCredentials) {
	console.warn('Supabase credentials not configured. Auth features will be disabled.');
}

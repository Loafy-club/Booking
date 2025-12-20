import { createClient, type SupabaseClient } from '@supabase/supabase-js';

const supabaseUrl = import.meta.env.VITE_SUPABASE_URL || '';
const supabaseAnonKey = import.meta.env.VITE_SUPABASE_ANON_KEY || '';

// Check if we have valid Supabase credentials (not placeholders)
const hasValidCredentials =
	supabaseUrl &&
	supabaseAnonKey &&
	!supabaseUrl.includes('your-project') &&
	!supabaseAnonKey.includes('your-');

let supabase: SupabaseClient;

if (hasValidCredentials) {
	supabase = createClient(supabaseUrl, supabaseAnonKey, {
		auth: {
			autoRefreshToken: true,
			persistSession: true,
			detectSessionInUrl: true
		}
	});
} else {
	// Create a mock client for development without Supabase
	console.warn('Supabase credentials not configured. Auth features will be disabled.');
	supabase = createClient('https://placeholder.supabase.co', 'placeholder-key', {
		auth: {
			autoRefreshToken: false,
			persistSession: false,
			detectSessionInUrl: false
		}
	});
}

export { supabase };

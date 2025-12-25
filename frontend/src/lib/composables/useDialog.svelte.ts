/**
 * Composable for managing dialog state in admin pages.
 *
 * Provides a consistent pattern for dialogs that operate on an item,
 * with loading/error state management and async action execution.
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { useDialog } from '$lib/composables/useDialog.svelte';
 *
 *   interface User {
 *     id: string;
 *     name: string;
 *   }
 *
 *   const deleteDialog = useDialog<User>();
 *
 *   async function handleDelete() {
 *     await deleteDialog.execute(async (user) => {
 *       await api.admin.deleteUser(user.id);
 *       // Reload users list
 *       await loadUsers();
 *     }, 'Failed to delete user');
 *   }
 * </script>
 *
 * <button onclick={() => deleteDialog.openWith(user)}>Delete</button>
 *
 * <AlertDialog.Root bind:open={deleteDialog.open}>
 *   {#if deleteDialog.error}
 *     <Alert variant="destructive">{deleteDialog.error}</Alert>
 *   {/if}
 *   <p>Are you sure you want to delete {deleteDialog.item?.name}?</p>
 *   <button onclick={handleDelete} disabled={deleteDialog.loading}>
 *     {deleteDialog.loading ? 'Deleting...' : 'Delete'}
 *   </button>
 * </AlertDialog.Root>
 * ```
 */

import { extractErrorMessage } from '$lib/utils';

export interface DialogState<T> {
	/** Whether the dialog is open */
	open: boolean;
	/** The item the dialog is operating on */
	item: T | null;
	/** Error message if the action failed */
	error: string | null;
	/** Whether an async action is in progress */
	loading: boolean;
	/** Open the dialog with an item */
	openWith: (item: T) => void;
	/** Close the dialog and reset state */
	close: () => void;
	/** Clear the error message */
	clearError: () => void;
	/**
	 * Execute an async action with loading/error handling.
	 * Closes the dialog on success.
	 *
	 * @param action - The async action to execute
	 * @param errorMessage - Default error message if extraction fails
	 * @returns The result of the action, or undefined if it failed
	 */
	execute: <R>(action: (item: T) => Promise<R>, errorMessage?: string) => Promise<R | undefined>;
}

/**
 * Creates a dialog state manager with loading/error handling.
 *
 * @returns Dialog state object with reactive properties and methods
 */
export function useDialog<T>(): DialogState<T> {
	let open = $state(false);
	let item = $state<T | null>(null);
	let error = $state<string | null>(null);
	let loading = $state(false);

	function openWith(newItem: T) {
		item = newItem;
		error = null;
		loading = false;
		open = true;
	}

	function close() {
		open = false;
		item = null;
		error = null;
		loading = false;
	}

	function clearError() {
		error = null;
	}

	async function execute<R>(
		action: (currentItem: T) => Promise<R>,
		errorMessage: string = 'Operation failed'
	): Promise<R | undefined> {
		if (!item) return undefined;

		loading = true;
		error = null;

		try {
			const result = await action(item);
			close();
			return result;
		} catch (err) {
			error = extractErrorMessage(err, errorMessage);
			return undefined;
		} finally {
			loading = false;
		}
	}

	return {
		get open() {
			return open;
		},
		set open(value: boolean) {
			open = value;
			if (!value) {
				// Reset state when dialog is closed externally
				item = null;
				error = null;
				loading = false;
			}
		},
		get item() {
			return item;
		},
		get error() {
			return error;
		},
		get loading() {
			return loading;
		},
		openWith,
		close,
		clearError,
		execute
	};
}

/**
 * Creates a confirmation dialog state for simple yes/no confirmations.
 * Unlike useDialog, this doesn't operate on an item - just tracks open/loading/error state.
 *
 * @example
 * ```svelte
 * const logoutConfirm = useConfirmDialog();
 *
 * async function handleLogout() {
 *   await logoutConfirm.execute(async () => {
 *     await auth.logout();
 *   }, 'Logout failed');
 * }
 * ```
 */
export function useConfirmDialog() {
	let open = $state(false);
	let error = $state<string | null>(null);
	let loading = $state(false);

	function openDialog() {
		error = null;
		loading = false;
		open = true;
	}

	function close() {
		open = false;
		error = null;
		loading = false;
	}

	async function execute<R>(
		action: () => Promise<R>,
		errorMessage: string = 'Operation failed'
	): Promise<R | undefined> {
		loading = true;
		error = null;

		try {
			const result = await action();
			close();
			return result;
		} catch (err) {
			error = extractErrorMessage(err, errorMessage);
			return undefined;
		} finally {
			loading = false;
		}
	}

	return {
		get open() {
			return open;
		},
		set open(value: boolean) {
			open = value;
			if (!value) {
				error = null;
				loading = false;
			}
		},
		get error() {
			return error;
		},
		get loading() {
			return loading;
		},
		openDialog,
		close,
		execute
	};
}

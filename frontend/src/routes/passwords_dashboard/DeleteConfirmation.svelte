<script lang="ts">
	import { createEventDispatcher } from 'svelte';
    import type { PasswordEntryClean } from './+page.server';
    import { fade, scale } from 'svelte/transition';
    import { enhance } from '$app/forms';
	const dispatch = createEventDispatcher();

    let { showModal, entry} : { showModal : boolean, entry : PasswordEntryClean} = $props()

</script>

{#if showModal}
  <form method="POST" action="?/delete" use:enhance>
    <input type="hidden" name="id" value={entry.id} />
    <div
      transition:fade={{ duration: 200 }}
      class="fixed inset-0 z-999 grid h-screen w-screen place-items-center bg-white/30 backdrop-blur-sm"
    >
      <div
        transition:scale={{
          duration: 300,
          start: 0.95,
        }}
        class="relative m-4 p-6 w-full max-w-125 rounded-2xl bg-white shadow-2xl border border-slate-100"
      >
        <h3 class="text-xl font-bold text-slate-800 mb-4">
                Passwords Entry Deletion
        </h3>

        <div class="space-y-4">
            <div>
                <!-- svelte-ignore a11y_label_has_associated_control -->
                <label class="text-lg font-semibold text-slate-800">
                    Are you sure you want to permanently delete this password ? This action is irreversible.
                </label>
            </div>
        </div>

        <div class="flex justify-end gap-3 mt-8">
          <button
            class="px-4 py-2 text-slate-600 hover:text-slate-800 font-medium"
            type="reset"
            onclick={() => dispatch("cancel")}
          >
            Cancel
          </button>
          <button
            class="px-6 py-2 bg-slate-900 text-white rounded-lg font-medium hover:bg-slate-800 shadow-lg transition-all active:scale-95"
            type="submit"
            onclick={() => dispatch("confirm")}
          >
            Confirm
          </button>
        </div>
      </div>
    </div>
  </form>
{/if}

<script lang="ts">
  import { enhance } from "$app/forms";
  import { fade, scale } from "svelte/transition";

  let { isAddingEntry }: { isAddingEntry: boolean } = $props();
</script>

<button
  class="flex min-w-fit gap-5 items-center rounded-md bg-slate-800 py-2 px-4 border border-transparent text-center text-lg text-white transition-all shadow-md hover:shadow-lg focus:bg-slate-700 focus:shadow-none active:bg-slate-700 hover:bg-slate-700 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none ml-2"
  type="button"
  onclick={() => (isAddingEntry = !isAddingEntry)}
>
  Add Password
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class="lucide lucide-circle-plus-icon lucide-circle-plus"
    ><circle cx="12" cy="12" r="10" /><path d="M8 12h8" /><path
      d="M12 8v8"
    /></svg
  >
</button>
{#if isAddingEntry}
  <form method="POST" action="?/create_entry" use:enhance>
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
          Add a New Password
        </h3>

        <div class="space-y-4">
          <div>
            <!-- svelte-ignore a11y_label_has_associated_control -->
            <label class="text-xs font-semibold text-slate-500 uppercase">
              Service
            </label>
            <input
              name="origin"
              placeholder="e.g. Netflix, Amazon, Youtube..."
              class="w-full mt-1 border border-slate-200 rounded-lg p-2 focus:ring-2 focus:ring-slate-200 outline-none transition-all"
            />
          </div>
          <div>
            <!-- svelte-ignore a11y_label_has_associated_control -->
            <label class="text-xs font-semibold text-slate-500 uppercase"
              >Username</label
            >
            <input
              name="username"
              placeholder="email@example.com"
              class="w-full mt-1 border border-slate-200 rounded-lg p-2 focus:ring-2 focus:ring-slate-200 outline-none transition-all"
            />
          </div>
          <div>
            <!-- svelte-ignore a11y_label_has_associated_control -->
            <label class="text-xs font-semibold text-slate-500 uppercase"
              >Password</label
            >
            <input
              placeholder="mySecretPassword"
              name="password"
              class="w-full mt-1 border border-slate-200 rounded-lg p-2 focus:ring-2 focus:ring-slate-200 outline-none transition-all"
            />
          </div>
        </div>

        <div class="flex justify-end gap-3 mt-8">
          <button
            onclick={() => (isAddingEntry = false)}
            class="px-4 py-2 text-slate-600 hover:text-slate-800 font-medium"
            type="reset"
          >
            Cancel
          </button>
          <button
            onclick={() => {
              isAddingEntry = false;
              /* entries.push(form.) */
            }}
            class="px-6 py-2 bg-slate-900 text-white rounded-lg font-medium hover:bg-slate-800 shadow-lg transition-all active:scale-95"
            type="submit"
          >
            Save
          </button>
        </div>
      </div>
    </div>
  </form>
{/if}

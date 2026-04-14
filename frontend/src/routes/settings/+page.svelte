<script lang="ts">
  import Settings from "@lucide/svelte/icons/Settings";
  import { enhance } from "$app/forms";
  import { browser } from "$app/environment";

    interface Settings {
        tokenExpiration : number,
        autoRefreshToken : boolean
    }

  const STORAGE_KEY = "settings";

  const OPTIONS = [
    { label: "15 minutes", value: 900 },
    { label: "60 minutes", value: 3600 },
    { label: "5 hours", value: 18000 },
    { label: "24 hours", value: 86400 },
    { label: "30 days", value: 2592000 },
    { label: "Never", value: 0xfffffffffff },
  ];

  let settings : Settings = $state({tokenExpiration : 3600, autoRefreshToken : false});

  // load from localStorage safely
  if (browser) {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
        settings = JSON.parse(stored);
    } else {
        // svelte-ignore state_referenced_locally
        localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
    }
  }

  function updateLocalStorage(tokenExpiration : number | null, autoRefreshToken : boolean | null) {
    if (tokenExpiration) {
        settings.tokenExpiration = tokenExpiration
    }
    if (autoRefreshToken) {
        settings.autoRefreshToken = autoRefreshToken
    }

    if (browser) {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
    }
  }

</script>

<main
  class="flex-1 flex flex-col items-center justify-center bg-slate-50 px-10"
>
  <div
    class="w-full max-w-xl bg-white shadow-xl shadow-blue-gray-900/5 rounded-xl p-8"
  >
    <!-- Header -->
    <div class="flex items-center gap-3 mb-6">
      <Settings class="text-slate-700" />
      <h2 class="text-2xl font-semibold text-slate-800">Settings</h2>
    </div>

    <div class="border-t border-slate-200 mb-6"></div>

    <!-- Form -->
    <form method="POST" action="/" class="space-y-6">
      <!-- JWT Expiry Select -->
      <div class="space-y-6">
        <!-- svelte-ignore a11y_label_has_associated_control -->
        <label class="text-lg text-slate-500 mb-2 block">
          Login Expiration
        </label>

        <div class="relative">
          <div class="relative">
            <select
              name="jwt_exp"
              bind:value={settings.tokenExpiration}
              onchange={(e) =>
                updateLocalStorage(
                  Number((e.target as HTMLSelectElement).value),
                  null
                )}
              class="
                  w-full
                  h-11
                  pl-3
                  pr-10
                  bg-white
                  border border-slate-200
                  rounded-lg
                  text-slate-700
                  text-lg
                  shadow-sm
                  appearance-none
                  transition
                  duration-150

                  hover:border-slate-300
                  hover:shadow

                  focus:outline-none
                  focus:ring-2
                  focus:ring-slate-300
                  focus:border-slate-400
              "
            >
              {#each OPTIONS as opt}
                <option value={opt.value}>
                  {opt.label}
                </option>
              {/each}
            </select>

            <!-- custom dropdown arrow -->
            <div
              class="absolute inset-y-0 right-3 flex items-center pointer-events-none"
            >
              <svg
                class="w-4 h-4 text-slate-400"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M19 9l-7 7-7-7"
                />
              </svg>
            </div>
          </div>
        </div>

        <p class="text-lg text-slate-400 mt-2">
          This controls how long you stay logged in.
        </p>
      </div>

      <!-- Auto refresh toggle -->
      <div
        class="flex items-center justify-between bg-slate-50 p-4 rounded-lg border border-slate-200"
      >
        <div>
          <p class="text-slate-700 font-medium">Auto refresh tokens</p>
          <p class="text-lg text-slate-400">Extend session when active</p>
        </div>

        <input
            type="checkbox"
            name="auto_refresh"
            class="h-5 w-5 accent-slate-700"
            bind:checked={settings.autoRefreshToken}
            onchange={(e) =>
            updateLocalStorage(
                null,
                (e.target as HTMLSelectElement).value == "true"
            )}
        />
      </div>

      <!-- Save -->
      <button
        type="submit"
        class="w-full bg-slate-800 text-white py-3 rounded-lg hover:bg-slate-700 transition"
      >
        Save Settings
      </button>
    </form>
  </div>
</main>

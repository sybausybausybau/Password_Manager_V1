<script lang="ts">
    import Settings from "@lucide/svelte/icons/Settings";
    import "./../layout.css";
    import type { PasswordEntryClean } from "./+page.server";
    import { fade, scale } from "svelte/transition";
    import { enhance } from "$app/forms";
    import DeleteConfirmation from "./DeleteConfirmation.svelte";
    import AddPassword from "./AddPassword.svelte";
    import ModifyPassword from "./ModifyPassword.svelte";

    let { data } = $props();
    let entries = $derived(data.entries);
    let filteredEntries: PasswordEntryClean[] = $derived(data.entries);
    let isAddingEntry = $state(false);
    let isModifyingEntry = $state(false);
    let activeEntry = $derived(filteredEntries[0]);
    let showModal = $state(false)

    let showPassword = $state(false);

    async function copyToClipboard(text: string) {
        navigator.clipboard.writeText(text);
    }

    function filterPasswords(filter: string) {
        let lst = [];
        for (const entry of entries) {
            if (entry.origin.toLowerCase().includes(filter.toLowerCase())) {
                lst.push(entry);
            }
        }
        return lst;
    }
</script>

<main class="h-screen overflow-hidden bg-white flex">
    <!-- SIDEBAR (UNCHANGED) -->
    <div
        class="no-scrollbar flex h-full overflow-y-auto overflow-hidden w-full max-w-100 flex-col rounded-xl bg-white bg-clip-border p-4 text-gray-700 shadow-xl shadow-blue-gray-900/5"
    >
        <div class="flex items-center justify-center gap-4 p-4 mb-2">
            <h5 class="text-xl font-semibold text-blue-gray-900">Passwords</h5>
            <AddPassword isAddingEntry={isAddingEntry} />
        </div>

        <div class="p-2">
            <div class="relative h-10 w-full min-w-50">
                <div class="absolute grid w-5 h-5 top-2/4 right-3 -translate-y-2/4 place-items-center text-blue-gray-500">
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
                        class="lucide lucide-search-icon lucide-search"
                        ><path d="m21 21-4.34-4.34" /><circle
                            cx="11"
                            cy="11"
                            r="8"
                        /></svg
                    >
                </div>

                <input
                    class="text-lg w-full pr-11 h-10 pl-3 py-2 bg-white placeholder:text-slate-400 text-slate-700 border border-slate-200 rounded focus:outline-none focus:border-slate-400 shadow-sm focus:shadow-md"
                    placeholder="Search..."
                    oninput={(e) => {
                        const value = (e.target as HTMLInputElement).value;
                        filteredEntries = filterPasswords(value);
                    }}
                />
            </div>
        </div>
        <nav
            class="flex flex-1 min-w-60 flex-col gap-1 p-2 text-base text-blue-gray-700"
        >
            {#each filteredEntries as entry}
                <div
                    class="group flex items-center justify-between w-full p-3 rounded-lg transition hover:bg-slate-100 overflow-hidden
                    {activeEntry?.id === entry.id ? 'bg-slate-200 font-medium' : ''}"
                >
                    <!-- Select entry -->
                    <button
                        class="flex-1 text-left"
                        onclick={() => (activeEntry = entry)}
                    >
                        <p class="font-bold">{entry.origin}</p>
                        <p>{entry.username}</p>
                    </button>

                    <!-- Delete button -->
                    <!-- svelte-ignore a11y_consider_explicit_label -->
                    <button
                        class="opacity-0 group-hover:opacity-100 transition text-red-400 hover:text-red-600 px-2"
                        onclick={() => {
                            showModal = true
                        }}                       
                    >

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
                            class="lucide lucide-trash2-icon lucide-trash-2"
                            ><path d="M10 11v6" /><path d="M14 11v6" /><path
                                d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"
                            /><path d="M3 6h18" /><path
                                d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
                            /></svg>
                    </button>
                    </div>
            {/each}

            <DeleteConfirmation 
                entry={activeEntry} 
                showModal={showModal}
                on:cancel={() => {
                    showModal = false 
                }}
                on:confirm={() => {
                    filteredEntries.filter(((entry) => {
                        entry.id !== activeEntry.id
                    }))
                    showModal = false
                }} 
            ></DeleteConfirmation>


            <!-- Logout -->
            <button
                class="flex gap-3 text-red-700 mt-auto items-center w-full p-3 rounded-lg hover:bg-slate-100"
            >
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
                    class="lucide lucide-log-out-icon lucide-log-out"
                    ><path d="m16 17 5-5-5-5" /><path d="M21 12H9" /><path
                        d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"
                    /></svg
                >
                Log Out
            </button>
        </nav>
    </div>




    <!-- RIGHT PANEL -->
    <section class="flex-1 flex flex-col">
        {#if activeEntry}
            <div class="h-full px-10 py-8 max-w-3xl">
                <!-- Header -->
                <div class="mb-8">
                    <h2 class="text-2xl font-semibold text-slate-800">
                        {activeEntry.origin}
                    </h2>
                    <p class="text-sm text-slate-400">
                        Added on {activeEntry.added_time}
                    </p>
                </div>

                <!-- Divider -->
                <div class="border-t border-slate-200 mb-6"></div>

                <!-- Credentials -->
                <div class="space-y-6">
                    <!-- Username -->
                    <div>
                        <p class="text-sm text-slate-400 mb-1">Username</p>
                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                        <div
                            class="flex items-center justify-between bg-slate-50 px-4 py-3 rounded-lg cursor-pointer hover:bg-slate-100"
                            onclick={() =>
                                copyToClipboard(activeEntry.username)}
                        >
                            <span class="text-slate-700">
                                {activeEntry.username}
                            </span>
                            <span class="text-xs text-slate-400">Copy</span>
                        </div>
                    </div>

                    <!-- Password -->
                    <div>
                        <p class="text-sm text-slate-400 mb-1">Password</p>
                        <div class="flex items-center gap-2">
                            <!-- svelte-ignore a11y_click_events_have_key_events -->
                            <!-- svelte-ignore a11y_no_static_element_interactions -->
                            <div
                                class="flex-1 flex items-center justify-between bg-slate-50 px-4 py-3 rounded-lg cursor-pointer hover:bg-slate-100"
                                onclick={() =>
                                    copyToClipboard(activeEntry.password)}
                            >
                                <span class="text-slate-700 tracking-wider">
                                    {showPassword
                                        ? activeEntry.password
                                        : "••••••••"}
                                </span>
                                <span class="text-xs text-slate-400">Copy</span>
                            </div>

                            <button
                                class="px-3 py-2 text-sm bg-slate-100 rounded hover:bg-slate-200"
                                onclick={() => (showPassword = !showPassword)}
                            >
                                {showPassword ? "Hide" : "Show"}
                            </button>
                        </div>
                    </div>
                </div>
                <ModifyPassword isModifyingEntry={isModifyingEntry} activeEntry={activeEntry} />
            </div>
        {:else}
            <!-- Empty state -->
            <div class="flex items-center justify-center h-full text-slate-400">
                Select an entry
            </div>
        {/if}
    </section>
</main>

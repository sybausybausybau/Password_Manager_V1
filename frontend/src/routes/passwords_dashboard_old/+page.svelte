<script lang="ts">
    import Settings from "@lucide/svelte/icons/Settings";
    import "./../layout.css";
    import type { PasswordEntryClean } from "./+page.server";
    import { fade, scale } from "svelte/transition";
    import { enhance } from "$app/forms";

    let { data } = $props();
    let entries = $derived(data.entries);
    let filteredEntries: PasswordEntryClean[] = $derived(data.entries);
    let isAddingEntry = $state(false);

    const options = [
        { id: "add-task:child", text: "Child task" },
        { id: "add-task:above", text: "Task above" },
        { id: "add-task:below", text: "Task below" },
    ];

    async function copyToClipboard(text: string) {
        navigator.clipboard.writeText(text);
    }

    function filterPasswords(filter: string) {
        let lst = [];
        for (const entry of entries) {
            if (entry.origin.includes(filter)) {
                lst.push(entry);
            }
        }
        return lst;
    }
</script>

<main class="min-h-screen bg-white flex flex-col">
    <section class="flex-1 flex flex-col items-center p-4 pt-[20vh]">
        <div
            class="w-full max-w-max flex justify-between items-center mb-7 mt-1 px-3 gap-8"
        >
            <div>
                <h3 class="text-xl font-semibold text-slate-800">
                    All your passwords
                </h3>
                <p class="text-slate-500 text-sm">{entries.length} Passwords</p>
            </div>
            <div>
                <div class="flex gap-6 items-center">
                    <input
                        class="text-lg w-full pr-11 h-10 pl-3 py-2 bg-white placeholder:text-slate-400 text-slate-700 border border-slate-200 rounded transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-400 shadow-sm focus:shadow-md"
                        placeholder="Search..."
                        oninput={(e) => {
                            const value = (e.target as HTMLInputElement).value;
                            filteredEntries = filterPasswords(value);
                        }}
                    />
                    <!-- svelte-ignore a11y_consider_explicit_label -->
                    <button
                        class="absolute h-8 w-8 right-1 top-1 my-auto px-2 flex items-center bg-white rounded"
                        type="button"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="3"
                            stroke="currentColor"
                            class="w-5 h-5 text-slate-600"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z"
                            />
                        </svg>
                    </button>
                    <!-- svelte-ignore a11y_consider_explicit_label -->
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
                            ><circle cx="12" cy="12" r="10" /><path
                                d="M8 12h8"
                            /><path d="M12 8v8" /></svg
                        >
                    </button>
                    {#if isAddingEntry}
                        <form method="POST" use:enhance>
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
                                    <h3
                                        class="text-xl font-bold text-slate-800 mb-4"
                                    >
                                        Add a New Password
                                    </h3>

                                    <div class="space-y-4">
                                        <div>
                                            <!-- svelte-ignore a11y_label_has_associated_control -->
                                            <label
                                                class="text-xs font-semibold text-slate-500 uppercase"
                                                >Service</label
                                            >
                                            <input
                                                name="origin"
                                                placeholder="e.g. Netflix, Amazon, Youtube..."
                                                class="w-full mt-1 border border-slate-200 rounded-lg p-2 focus:ring-2 focus:ring-slate-200 outline-none transition-all"
                                            />
                                        </div>
                                        <div>
                                            <!-- svelte-ignore a11y_label_has_associated_control -->
                                            <label
                                                class="text-xs font-semibold text-slate-500 uppercase"
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
                                            <label
                                                class="text-xs font-semibold text-slate-500 uppercase"
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
                                            onclick={() =>
                                                (isAddingEntry = false)}
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
                </div>
            </div>
        </div>

        <div
            class="relative no-scrollbar flex flex-col w-auto max-w-max max-h-150 overflow-y-auto h-full overflow-hidden text-gray-700 bg-white shadow-md rounded-lg bg-clip-border"
        >
            <table class="text-left overflow-y table-auto">
                <thead>
                    <tr>
                        <th class="p-4 border-b border-slate-300 bg-slate-50">
                            <p
                                class="block text-sm font-semibold leading-none text-slate-500"
                            >
                                Service
                            </p>
                        </th>
                        <th class="p-4 border-b border-slate-300 bg-slate-50">
                            <p
                                class="block text-sm font-semibold leading-none text-slate-500"
                            >
                                Username
                            </p>
                        </th>
                        <th class="p-4 border-b border-slate-300 bg-slate-50">
                            <p
                                class="block text-sm font-semibold leading-none text-slate-500"
                            >
                                Password
                            </p>
                        </th>
                        <th class="p-4 border-b border-slate-300 bg-slate-50">
                            <p
                                class="block text-sm font-semibold leading-none text-slate-500"
                            >
                                Added on
                            </p>
                        </th>
                        <th class="p-4 border-b border-slate-300 bg-slate-50">
                            <p
                                class="block text-sm font-semibold leading-none text-slate-500"
                            >
                                Action
                            </p>
                        </th>
                    </tr>
                </thead>
                <tbody id="entryList">
                    {#each filteredEntries as entry}
                        <tr class="hover:bg-slate-50" id={entry.id}>
                            <td class="p-4 border-b border-slate-200 py-5">
                                <p
                                    class="block font-semibold text-sm text-slate-800"
                                >
                                    {entry.origin}
                                </p>
                            </td>
                            <td
                                class="p-4 border-b cursor-pointer border-slate-200 py-5"
                                onclick={async () => await copyToClipboard(entry.username)}
                                title="Copy username"
                            >
                                <p class="text-sm text-slate-500">
                                    {entry.username}
                                </p>
                            </td>
                            <td
                                class="p-4 border-b cursor-pointer border-slate-200 py-5"
                                onclick={async () => await copyToClipboard(entry.password)}
                                title="Copy password"
                            >

                                <p class="text-sm text-slate-500">
                                    {entry.password}
                                </p>
                            </td>
                            <td class="p-4 border-b border-slate-200 py-5">
                                <p class="text-sm text-slate-500">
                                    {entry.added_time}
                                </p>
                            </td>
                            <td
                                class="p-4 border-b border-slate-200 py-5 flex justify-center items-center"
                            >
                                <!-- svelte-ignore a11y_consider_explicit_label -->

                                <!-- <DropDownMenu at="right" {options}>
                                    <Button css="padding:8px; margin-left:10px;">
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
                                            class="lucide lucide-ellipsis-vertical-icon lucide-ellipsis-vertical"
                                            ><circle
                                                cx="12"
                                                cy="12"
                                                r="1"
                                            /><circle
                                                cx="12"
                                                cy="5"
                                                r="1"
                                            /><circle
                                                cx="12"
                                                cy="19"
                                                r="1"
                                            /></svg>
                                    </Button>
                                </DropDownMenu> -->
                                <div class="dropdown" data-placement="right-start">
                                    <button data-toggle="dropdown" aria-expanded="false" >

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
                                            class="lucide lucide-ellipsis-vertical-icon lucide-ellipsis-vertical"
                                            ><circle
                                                cx="12"
                                                cy="12"
                                                r="1"
                                            /><circle
                                                cx="12"
                                                cy="5"
                                                r="1"
                                            /><circle
                                                cx="12"
                                                cy="19"
                                                r="1"
                                            /></svg>

                                    </button>
                                    <div data-role="menu" class="hidden mt-2 bg-white border border-slate-200 rounded-lg shadow-xl shadow-slate-950/2.5 p-1 z-10">
                                        <a href="/" class="block px-4 py-2 text-sm text-slate-600 hover:text-slate-800 hover:bg-slate-200 rounded-md">Add Team</a>
                                        <a href="/" class="block px-4 py-2 text-sm text-slate-600 hover:text-slate-800 hover:bg-slate-200 rounded-md">Add Project</a>
                                        <a href="/" class="block px-4 py-2 text-sm text-slate-600 hover:text-slate-800 hover:bg-slate-200 rounded-md">My Profile</a>
                                        <div class="h-px bg-slate-200 my-1"></div>
                                        <a href="/" class="block px-4 py-2 text-sm text-red-500 hover:bg-red-100 rounded-md">Logout</a>
                                    </div>
                                </div>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </section>
    <section class="pl-[4vh] pb-6">
        <a
            href="/settings"
            class="flex items-center w-fit gap-3 transition duration-300 ease active:translate-y-1.5"
        >
            <Settings color="#364153" size={50} />
        </a>
    </section>
</main>

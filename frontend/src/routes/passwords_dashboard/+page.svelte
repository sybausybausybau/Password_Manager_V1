<script lang="ts">
    import type { PageProps } from "./$types";
    import type { PasswordEntryClean } from "./+page.server";

    let props: PageProps = $props();
    let entries = $derived(props.data.entries);

</script>

<main class="min-h-screen flex flex-col justify-center items-center p-4 bg-slate-50">
    
    <div class="w-full max-w-max flex justify-between items-center mb-3 mt-1 px-3 gap-8">
        <div>
            <h3 class="text-lg font-semibold text-slate-800">All your passwords</h3>
            <p class="text-slate-500 text-sm">Overview of your entries.</p>
        </div>
        <div>
            <div class="relative">
                <input
                    class="w-full pr-11 h-10 pl-3 py-2 bg-white placeholder:text-slate-400 text-slate-700 text-sm border border-slate-200 rounded transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-400 shadow-sm focus:shadow-md"
                    placeholder="Search..."
                />
                <!-- svelte-ignore a11y_consider_explicit_label -->
                <button
                    class="absolute h-8 w-8 right-1 top-1 my-auto px-2 flex items-center bg-white rounded"
                    type="button"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="3" stroke="currentColor" class="w-5 h-5 text-slate-600">
                        <path stroke-linecap="round" stroke-linejoin="round" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
                    </svg>
                </button>
            </div>
        </div>
    </div>
    
    <div class="relative flex flex-col w-auto max-w-max h-full overflow-hidden text-gray-700 bg-white shadow-md rounded-lg bg-clip-border">
        <table class="text-left table-auto">
            <thead>
                <tr>
                    <th class="p-4 border-b border-slate-300 bg-slate-50">
                        <p class="block text-sm font-semibold leading-none text-slate-500">Service</p>
                    </th>
                    <th class="p-4 border-b border-slate-300 bg-slate-50">
                        <p class="block text-sm font-semibold leading-none text-slate-500">Username</p>
                    </th>
                    <th class="p-4 border-b border-slate-300 bg-slate-50">
                        <p class="block text-sm font-semibold leading-none text-slate-500">Password</p>
                    </th>
                    <th class="p-4 border-b border-slate-300 bg-slate-50">
                        <p class="block text-sm font-semibold leading-none text-slate-500">Last Modified</p>
                    </th>
                </tr>
            </thead>
            <tbody>
                {#each entries as entry}
                    <tr class="hover:bg-slate-50">
                        <td class="p-4 border-b border-slate-200 py-5">
                            <p class="block font-semibold text-sm text-slate-800">{entry.origin}</p>
                        </td>
                        <td class="p-4 border-b border-slate-200 py-5">
                            <p class="text-sm text-slate-500">{entry.username}</p>
                        </td>
                        <td class="p-4 border-b border-slate-200 py-5">
                            <p class="text-sm text-slate-500">{entry.password}</p>
                        </td>
                        <td class="p-4 border-b border-slate-200 py-5">
                            <p class="text-sm text-slate-500">{entry.added_time}</p>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
</main>

<script lang="ts">
    import { page } from '$app/state';
    import { redirect } from '@sveltejs/kit';
</script>

<main class="min-h-screen flex flex-col items-center justify-center gap-5!">
    <p class="text-4xl font-bold text-slate-800">HTTPS {page.status} Error</p>
    <p class="text-3xl font-bold text-slate-800">{page.error?.message}</p>
    
    <button
        onclick={() => redirect(303, "/")}
        class="rounded-md transform-y-1.5 font-bold bg-slate-800 py-2 px-4 border border-transparent text-center text-3xl text-white transition-all shadow-md hover:shadow-lg focus:bg-slate-700 focus:shadow-none active:bg-slate-700 hover:bg-slate-700 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none ml-2" type="button">
    Go Back
    </button>
</main>
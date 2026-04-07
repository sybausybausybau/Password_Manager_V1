<script lang="ts">
    interface LoginInfo {
        username : string,
        masterPassword : string,
    }
</script>

<main class="h-[100vh]! bg-black!">
    <form
        on:submit={handleSubmit}
        class="max-w-md mx-auto mt-10 p-6 bg-white rounded-2xl shadow-lg space-y-4"
    >
        <h2 class="text-2xl font-bold text-gray-800 text-center">
            Contact
        </h2>

        <input
            type="text"
            placeholder="Nom"
            bind:value={name}
            required
            class="w-full p-3 border rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500"
        />

        <input
            type="email"
            placeholder="Email"
            bind:value={email}
            required
            class="w-full p-3 border rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500"
        />

        <textarea
            placeholder="Message"
            bind:value={message}
            required
            class="w-full p-3 border rounded-xl h-32 focus:outline-none focus:ring-2 focus:ring-blue-500"
        ></textarea>

        <button
            type="submit"
            disabled={loading}
            class="w-full bg-blue-600 text-white py-3 rounded-xl hover:bg-blue-700 transition disabled:opacity-50"
        >
            {loading ? 'Envoi...' : 'Envoyer'}
        </button>

        {#if success === true}
            <p class="text-green-600 text-center">Message envoyé ✅</p>
        {:else if success === false}
            <p class="text-red-600 text-center">Erreur lors de l'envoi ❌</p>
        {/if}
    </form>
</main>


<script lang="ts">
    import PasswordsTable from './PasswordsTable.svelte';

</script>

<svelte:head>
	<title>Password Manager</title>
	<meta name="description" content="Password Manager" />
</svelte:head>

<main class="flex flex-col h-screen">
    <PasswordsTable token=""/>
</main>
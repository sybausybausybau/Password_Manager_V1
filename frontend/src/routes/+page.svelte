<script lang="ts">
	import { enhance } from "$app/forms";
  	import Info from '@lucide/svelte/icons/Info';
  	import Eye from '@lucide/svelte/icons/Eye';

	let { form } = $props();

	let isSigningIn = $state(true);
	let showPassword =$state(false);
	let localErrors = $derived(form?.errors);

	function toggleShowPassword() {
		showPassword = !showPassword;
	}

	function toggleMode() {
		isSigningIn = !isSigningIn;
		localErrors = undefined;
	}


</script>

<svelte:head>
	<title>Password Manager</title>
	<meta name="description" content="Password Manager" />
</svelte:head>

<main class="h-screen flex items-center flex-col">
	<div class="flex flex-col mt-[30vh] p-8 gap-4">
		<div class="flex flex-col">
			<p class="font-bold text-3xl">{isSigningIn ? "Sign In" : "Create an account"}</p>
			<button 
				onclick={toggleMode} 
				class="font-semibold text-xl underline text-left cursor-pointer" 
			>
				{isSigningIn ? "Or create an account" : "Or sign into your account"}
			</button>
		</div>

		<!-- * https://www.material-tailwind.com/docs/html/input -->
 		<form method="POST" action="/" use:enhance> 

			<input type="hidden" name="isSigningIn" value={isSigningIn} />

			<div class="flex flex-col items-center justify-center gap-4">
				<div class="w-full max-w-sm min-w-100">
					<input 
						class="text-xl w-full bg-transparent placeholder:text-slate-400 text-slate-700 border border-slate-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow" 
						placeholder="Username or Email"
						name="username"
						value={form?.username}
					>
				</div>

			{#if localErrors?.username}
				{#each localErrors?.username as error}
					<p class="text-red-500 text-[18px]">{error}</p>
				{/each}
            {/if}

				<div class="w-full max-w-sm min-w-100">
					<div class="relative">
						<input 
							type={showPassword ? "text" : "password"} 
							class={`text-xl w-full pl-3 pr-3 py-2 bg-transparent placeholder:text-slate-400 text-slate-600 border border-slate-200 rounded-md 
							transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow ${form?.errors?.username ? "border-red-500, text-red-500" : ""}`
							}
							placeholder="Password"
							name="password"
						/>
						
						<button 
							class="absolute right-1 top-1 pl-3 pr-3 py-2" 
							onclick={toggleShowPassword}
							type="button"
						>
							<Eye color="#1d293d" />
						</button>
						{#if localErrors?.password}
							{#each localErrors?.password as error}
								<p class="text-red-500 text-[18px]">{error}</p>
							{/each}
						{:else}
							<div class="flex gap-2 justify-center items-center">
								<Info class="w-9 h-9" color="#90a1b9"/>
								<p class="flex items-start mt-2 text-[18px] text-slate-400">
									Use at least 8 characters, one uppercase, one lowercase and one number.
								</p>    
							</div>
						{/if}

					</div>
				</div>

				<button 
					class={`ease-in active:translate-y-1.5 font-semibold text-xl min-w-100 rounded-md bg-slate-800 py-2 px-4 border border-transparent 
					text-center text-white transition-all shadow-md hover:shadow-lg focus:bg-slate-700 focus:shadow-none active:bg-slate-700
					hover:bg-slate-700 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none ml-2 `} 
					type="submit"
				>
				
					{isSigningIn ? "Sign In" : "Create Account"}
				</button>
			</div>
		</form> 
	</div> 
</main>

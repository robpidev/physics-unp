<script>
	import { onDestroy, onMount } from 'svelte';
	import { page } from '$app/stores';
	import { user } from '$lib/stores';

	let usr;
	onMount(async () => {
		usr = JSON.parse(localStorage.getItem('user'));
	});

	const user_sus = user.subscribe((value) => {
		usr = value;
	});
	onDestroy(user_sus);

	function signout() {
		user.update(() => null);
		localStorage.removeItem('user');
		//goto('/');
	}
</script>

<header>
	<a class="logo" href="/">
		<img src="favicon.png" alt="" />
		<span>Física UNP</span>
	</a>

	<nav>
		<ul>
			{#if usr !== null}
				<li>
					<a href="/student" class="link">Mi perfil</a>
				</li>
				<li>
					<a on:click={signout} data-sveltekit-preload-data={false} class="link" href="/signin"
						>Salir</a
					>
				</li>
			{:else if !$page.url.pathname.includes('/signin') && !$page.url.pathname.includes('/signup')}
				<li><a class="signin" href="/signin">Iniciar sesión</a></li>
			{/if}
		</ul>
	</nav>
</header>
<main>
	<slot></slot>
</main>

<style>
	header {
		height: 58px;
		background: var(--bg);
		border-bottom: 1px solid var(--border);
		display: flex;
		padding: 0 1em;
		justify-content: space-between;
	}

	.logo {
		/*background: var(--primary);*/
		display: flex;
		align-items: center;
		gap: 0.5em;
	}

	.logo img {
		height: 40px;
	}

	.logo span {
		/*background: purple;*/
		font-weight: 700;
		font-size: 1.5rem;
	}

	a {
		text-decoration: none;
		color: var(--primary);
	}

	a:hover {
		color: var(--hover);
	}

	nav ul {
		padding: 0.2em 1em;
		list-style: none;
		display: flex;
		gap: 1em;
	}

	/*.name {*/
	/*	background: var(--primary);*/
	/*	color: white;*/
	/*	padding: 0.5em 1em;*/
	/*	border-radius: 12px;*/
	/*	pointer-events: none;*/
	/*}*/
	/**/
	.link {
		font-weight: 700;
	}

	.signin {
		font-weight: 700;
		background: var(--primary);
		color: white;
		padding: 0.5em 1em;
		border-radius: 12px;
	}

	.signin:hover {
		background: var(--hover);
		color: white;
	}

	.signin:active {
		background: var(--active);
		color: white;
	}

	main {
		display: flex;
		justify-content: center;
	}
</style>

<script>
	import { fly, fade } from 'svelte/transition';
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import { user } from '$lib/stores.js';

	export let data;

	let code = '';
	$: code_name = code.length === 10 ? 'code' : 'dni';
	let id = -1;
	let pass = '';
	let pass1 = '';
	let pass_msg = '';
	let pas_con = '';
	let cod_msg = '';
	let server_msg = '';

	function check_code() {
		if (/^\d+$/.test(code) && (code.length === 10 || code.length === 8)) {
			cod_msg = '';
		} else {
			cod_msg = 'Código incorrecto';
		}
	}

	let loading = false;
</script>

<div class="content">
	<h1>Crear nueva cuenta</h1>
	<form
		method="POST"
		action="?/signup"
		use:enhance={() => {
			loading = true;
			return async ({ result }) => {
				if (result.status === 200) {
					user.update(() => result.data.user);
					goto('/newuser');
				} else {
					server_msg = result.data.message;
					loading = false;
				}
			};
		}}
	>
		<label>
			<span>Código:</span>
			<input on:keyup={check_code} name={code_name} type="text" required bind:value={code} />
			{#if cod_msg != ''}
				<span class="error" in:fade>{cod_msg}</span>
			{/if}
		</label>
		<label>
			<span>Nombres: </span>
			<input name="names" type="text" required />
		</label>
		<label>
			<span>Apellido paterno</span>
			<input name="last_name1" type="text" required />
		</label>
		<label>
			<span>Apellido materno</span>
			<input name="last_name2" type="text" required />
		</label>
		<label>
			<span>Contraseña:</span>
			<input
				on:keyup={() => (pass.length < 8 ? (pass_msg = 'Minimo 8 caracteres') : (pass_msg = ''))}
				name="password"
				type="password"
				bind:value={pass}
				required
			/>
			{#if pass_msg != ''}
				<span class="error" in:fade>{pass_msg}: {pass.length}</span>
			{/if}
		</label>
		<label>
			<span>Confirmar contraseña: </span>
			<input
				on:keyup={() =>
					pass === pass1 ? (pas_con = '') : (pas_con = 'Las contraseñas no coinciden')}
				bind:value={pass1}
				type="password"
				required
			/>
			{#if pas_con != ''}
				<span class="error" in:fade>{pas_con}</span>
			{/if}
		</label>

		<div class="gender">
			<span>Género:</span>
			<label>
				<input type="radio" name="gender" value="true" required />
				Masculino
			</label>
			<label>
				<input type="radio" name="gender" value="false" required />
				Femenino
			</label>
		</div>

		{#if code.length === 10 && cod_msg == ''}
			<label in:fly={{ y: 200 }} out:fly={{ y: 200 }}>
				<span>Escuela: </span>
				<select bind:value={id} name="school_id" id="schol" required>
					{#each data.schools as school, index}
						<option value={data.schools[index].id}>{school.name}</option>
					{/each}
				</select>
			</label>
		{/if}

		<button type="submit" disabled={cod_msg != '' || pass != pass1 || loading}>Registrarse</button>
	</form>

	{#if server_msg != ''}
		<p class="error_server">{server_msg}</p>
	{/if}

	{#if loading}
		<p class="loading">Registrando...</p>
	{:else}
		<p>
			¿Ya tienes una cuenta?
			<a href="/signin">Inicia sesión</a>
		</p>
	{/if}
</div>

<style>
	.content {
		padding: 1em;
		display: flex;
		align-items: center;
		justify-content: center;
		display: flex;
		flex-direction: column;
		gap: 1em;
	}

	.error_server {
		color: red;
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 1.2em;
		border: solid 1px var(--border);
		padding: 1.5em;
		border-radius: 12px;
		flex-flow: wrap;
		max-width: 540px;
	}

	form > label,
	.gender {
		display: flex;
		position: relative;
		flex-direction: column;
		gap: 0.5rem;
		flex-grow: 1;
		min-width: 200px;
	}

	.error {
		position: absolute;
		bottom: -1.2em;
		right: 0;
		font-size: 0.9rem;
		color: red;
	}
	.gender {
		width: 100%;
		max-width: 237px;
	}

	.gender label {
		display: flex;
		align-items: center;
	}

	.gender input {
		height: min-content;
	}

	.gender input:focus {
		border: 0;
	}

	form > label > input,
	select {
		border-radius: 6px;
		border: solid 1px var(--border);
		height: 2.3em;
	}

	form > label > input:focus,
	select:focus {
		outline: solid 2px var(--primary);
		/*outline: none;*/
	}
	button {
		/*margin: 1em 0 0.5em 0;*/
		width: 100%;
	}

	button:disabled {
		background: var(--color-300);
	}

	a {
		color: var(--primary);
		text-decoration: none;
		font-weight: 600;
	}

	a:hover {
		color: var(--hover);
	}

	a:active {
		color: var(--active);
	}
	.loading {
		color: green;
	}
</style>

<script>
	import { fly, scale } from 'svelte/transition';
	import { enhance } from '$app/forms';
	export let data;
	let code = '';
	let id = -1;
</script>

<div class="content">
	<h1>Crear nueva cuenta</h1>
	<form method="POST" action="/send" use:enhance>
		<label>
			<span>Código: </span>
			<input type="text" required bind:value={code} />
		</label>
		<label>
			<span>Nombres: </span>
			<input type="text" required />
		</label>
		<label>
			<span>Apellido paterno</span>
			<input type="text" required />
		</label>
		<label>
			<span>Apellido materno</span>
			<input type="text" required />
		</label>
		<label>
			<span>Contraseña: </span>
			<input type="password" required />
		</label>
		<label>
			<span>Confirmar contraseña: </span>
			<input type="password" required />
		</label>

		<div class="gender">
			<span>Género:</span>
			<label>
				<input type="radio" name="gender" value="male" required />
				Hombre
			</label>
			<label>
				<input type="radio" name="gender" value="female" required />
				Mujer
			</label>
		</div>

		{#if code.length === 10}
			<label in:fly={{ y: 200 }} out:fly={{ y: 200 }}>
				<span>Escuela: </span>
				<select bind:value={id} name="school" id="schol" required>
					{#each data.schools as school, id}
						<option value={id}>{school.name}</option>
					{/each}
				</select>
			</label>
		{/if}

		<button>Registrarse</button>
	</form>

	<p>
		¿Ya tienes una cuenta?
		<a href="/signin">Inicia sesión</a>
	</p>
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

	form {
		display: flex;
		flex-direction: column;
		gap: 1em;
		border: solid 1px var(--border);
		padding: 1.5em;
		border-radius: 12px;
		flex-flow: wrap;
		max-width: 540px;
	}

	form > label,
	.gender {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		flex-grow: 1;
		min-width: 200px;
	}

	.gender {
		max-width: 237px;
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
		margin: 1em 0 0.5em 0;
		background: var(--primary);
		color: white;
		padding: 0.5em 1em;
		border-radius: 6px;
		border: none;
		font-weight: 600;
		width: 100%;
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
</style>

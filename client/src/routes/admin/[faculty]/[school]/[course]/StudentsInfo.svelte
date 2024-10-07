<script>
	import { enhance } from '$app/forms';
	export let students = [];
</script>

{#if students?.length > 0}
	<h2>Datos de los alumnos</h2>
	<hr />

	<div class="table">
		<!-- content here -->
		<span class="head">Código</span>
		<span class="head">Nombres</span>
		<span class="head">Email <button class="copy" aria-label="Copy Email">Copiar</button></span>
		<span class="head">Grupos ocupados</span>
		{#each students as student}
			<span class="code">
				{student.id}
			</span>
			<span class="name">
				{student.name}
			</span>
			<span class="email">{student.email}</span>
			<span class="groups">
				{student.ocupated.join(', ')}
			</span>
		{/each}
	</div>
{:else}
	<form method="post" action="?/students_info" use:enhance>
		<button type="submit">Datos adicionales de los alumnos</button>
	</form>
{/if}

<style>
	button {
		float: right;
	}

	.table {
		display: grid;
		grid-template-columns: 0.5fr 1fr 2fr 1fr;
		gap: 2px;
	}

	.code {
		text-align: center;
		min-width: 100px;
	}

	.groups {
		text-align: right;
	}

	.head {
		background: var(--color-200);
	}

	span {
		border-radius: 3px;
		padding: 2px 5px;
		background: var(--bg);
	}

	.copy {
		background: none;
		border: none;
		padding: 0;
		margin: 0;
		color: green;
	}

	.copy:hover {
		text-decoration: underline;
	}
</style>

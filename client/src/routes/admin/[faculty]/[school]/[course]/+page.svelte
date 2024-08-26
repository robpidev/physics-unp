<script>
	import { onMount } from 'svelte';
	import AssignButton from './AssignButton.svelte';
	let professors = [];
	async function getProfessors() {
		const url = '/api/professors';
		const resp = await fetch(url);
		professors = await resp.json();
	}

	export let data;
</script>

{#if data.professors.length > 0}
	<h2>Profesores del curso</h2>
	{#each data.professors as professor}
		<ul>
			<li class="professor">
				<div class="info">
					<span class="name">{professor.names}</span>
					<span class="lastname">{professor.last_name1} {professor.last_name2}</span>
				</div>

				<div class="buttons">
					<span>{professor.role}</span>
					<button>Quitar</button>
				</div>
			</li>
		</ul>
	{/each}
{:else}
	<h2>No hay profesores asignados al curso</h2>
{/if}

{#if professors.length > 0}
	<h3>Eliga el profesor que quiere asignar al curso</h3>
	{#each professors as professor}
		<ul>
			{#if !data.professors.some((p) => p.id == professor.id)}
				<!-- content here -->
				<li class="professor">
					<div class="info">
						<span class="name">{professor.names}</span>
						<span class="lastname">{professor.last_name1} {professor.last_name2}</span>
					</div>
					<div class="buttons">
						<AssignButton
							role="practice"
							user_id={professor.id}
							on:assigned={() => {
								professors = professors.filter((p) => !data.professors.some((cp) => cp.id == p.id));
							}}>Práctica</AssignButton
						>
						<AssignButton
							role="teory"
							user_id={professor.id}
							on:assigned={() => {
								professors = professors.filter((p) => !data.professors.some((cp) => cp.id == p.id));
							}}>Teoría</AssignButton
						>
					</div>
				</li>
			{/if}
		</ul>
	{/each}
{:else}
	<button on:click={getProfessors}>Asignar profesor</button>
{/if}

<style>
	ul {
		list-style: none;
		padding: 0;
		margin: 0;
	}
	.professor {
		gap: 1em;
		display: flex;
		justify-content: space-between;
		border: solid 1px var(--border);
		padding: 0.5em 1em;
		margin: 5px;
		align-items: center;
		border-radius: 8px;
		flex-wrap: wrap;
	}

	.buttons {
		display: flex;
		gap: 5px;
	}
</style>

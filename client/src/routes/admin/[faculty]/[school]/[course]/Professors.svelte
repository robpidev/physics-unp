<script>
	import AssignButton from './AssignButton.svelte';
	import DeleteButton from './DeleteButton.svelte';
	export let professorsAssigneds;

	let professors = [];
	async function getProfessors() {
		const url = '/api/professors';
		const resp = await fetch(url);
		professors = await resp.json();
	}
</script>

{#if professorsAssigneds?.length > 0}
	<h2>Profesores del curso</h2>
	{#each professorsAssigneds as professor}
		<ul>
			<li class="professor">
				<div class="info">
					<span class="name">{professor.full_name}</span>
				</div>

				<div class="buttons">
					<span>{professor.role}</span>
					<DeleteButton user_id={professor.id} />
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
			{#if !professorsAssigneds.some((p) => p.id == professor.id)}
				<!-- content here -->
				<li class="professor">
					<div class="info">
						<span class="name">{professor.names}</span>
						<span class="lastname">{professor.last_name1} {professor.last_name2}</span>
					</div>
					<div class="buttons">
						<AssignButton role="practice" user_id={professor.id}>Práctica</AssignButton>
						<AssignButton role="theory" user_id={professor.id}>Teoría</AssignButton>
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
		align-items: center;
		/*border: solid 1px var(--border);*/
		background: var(--bg);
		padding: 0em 1em;
		margin: 5px;
		align-items: center;
		border-radius: 5px;
		flex-wrap: wrap;
	}

	.buttons {
		display: flex;
		gap: 5px;
	}

	.buttons span {
		width: 4em;
	}
</style>

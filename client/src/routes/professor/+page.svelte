<script>
	let { data } = $props();
</script>

<section>
	<h1>Cursos</h1>
	<div>
		<!-- TODO: No course not found -->
		{#if data.courses}
			<ul class="courses">
				{#each data.courses as course}
					<li>
						<a href="/professor/{course.id}" data-sveltekit-preload-data="false">
							<span class="name">{course.name}</span>
							<div class="info">
								<span class={course.role}>{course.role == 'theory' ? 'Teoría' : 'Práctica'}</span>
								<span class="school">{course.school}</span>
							</div>
						</a>
					</li>
				{/each}
			</ul>
		{:else}
			<p>No se a asignado ningún curso.</p>{/if}
	</div>
</section>

<style>
	.legend {
		display: flex;
	}

	section {
		background: white;
		padding: 1em 1em;
		border-radius: 8px;
		width: 100%;
		max-width: 800px;
	}

	ul {
		display: flex;
		gap: 5px;
		width: 100%;
		list-style: none;
		margin: 0;
		padding: 0;
		flex-direction: column;
	}

	.theory {
		color: red;
		width: 50pt;
	}

	.practice {
		width: 50pt;
		color: green;
	}

	li {
		background: var(--bg);
		width: 100%;
		padding: 0.3em 1em;
		border-radius: 3px;
	}

	.info {
		display: flex;
		justify-content: space-between;
	}

	a {
		width: 100%;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
	}
</style>

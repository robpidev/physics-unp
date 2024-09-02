<script>
	export let data;
</script>

<section>
	<h1>Cursos</h1>
	<div>
		<ul class="courses">
			{#each data.courses as course}
				<li>
					<a href="/professor/{course.id}">
						<span class="name">{course.name}</span>
						<span class="school">{course.school}</span>
					</a>
				</li>
			{/each}
		</ul>
	</div>
</section>

<style>
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

	li {
		width: 100%;
		background: var(--bg);
		padding: 0.3em 1em;
		border-radius: 8px;
	}

	a {
		width: 100%;
		display: flex;
		justify-content: space-between;
	}
</style>

<script>
	import { onMount } from 'svelte';
	import Courses from './Courses.svelte';
	import Evaluations from './Evaluations.svelte';
	import { enhance } from '$app/forms';

	let disabled = false;
	let selected = null;

	let usr;

	export let data;
	onMount(() => {
		usr = JSON.parse(localStorage.getItem('user'));
	});
	//const user_suscribe = user.subscribe((u) => (usr = u));
	//onDestroy(user_suscribe);
	export let form;
</script>

<div class="page">
	<div class="info">
		<span class="name">
			{usr?.names}
			{usr?.last_name1}
			{usr?.last_name2}
		</span>
		<span class="code">{usr?.id}</span>
	</div>

	<section>
		<h2>Cursos matrículados</h2>
		<hr />

		{#if data?.courses.length > 0}
			<ul class="courses">
				{#each data?.courses as course}
					<li class="course">
						<span class="course-name">{course?.name}</span>
						{#if selected != course.id}
							<form
								action="?/scores"
								method="post"
								use:enhance={() => {
									disabled = true;
									return async ({ update }) => {
										await update();
										selected = course.id;
										disabled = false;
									};
								}}
							>
								<input
									hidden
									on:submit|preventDefault
									type="text"
									name="course_id"
									value={course?.id}
								/>
								<button type="submit" {disabled}>Ver notas</button>
							</form>
						{/if}
					</li>
					<!-- content here -->
				{/each}
			</ul>
		{:else}
			<Courses />
		{/if}
	</section>

	{#if form?.evaluations}
		<!-- content here -->
		<section>
			<Evaluations evaluations={form.evaluations} />
		</section>
	{/if}
</div>

<style>
	.info {
		display: flex;
		width: 100%;
		align-items: center;
		justify-content: space-between;
		margin: 0em 0 1em 0;
	}
	section {
		width: 100%;
		padding: 1em;
	}

	.page {
		display: flex;
		gap: 1em;
		flex-direction: column;
		width: 100%;
		max-width: 1000px;
	}

	.name {
		font-size: 1.3em;
		font-weight: 700;
	}

	ul {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 1em;
	}

	.course {
		display: flex;
		background: var(--bg);
		justify-content: space-between;
		align-items: center;
		padding: 0 0.5em;
		border-radius: 5px;
		height: 2em;
	}

	.courses {
		display: flex;
		flex-direction: column;
		gap: 1em;
		position: relative;
	}

	button {
		background: inherit;
		color: var(--primary);
	}

	button:hover {
		color: var(--color-600);
	}

	button:active {
		color: var(--color-700);
	}

	button:disabled {
		color: var(--color-300);
	}
</style>

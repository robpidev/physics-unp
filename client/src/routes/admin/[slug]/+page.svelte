<script>
	import { enhance } from '$app/forms';
	import { page } from '$app/stores';
	export let data;
	let name = '';
	let error = '';
</script>

<section>
	<form
		method="post"
		action="?/add"
		use:enhance={() => {
			return async ({ result }) => {
				if (result.status !== 200) {
					// console.log(error);
					error = result.data.error;
				} else {
					error = '';
				}
			};
		}}
	>
		<input bind:value={name} type="text" name="school" />
		<button
			type="submit"
			disabled={name.length === 0 ||
				data?.data.some((s) => {
					return s.name === name.toUpperCase();
				})}>Agregar</button
		>
	</form>
	<br />
	<span class="error">{error}</span>
</section>

<section>
	<ul class="schools">
		{#each data.data as school, id}
			<li class="school">
				<a href="{$page.params.slug}/{school.id}">{school.name}</a>
			</li>
		{/each}
	</ul>
</section>

<style>
	.schools {
		list-style: none;
		display: flex;
		flex-direction: column;
		gap: 6px;
	}
	.school {
		border: 1px solid var(--border);
		width: 100%;
		padding: 0.5em 1em;
		border-radius: 8px;
	}
</style>

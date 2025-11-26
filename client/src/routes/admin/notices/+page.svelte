<script lang="ts">
	import type { Notice } from '$lib/types/notice';
	import Delete from '$lib/components/Delete.svelte';
	import Add from './Add.svelte';

	interface Data {
		notices: Notice[];
	}

	let { data }: { data: Data } = $props();
	function parseDate(date: Date) {
		let day = date.toLocaleString('es-PE').split('T')[0].split('-').reverse().join('/');
		let hour = date.toLocaleString('es-PE').split('T')[1].split('.')[0].slice(0, -3);

		return [day, hour];
	}
</script>

<section>
	<div class="notices">
		<h2>Avisos</h2>
		<Add />
		<hr />
		<ul>
			{#each data.notices as notice}
				<li class="notice">
					<span class="note">
						{notice.note}
					</span>
					<div class="datetime">
						<span>
							{parseDate(notice.datetime)[0]}
						</span>
						<span>
							{parseDate(notice.datetime)[1]}
						</span>
					</div>

					<Delete id={notice.id} />
				</li>
			{/each}
		</ul>
	</div>
</section>

<style>
	.notices {
		padding: 0;
	}

	ul {
		list-style: none;
		padding: 0;
		margin: 0;
	}
	li {
		display: flex;
		justify-content: space-between;
		align-items: center;
		/*border: solid 1px var(--border);*/
		background: var(--bg);
		margin: 5px;
		align-items: center;
		border-radius: 5px;
		/* border: solid 1px var(--border); */
		padding: 0em 1em;
		position: relative;
		margin-top: 1.5rem;
	}

	li:hover {
		background: rgba(0, 0, 0, 0.1);
	}

	.datetime {
		/* position: absolute; */
		align-items: flex-end;
		right: 1em;
		font-size: 0.7em;
		bottom: 1px;
		position: absolute;
		bottom: -1rem;
	}

	h2 {
		color: var(--primary);
	}

	section {
		padding: 0em 1em 1em 1em;
	}
</style>

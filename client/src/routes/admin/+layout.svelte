<script>
	import { breadcrum } from '$lib/stores';
	/**
	 * @typedef {Object} Props
	 * @property {import('svelte').Snippet} [children]
	 */

	/** @type {Props} */
	let { children } = $props();
</script>

<div class="page">
	<nav class="nav">
		<ul class="menu-item">
			<li>
				<a href="/admin">facultades</a>
			</li>
			<li>
				<a href="/admin/calendar">Horarios</a>
			</li>
		</ul>
	</nav>

	{#if $breadcrum != []}
		<nav class="breadcrumb">
			<ul class="items">
				{#each $breadcrum as path, i}
					<li class="item">
						<a
							href="/admin/{$breadcrum
								.slice(0, i + 1)
								.map((s) => s.url)
								.join('/')}"
							onclick={() => breadcrum.update((path) => path.slice(0, i + 1))}
						>
							{path.name}
						</a>
					</li>
				{/each}
			</ul>
		</nav>
	{/if}

	{@render children?.()}
</div>

<style>
	.page {
		max-width: 1000px;
		/*border: solid 1px;*/
		width: 100%;
		padding: 0 1em;
		display: flex;
		flex-direction: column;
		gap: 1em;
	}

	.menu-item {
		display: flex;
		list-style: none;
		padding: 0;
		margin: 0;
		gap: 1em;
	}

	.nav {
		padding: 6px;
		border-bottom: solid 1px var(--border);
	}

	.nav a {
		color: var(--primary);
		font-weight: 600;
	}

	.items {
		display: flex;
		list-style: none;
		padding: 0;
		margin: 0;
		gap: 5px;
	}

	.items li {
		color: white;
		background: var(--primary);
		position: relative;
		border: solid 1px;
	}

	.item a {
		padding: 0 0.5em;
		color: white;
	}

	.items li:nth-child(1) {
		border-radius: 10px 0 0 10px;
	}

	.items a::after {
		content: '>';
		right: -10px;
		bottom: -3px;
		color: var(--primary);
		width: 10px;
		border-radius: 0 10px 10px 0;
		position: absolute;
		background: var(--primary);
		border-right: solid 3px var(--bg);
		border-top: solid 3px var(--bg);
		border-bottom: solid 3px var(--bg);
		z-index: 1;
	}
</style>

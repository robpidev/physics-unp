<div class="page">
	<nav>
		<ul class="breadcrumb">
			<li>
				<a href="/admin">facultades</a>
			</li>
			<li>
				<a href="/admin/calendar">Horarios</a>
			</li>
		</ul>
	</nav>
	<slot></slot>
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

	.breadcrumb {
		display: flex;
		list-style: none;
		padding: 0;
		margin: 0;
		gap: 1em;
	}

	nav {
		margin: 10px;
		padding: 6px;
		border: solid 1px var(--border);
		border-radius: 8px;
	}
</style>

<script>
	import { enhance } from '$app/forms';
	let loading = false;
	export let form;
	$: student = form?.student;
</script>

<div>
	<form
		action="?/studentInfo"
		method="post"
		use:enhance={() => {
			loading = true;
			return async ({ update }) => {
				await update();
				loading = false;
			};
		}}
	>
		<label>
			Agregar a un alumno:
			<input disabled={loading} type="type" name="student_id" placeholder="Código del alumno" />
			{#if loading}
				<span class="loading">Cargando...</span>
			{/if}
		</label>
	</form>

	{#if form?.student}
		<form
			action="?/enroll"
			method="post"
			use:enhance={() => {
				loading = true;
				return async ({ update }) => {
					await update();
					loading = false;
				};
			}}
		>
			<label class="info">
				<span>{student.names} {student.last_name1} {student.last_name2}</span>
				<span>{student.toString().length < 10 ? '0' : ''}{student.code}</span>
				<input type="text" name="student_id" hidden value={student.code} />
				<button disabled={loading}>Matricular</button>
			</label>
		</form>
		<!-- content here -->
	{/if}

	{#if form?.error && !loading}
		<p class="error">{form?.error.split(':').pop()}</p>
	{/if}
</div>

<style>
	.loading {
		color: green;
	}

	form {
		width: 100%;
		display: flex;
	}

	.info {
		margin: 1em 0;
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.error {
		color: red;
	}
</style>

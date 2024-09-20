<script>
	import { enhance } from '$app/forms';
	export let course;
	export let total;
	let disabled = false;
</script>

<div class="course">
	<div>
		<span class="name">
			{course.name}
			{disabled}
		</span>
	</div>

	<div class="places">
		<span class="label">Alumnos: </span>
		<div class="value">
			<form
				method="POST"
				action="?/updatePlaces"
				use:enhance={() => {
					disabled = true;
					return async ({ update }) => {
						await update();
						disabled = false;
					};
				}}
			>
				<label
					>{total}/
					<input type="number" name="places" value={course.places} required min="0" max="256" />
				</label>
				<button type="submit" {disabled}>Actualizar</button>
			</form>
		</div>
	</div>
</div>

<style>
	.course {
		display: flex;
		justify-content: space-between;
	}

	.name {
		font-weight: 600;
		color: green;
		font-size: 14pt;
	}

	.value {
		font-size: 13pt;
		font-weight: 600;
		color: var(--primary);
		display: flex;
	}

	.places {
		display: flex;
		flex-direction: row;
		gap: 1em;
		align-items: center;
	}

	input {
		border: none;
		font-size: 13pt;
		width: 3em;
		padding: 0 5px;
		height: 2em;
		color: green;
		font-weight: 600;
	}

	input:focus {
		outline: none;
	}

	button:disabled {
		background: var(--color-300);
	}
</style>

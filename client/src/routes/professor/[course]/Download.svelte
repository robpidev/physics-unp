<script>
	import * as XLSX from 'xlsx';
	export let data = [];

	function toScv() {
		const students = data.map((ev) => {
			return { Código: ev.id, Nombres: ev.name };
		});

		const ws = XLSX.utils.json_to_sheet(students);
		const wb = XLSX.utils.book_new();
		XLSX.utils.book_append_sheet(wb, ws, 'Estudiantes');

		XLSX.writeFile(wb, 'Estudiantes.xlsx');
	}
</script>

<button on:click={toScv}> Descargar </button>

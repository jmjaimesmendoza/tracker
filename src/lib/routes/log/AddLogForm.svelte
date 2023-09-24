<script lang="ts">
	import { getContext } from "svelte";
	import { addPerson, personStore, setPersons } from "../../stores/personStore";
	import Select from "svelte-select";
	import { equipmentStore } from "../../stores/equipmentStore";
	import { invoke } from "@tauri-apps/api";
	const { close } = getContext("simple-modal");
	let selectedEquipment: { value: number; label: string } | null = null;
	let selectedPerson: { value: number; label: string } | null = null;
	let description = "";
	let km = 0;
	let selectedJob: { index: number, value: string; label: string } | null = null;
	let jobItems = [
		"Preventivo",
		"Reparacion leve",
		"Reparacion mayor",
		"Cambio de aceite y filtros"
	];
	$: personItems = $personStore.map((person) => ({
		label: person.name,
		value: person.id
	}));
	$: equipmentItems = $equipmentStore.map((equipment) => ({
		label: equipment.name,
		value: equipment.id
	}));

	async function onSubmit() {
		let equipmentId = parseInt(selectedEquipment?.value);
		let personId = parseInt(selectedPerson?.value);
		let job = selectedJob?.value.toString();
		await invoke("add_log", {
			equipmentId,
			personId,
			description,
			km,
			job
		});

		close();
	}
</script>

<form class="grid grid-cols-1 gap-4 mt-4" on:submit|preventDefault={onSubmit}>
	<div class="flex flex-col">
		<label for="equipo" class="text-sm text-gray-600">Equipo</label>
		<Select
			id="equipo"
			placeholder={"Equipo"}
			items={equipmentItems}
			bind:value={selectedEquipment}
		/>
	</div>
	<div class="flex flex-col">
		<label for="persona" class="text-sm text-gray-600">Responsable</label>
		<Select
			id="persona"
			placeholder={"Persona"}
			items={personItems}
			bind:value={selectedPerson}
		/>
	</div>
	<div class="flex flex-col">
		<label for="selectedJob" class="text-sm text-gray-600"
			>Tipo de Trabajo</label
		>
		<Select
			id="selectedJob"
			placeholder={"Tipo de Trabajo"}
			items={jobItems}
			bind:value={selectedJob}
		/>
	</div>
	<div class="flex flex-col">
		<label for="description" class="text-sm text-gray-600">Descripcion</label>
		<input
			id="description"
			class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-indigo-500"
			type="text"
			placeholder="Nombre del Equipo"
			bind:value={description}
		/>
	</div>
	<div class="flex flex-col">
		<label for="km" class="text-sm text-gray-600">Kilometraje</label>
		<input
			id="km"
			class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-indigo-500"
			type="number"
			placeholder="Kilometros del Equipo"
			bind:value={km}
		/>
	</div>
	<button
		type="submit"
		class="bg-indigo-500 hover:bg-indigo-600 text-white font-semibold py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-300"
	>
		Añadir Registro
	</button>
</form>

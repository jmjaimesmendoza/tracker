<script lang="ts">
	import { getContext } from "svelte";
	import { addPerson, personStore, setPersons } from "../../stores/personStore";
	import Select from "svelte-select";
	import { equipmentStore, setEquipments } from "../../stores/equipmentStore";
	import { invoke } from "@tauri-apps/api";
  import { setLogs } from "../../stores/logStore"
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
	let selectedOption = "D";
	let target: any;

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
		await invoke("add_revision", {
			equipmentId,
			tipo: selectedOption,
			target: target.toString()
		})

		await setLogs()
		await setEquipments()

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
		<input required
			id="description"
			class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500"
			type="text"
			placeholder="Descripcion"
			bind:value={description}
		/>
	</div>
	<div class="flex flex-col">
		<label for="km" class="text-sm text-gray-600">Kilometraje/Horas Actuales</label>
		<input required
			id="km"
			class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500"
			type="number"
			placeholder="Kilometros/Horas del Equipo"
			bind:value={km}
		/>
	</div>
	<div class="flex flex-col">
		<h2 class="text-center font-bold font">Proxima Revision</h2>
		<div class="flex justify-evenly gap-20 mt-8">
			<label>
				<input required type="radio" bind:group={selectedOption} value="D" />
				Fecha
			</label>

			<label>
				<input required type="radio" bind:group={selectedOption} value="K" />
				Kilometros / Horas
			</label>
		</div>
			<div class="flex flex-col mt-8">
			{#if selectedOption === "D"}
			<label for="target">Fecha revision:</label>
			<input required id="target" class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500 h-12" type="date" name="target"  bind:value={target}/>
			{:else if selectedOption === "K"}
			<label for="revision_km">Kilometros/Horas revision:</label>
			<input required id="target" class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500 h-12" type="number" name="target" bind:value={target} />
			{/if}
		</div>
	</div>
	<button
		type="submit"
		class="bg-green-400 hover:bg-green-500 text-white font-semibold py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-green-300"
	>
		Añadir Registro
	</button>
</form>

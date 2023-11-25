<script lang="ts">
  import { getContext, onMount } from "svelte";
  import { personStore } from "../../stores/personStore";
  import Select from "svelte-select";
  import { equipmentStore, setEquipments } from "../../stores/equipmentStore";
  import { invoke } from "@tauri-apps/api";
  import { logStore, parsedLogStore, setLogs } from "../../stores/logStore";
  import toast from "svelte-french-toast";
  import { get } from "svelte/store";
  import {
    addDays,
    format,
    getDay,
    getISODay,
    parse,
    parseISO,
  } from "date-fns";
  const { close } = getContext("simple-modal");
  import { DateInput } from "date-picker-svelte";
  export let id: number;
  let selectedEquipment: { value: string; label: string };
  let selectedPerson: { value: string; label: string };
  let description = "";
  let km = 0;
  let fecha: Date;
  let selectedJob: { index: number; value: string; label: string };
  let jobItems = [
    "Preventivo",
    "Reparacion leve",
    "Reparacion mayor",
    "Cambio de aceite y filtros",
  ];
  let selectedOption = "D";
  let target: any;

  $: personItems = $personStore.map((person) => ({
    label: person.name,
    value: person.id,
  }));

  $: equipmentItems = $equipmentStore.map((equipment) => ({
    label: equipment.name,
    value: equipment.id,
  }));

  onMount(async () => {
    if (!id) {
      toast.error("No se ha seleccionado un registro");
      close();
      return;
    }
    const log = get(logStore).find((log) => log.id === id);
    console.log(log);

    if (!log) {
      toast.error("No se ha seleccionado un registro");
      close();
      return;
    }
    const logEq = get(equipmentStore).find((eq) => eq.id === log.equipment_id);
    selectedEquipment = {
      value: logEq?.id.toString() || "",
      label: logEq?.name || "",
    };
    const logPerson = get(personStore).find(
      (person) => person.id === log.person_id
    );
    selectedPerson = {
      value: logPerson?.id.toString() || "",
      label: logPerson?.name || "",
    };
    description = log.description;
    km = log.km;
    fecha = new Date(log.created_at);
    fecha = new Date(fecha.getTime() + fecha.getTimezoneOffset() * 60000);

    selectedJob = {
      index: jobItems.findIndex((job) => job === log.job),
      value: log.job,
      label: log.job,
    };
  });

  async function onSubmit() {
    const toff = fecha.getTimezoneOffset() * 60000;
    await invoke("edit_log", {
      logId: id,
      equipmentId: parseInt(selectedEquipment?.value),
      personId: parseInt(selectedPerson?.value),
      description,
      km,
      job: selectedJob?.value,
      newCreatedAt: format(new Date(fecha.getTime() + toff), "yyyy-MM-dd"),
    });
    await setLogs();
    await setEquipments();

    close();
  }
</script>

<form class="grid grid-cols-1 gap-4 mt-4" on:submit|preventDefault={onSubmit}>
  <div class="flex flex-col">
    <label for="fecha" class="text-sm text-gray-600">Fecha</label>
    <DateInput closeOnSelection format="dd/MM/yyyy" bind:value={fecha} />
  </div>
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
      required
      id="description"
      class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500"
      type="text"
      placeholder="Descripcion"
      bind:value={description}
    />
  </div>
  <div class="flex flex-col">
    <label for="km" class="text-sm text-gray-600"
      >Kilometraje/Horas Actuales</label
    >
    <input
      required
      id="km"
      class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500"
      type="number"
      placeholder="Kilometros/Horas del Equipo"
      bind:value={km}
    />
  </div>
  <button
    type="submit"
    class="bg-green-400 hover:bg-green-500 text-white font-semibold py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-green-300"
  >
    Editar Registro
  </button>
</form>

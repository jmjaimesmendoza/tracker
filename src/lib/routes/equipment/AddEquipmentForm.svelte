<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { addEquipment, setEquipments } from "../../stores/equipmentStore";
  import { getContext } from "svelte";

  let name = "";
  let km = 0;
  let test = "";
  async function onSubmit() {
    await addEquipment(name, km)
    await setEquipments()
    close()
  }
  const { close } = getContext('simple-modal');
</script>

<form class="grid grid-cols-1 gap-4 mt-4" on:submit|preventDefault={onSubmit}>
  <div class="flex flex-col">
    <label for="name" class="text-sm text-gray-600">Nombre</label>
    <input id="name" class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500" type="text" placeholder="Nombre del Equipo" bind:value={name} />
  </div>
  <div class="flex flex-col">
    <label for="km" class="text-sm text-gray-600">Kilometros/Horas</label>
    <input id="km" class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500" type="number" placeholder="Kilometros/Horas del Equipo" bind:value={km} />
  </div>
  <button type="submit" class="bg-green-500 hover:bg-green-400 text-white font-semibold py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-green-300">
    Añadir Equipo
  </button>
</form>

<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { addEquipment } from "../../stores/equipmentStore";
  import { open } from "@tauri-apps/api/dialog";
  import { getContext, onMount } from "svelte";
  const { close } = getContext("simple-modal");
  import Select from "svelte-select";
  let name = "";
  let km = 0;
  let notes = "";
  let nserial = "";
  let modelIn: { value: number; label: string };

  let items: { value: number; label: string }[] = [];
  let models: { id: number; brand_id: number; name: string }[] = [];
  let brands: { id: number; name: string }[] = [];
  let file_path: string;

  $: items = models.map((model) => {
    return {
      label: `${brands.find((b) => b.id === model.brand_id)?.name} - ${
        model.name
      }`,
      value: model.id,
    };
  });

  onMount(async () => {
    models = JSON.parse(await invoke("get_models"));
    brands = JSON.parse(await invoke("get_brands"));
  });

  async function onSubmit() {
    await addEquipment(name, km, modelIn?.value, nserial, notes, file_path);
    close();
  }
</script>

<form class="grid grid-cols-1 gap-4 mt-4" on:submit|preventDefault={onSubmit}>
  <div class="flex flex-col">
    <label
      for="name"
      class="text-sm text-gray-600"
    >
      Nombre</label
    >
    <input
      required
      id="name"
      class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500"
      type="text"
      placeholder="Nombre del Equipo"
      bind:value={name}
    />
  </div>
  <div class="flex flex-col">
    <label for="model" class="text-sm text-gray-600">Modelo</label>
    <Select id="model" placeholder={"Modelo"} {items} bind:value={modelIn} />
  </div>
  <div class="flex flex-col">
    <label for="km" class="text-sm text-gray-600">Kilometros/Horas</label>
    <input
      required
      id="km"
      class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500"
      type="number"
      placeholder="Kilometros/Horas del Equipo"
      bind:value={km}
    />
  </div>
  <div class="flex flex-col">
    <label for="nserial" class="text-sm text-gray-600">Serial</label>
    <input
      required
      id="nserial"
      class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500"
      type="text"
      placeholder="Serial del Equipo"
      bind:value={nserial}
    />
  </div>
  <div class="flex flex-col">
    <label for="notes" class="text-sm text-gray-600">Notas</label>
    <input
      required
      id="notes"
      class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500"
      type="text"
      placeholder="Descripcion"
      bind:value={notes}
    />
  </div>
  <button
    on:click={() => {
      open({
        multiple: false,
        filters: [
          {
            name: "Image",
            extensions: ["png", "jpeg", "jpg"],
          },
        ],
      }).then((res) => {
        file_path = res;
      });
    }}
    type="button"
    class="bg-green-500 hover:bg-green-400 text-white font-semibold py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-green-300"
  >
    Subir Foto
  </button>
  <button
    type="submit"
    class="bg-green-500 hover:bg-green-400 text-white font-semibold py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-green-300"
  >
    Añadir Equipo
  </button>
</form>

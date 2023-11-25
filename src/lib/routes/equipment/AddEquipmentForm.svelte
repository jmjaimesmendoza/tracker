<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { addEquipment, equipmentStore } from "../../stores/equipmentStore";
  import { open } from "@tauri-apps/api/dialog";
  import { getContext, onMount } from "svelte";
  const { close } = getContext("simple-modal");
  import Select from "svelte-select";
  import { parsedModelStore } from "../../stores/modelStore";
  import { get } from "svelte/store";
  import { isArray } from "lodash";
  import Image from "../../components/Image.svelte";
  import { convertFileSrc } from "@tauri-apps/api/tauri";

  export let id: number = -1;
  let name: string = "";
  let km: number = 0;
  let notes: string = "";
  let nserial: string = "";
  let file_path: string = "";

  let modelIn: { value: number; label: string };

  let items: { value: number; label: string }[] = [];
  let models: { id: number; brand: string; name: string }[] = [];

  $: items = models.map((model) => {
    return {
      label: `${model.brand} - ${model.name}`,
      value: model.id,
    };
  });

  onMount(async () => {
    models = get(parsedModelStore);
    if (id === -1) return;
    const found = get(equipmentStore).find((e) => {
      const foundModel = models.find((m) => m.id === e.model_id);
      if (e.id === id) {
        name = e.name;
        km = e.km;
        notes = e.notes;
        nserial = e.nserial;
        file_path = e.file_path;
        modelIn = {
          value: foundModel?.id ?? -1,
          label: `${foundModel?.brand} - ${foundModel?.name}`,
        };
      }
    });
    console.log(found);
  });

  async function onSubmit() {
    if (id === -1) {
      await addEquipment(name, km, modelIn?.value, nserial, notes, file_path);
    } else {
      await invoke("put_equipment", {
        equipmentId: id,
        name: name,
        km: km,
        modelId: modelIn?.value,
        nserial: nserial,
        notes: notes,
        filePath: file_path,
      });
    }
    close();
  }
</script>

<form class="grid grid-cols-1 gap-4 mt-4" on:submit|preventDefault={onSubmit}>
  <div class="flex flex-col">
    <label for="name" class="text-sm text-gray-600"> Nombre</label>
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
  {#if file_path !== ""}
    <div class="flex flex-col">
      <label for="file_path" class="text-sm text-gray-600">Foto</label>
      <Image source={convertFileSrc(file_path)} altText="Foto" />
    </div>
  {:else}
    <div class="bg-gray-400 w-32 h-32 flex items-center justify-center">
      <span class="text-white">Foto Aqui</span>
    </div>
  {/if}
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
        if (Array.isArray(res)) {
          res = res[0];
        }
        file_path = res ?? "";
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
    {id === -1 ? "Añadir Equipo" : "Editar Equipo"}
  </button>
</form>

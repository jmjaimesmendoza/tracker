<script lang="ts">
  import { invoke } from "@tauri-apps/api"
  import { addEquipment, setEquipments } from "../../stores/equipmentStore";
  import { getContext, onMount } from "svelte";
  import Select from "svelte-select";
  let name = "";
  let brand: {value: number, label: string};
  let items: {value: number, label: string}[]=[];
  let brands: {id: number, name: string}[] = [];

  $: items = brands.map((brand)=> {
    return {
      label: brand.name,
      value: brand.id
    }
  })

  onMount (async () => {
    brands = JSON.parse(await invoke("get_brands"));    
  })

  async function onSubmit() {
    await invoke("add_model",{name:name, brandId:brand?.value});
    close()
  }
  const { close } = getContext('simple-modal');
</script>

<form class="grid grid-cols-1 gap-4 mt-4" on:submit|preventDefault={onSubmit}>
  <div class="flex flex-col">
    <label for="name" class="text-sm text-gray-600">Nombre</label>
    <input required id="name" class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500" type="text" placeholder="Nombre del Modelo" bind:value={name} />
  </div>
  <div class="flex flex-col">
    <label for="brand" class="text-sm text-gray-600">Marca</label>
    <Select
      id="brand"
			placeholder={"Marca"}
			items={items}
			bind:value={brand}
    />
  </div> 
  <button type="submit" class="bg-green-500 hover:bg-green-400 text-white font-semibold py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-green-300">
    Añadir Modelo
  </button>
</form>

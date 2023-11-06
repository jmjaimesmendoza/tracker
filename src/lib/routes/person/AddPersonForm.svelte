<script>
  import { getContext } from "svelte";
	import { addPerson, setPersons } from "../../stores/personStore";
  const { close } = getContext('simple-modal');
  let name = "";

  async function onSubmit() {
    await addPerson(name)
    await setPersons()
    close()
  }
</script>

<form class="grid grid-cols-1 gap-4 mt-4" on:submit|preventDefault={onSubmit}>
  <div class="flex flex-col">
    <label for="name" class="text-sm text-gray-600">Nombre</label>
    <input id="name" class="border border-gray-300 rounded-md px-3 py-2 mt-1 focus:outline-none focus:border-green-500" type="text" placeholder="Nombre del Responsable" bind:value={name} />
  </div>
  <button type="submit" class="bg-green-500 hover:bg-green-400 text-white font-semibold py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-green-300">
    Añadir Persona
  </button>
</form>

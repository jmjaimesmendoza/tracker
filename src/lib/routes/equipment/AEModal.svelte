<script>
  import { getContext } from "svelte";
  import AddEquipmentForm from "./AddEquipmentForm.svelte";
  const { open } = getContext("simple-modal");
  const showForm = () => open(AddEquipmentForm);
</script>

<button
  on:click={showForm}
  type="button"
  class="bg-green-500 hover:bg-green-400 text-white font-semibold py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-green-300"
> Añadir Equipo </button>

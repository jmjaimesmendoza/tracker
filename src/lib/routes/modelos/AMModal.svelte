<script lang="ts">
  import { getContext } from "svelte";
  import AddModelForm from "./AddModelForm.svelte";
  const { open, close } = getContext("simple-modal");
  const showForm = () => open(AddModelForm);
</script>

<button
  on:click={showForm}
  type="button"
  class="bg-green-500 hover:bg-green-400 text-white font-semibold py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-green-300"
>
  Añadir Modelo
</button>

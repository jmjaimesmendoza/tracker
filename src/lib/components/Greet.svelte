<script lang="ts">
	
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	import { Link } from "svelte-routing";
	import Table from "./Table.svelte";
	// let name = "";
	// let km = 0;
	// let test = "";
	// async function addEquipment() {
	// 	// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
	// 	test = await invoke("add_equipment", { name, km });
	// }
	// <form class="row" on:submit|preventDefault={addEquipment}>
	// 	<input id="name" placeholder="Enter a name..." bind:value={name} />
	// 	<input id="km" type="number" placeholder="Enter a km..." bind:value={km} />
	// 	<button type="submit">add eskeler</button>
	// </form>
  type Equipment = {
    id: number;
    name: string;
    km: number;
  };
  let equipments:Array<Equipment> = [];
	onMount(async () => {
		const res = await invoke("get_equipments");
		equipments = JSON.parse(res);
	});
</script>

<div>
	<Table data={equipments}/>
</div>
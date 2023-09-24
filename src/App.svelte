<script lang="ts">
  import "./styles.css";
	import Equipments from "./lib/routes/equipment/Equipments.svelte";
	import About from "./lib/components/About.svelte";
	import Persons from "./lib/routes/person/Persons.svelte";
	import { Router, Route } from "svelte-routing";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import { onDestroy, onMount } from "svelte";
  import { globalHistory } from "svelte-routing/src/history";
  import { currentPath } from "./lib/stores/stores";
	import Logs from "./lib/routes/log/Logs.svelte";

	let unsub: () => void;

	onMount(() => {
    unsub = globalHistory.listen(({ location, action }) => {
      $currentPath = location.pathname
    })
  })

	onDestroy(() => {
		unsub()
	})
</script>

<div class="flex">
	<Sidebar/>
	<div class="flex-1">
		<Router>
			<Route path="/" component={Equipments} />
			<Route path="/logs" component={Logs} />
			<Route path="/personas" component={Persons} />
		</Router>
	</div>
</div>
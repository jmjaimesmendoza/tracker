<script lang="ts">
  import "./styles.css";
	import Equipments from "./lib/components/Equipments.svelte";
	import About from "./lib/components/About.svelte";
	import Users from "./lib/components/Users.svelte";
	import { Router, Route } from "svelte-routing";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import { onDestroy, onMount } from "svelte";
  import { globalHistory } from "svelte-routing/src/history";
  import { currentPath } from "./lib/stores/stores";

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
			<Route path="/about" component={About} />
			<Route path="/users" component={Users} />
		</Router>
	</div>
</div>
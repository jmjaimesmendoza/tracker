<script lang="ts">
  import "./styles.css";
	import Greet from "./lib/components/Greet.svelte";
	import About from "./lib/components/About.svelte";
	import Users from "./lib/components/Users.svelte";
	import { Router, Route } from "svelte-routing";
</script>

<Router>
	<Route path="/" component={Greet} />
	<Route path="/about" component={About} />
	<Route path="/users" component={Users} />
</Router>
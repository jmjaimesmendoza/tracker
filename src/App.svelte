<script lang="ts">
  import "./styles.css";
  import Equipments from "./lib/routes/equipment/Equipments.svelte";
  import About from "./lib/components/About.svelte";
  import Persons from "./lib/routes/person/Persons.svelte";
  import Models from "./lib/routes/modelos/Models.svelte";
  import { Router, Route } from "svelte-routing";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import { onDestroy, onMount } from "svelte";
  import { globalHistory } from "svelte-routing/src/history";
  import { currentPath } from "./lib/stores/stores";
  import Logs from "./lib/routes/log/Logs.svelte";
  import { Toaster } from "svelte-french-toast";
  import { setEquipments } from "./lib/stores/equipmentStore";
  import { setPersons } from "./lib/stores/personStore";
  import { setLogs } from "./lib/stores/logStore";
  import { listen } from "@tauri-apps/api/event";

  let unsub: () => void;

  onMount(async () => {
    unsub = globalHistory.listen(({ location, action }) => {
      $currentPath = location.pathname;
    });
    await setPersons();
    await setLogs();
    await setEquipments();
  });

  onDestroy(() => {
    unsub();
  });
</script>

<div class="flex">
  <Sidebar />
  <Toaster />
  <div class="flex-1 z-20 max-h-[99vh] overflow-y-scroll">
    <Router>
      <Route path="/" component={Logs} />
      <Route path="/equipos" component={Equipments} />
      <Route path="/personas" component={Persons} />
      <Route path="/modelos" component={Models} />
    </Router>
  </div>
</div>

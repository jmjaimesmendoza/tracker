<script lang="ts">
  import { onMount } from "svelte";
  import { createTable, Subscribe, Render } from "svelte-headless-table";
  import { addSortBy } from "svelte-headless-table/plugins";
  import Modal from 'svelte-simple-modal';
  import AEModal from "./AEModal.svelte";
  import { equipmentStore, setEquipments } from "../../stores/equipmentStore";
	import { setLogs, logStore } from "../../stores/logStore";
	import { invoke } from "@tauri-apps/api";
	import { get, writable } from "svelte/store";
  import _ from "lodash";
	import { addDays, differenceInDays, format } from "date-fns";
  let revisions: {id: number, equipment_id: number, tipo: string, target: string}[] = [];
  const parsedEquipment = writable([]);   
  const table = createTable(parsedEquipment, {
    sort: addSortBy({ disableMultisort: true }),
  });
  const columns = table.createColumns([
    table.column({
      header: "Nombre",
      accessor: "name",
      plugins: {
        sort: {
          invert: true,
        },
      },
    }),
    table.column({
      header: "Km Actuales",
      accessor: "km",
    }),
    table.column({
      header: "Ultima revision",
      accessor: "lastRevision",
    }),
    table.column({
      header: "Proxima revision",
      accessor: "expectedRevisionDate",
    }),
  ]);
  const { headerRows, rows, tableAttrs, tableBodyAttrs } =
    table.createViewModel(columns);

  onMount(async () => {
    await setLogs()
    await setEquipments()
    const res: string = await invoke("get_revisions");
    
    revisions = await JSON.parse(res);
    parseRevisionDates()
  });
  
  const parseRevisionDates = () => {
    let test = $equipmentStore.map((e) => {
      const revision = revisions.findLast((r) => r.equipment_id === e.id);
      const logs = $logStore.filter((l) => l.equipment_id === e.id);
      if (!revision || !logs.length) return {...e, lastRevision: "Por determinar", expectedRevisionDate: "Por determinar"};
      const lastLog = logs[logs.length - 1];
      const revDiff = parseInt(revision?.target) - lastLog.km;
      if (revision?.tipo === "D") {
        const revisionDate = new Date(revision.target.replace(/-/g, '/'))
        const timeUntilRevision = differenceInDays(new Date(),new Date(revision.target));
        console.log(timeUntilRevision);
        return {...e, lastRevision: format(new Date(lastLog.created_at), "dd/MM/yyy"), expectedRevisionDate: format(revisionDate, "dd/MM/yyyy")}
      }
      const avgs = logs.map((l,i) => {
        if (i === logs.length - 1) return;
        const dateDiff = differenceInDays(new Date(logs[i+1]?.created_at), new Date(l.created_at));
        if (!dateDiff) return;
        const kmDiff = logs[i+1]?.km - l.km;
        const kmPerInterval = kmDiff / dateDiff;
        return kmPerInterval;
      })
      
      if (avgs.includes(undefined)) {
        return {...e, lastRevision: format(new Date(lastLog.created_at.replace(/-/g, '/')), "dd/MM/yyy"),expectedRevisionDate: `${revision.target} Km/Horas`}
      };
      avgs.pop()
      const avgkm = _.mean(avgs);
      const estimatedDays = revDiff/avgkm;
      const estimatedRevisionDate = addDays(new Date(lastLog.created_at.replace(/-/g, '/')), estimatedDays);
      
      return {...e, lastRevision: format(new Date(lastLog.created_at.replace(/-/g, '/')), "dd/MM/yyy"),expectedRevisionDate: `${format(estimatedRevisionDate, "dd/MM/yyyy")} (${revision.target} Km/Horas)`}
    })
    parsedEquipment.set(test);
  };
</script>

<div>
  <Modal>
    <AEModal />
  </Modal>
  <table {...$tableAttrs} class="min-w-full divide-y divide-gray-200">
    <thead>
      {#each $headerRows as headerRow (headerRow.id)}
        <Subscribe rowAttrs={headerRow.attrs()} let:rowAttrs>
          <tr {...rowAttrs}>
            {#each headerRow.cells as cell (cell.id)}
              <Subscribe
                attrs={cell.attrs()}
                let:attrs
                props={cell.props()}
                let:props
              >
                <th
                  {...attrs}
                  on:click={props.sort.toggle}
                  class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
                >
                  <Render of={cell.render()} />
                  {#if props.sort.order === "asc"}
                    ⬇️
                  {:else if props.sort.order === "desc"}
                    ⬆️
                  {/if}
                </th>
              </Subscribe>
            {/each}
          </tr>
        </Subscribe>
      {/each}
    </thead>
    <tbody {...$tableBodyAttrs}>
      {#each $rows as row (row.id)}
        <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
          <tr
            {...rowAttrs}
            class={row.id % 2 === 0 ? "bg-gray-100" : "bg-white"}
          >
            {#each row.cells as cell (cell.id)}
              <Subscribe attrs={cell.attrs()} let:attrs>
                <td {...attrs} class="px-6 py-4 whitespace-nowrap">
                  <Render of={cell.render()} />
                </td>
              </Subscribe>
            {/each}
          </tr>
        </Subscribe>
      {/each}
    </tbody>
  </table>
</div>

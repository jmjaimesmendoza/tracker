<script lang="ts">
  import { onMount } from "svelte";
  import { createTable, Subscribe, Render } from "svelte-headless-table";
  import { addSortBy } from "svelte-headless-table/plugins";
  import Modal from 'svelte-simple-modal';
  import AEModal from "./AMModal.svelte";
  import { equipmentStore } from "../../stores/equipmentStore";
	import { logStore } from "../../stores/logStore";
	import { get, writable } from "svelte/store";
  import _ from "lodash";
	import { addDays, differenceInDays, format } from "date-fns";
  let revisions: {id: number, equipment_id: number, tipo: string, target: string}[] = [];
  const parsedEquipment = writable([]);
  const table = createTable(equipmentStore, {
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

<script lang="ts">
  import { onMount } from "svelte";
  import { createTable, Subscribe, Render, createRender } from "svelte-headless-table";
  import { addSortBy, addColumnFilters } from "svelte-headless-table/plugins";
  import type { ColumnFilterFn } from 'svelte-headless-table/plugins';
  import Modal from 'svelte-simple-modal';
  import AddLogModal from "./AddLogModal.svelte";
  import { logStore, setLogs } from "../../stores/logStore";
	import { equipmentStore, setEquipments } from "../../stores/equipmentStore";
	import { personStore, setPersons } from "../../stores/personStore";
	import { writable } from "svelte/store";
  import { format } from "date-fns";
  import SelectFilter from './SelectFilter.svelte';


  $: personList = $personStore;

  $: equipmentList = $equipmentStore;
  
  $: logsList = $logStore;

  const matchFilter: ColumnFilterFn = ({ filterValue, value }) => {
    if (filterValue === undefined) return true;
    return filterValue === value;
  };

  const eskeler = writable([]);   

  const table = createTable(eskeler, {
    sort: addSortBy({ disableMultisort: true }),
    filter: addColumnFilters(),
  });
  const columns = table.createColumns([
    table.column({
      header: "Fecha",
      accessor: "date",
    }),
    table.column({
      header: "Equipo",
      accessor: "name",
      plugins: {
        sort: {
          invert: true,
        },
      },
    }),
    table.column({
      header: "Responsable",
      accessor: "person",
      plugins: {
        sort: {
          invert: true,
        },
      },
    }),
    table.column({
      header: "Trabajo",
      accessor: "job",
      plugins: {
        sort: {
          invert: true,
        },
        filter: {
          fn: matchFilter,
          render: ({ filterValue, preFilteredValues }) =>
            createRender(SelectFilter, { filterValue, preFilteredValues }),
        },
      },
    }),
    table.column({
      header: "Kilometros",
      accessor: "km",
      plugins: {
        sort: {
          invert: true,
        },
      },
    }),
    table.column({
      header: "Descripcion",
      accessor: "description",
    }),
  ]);
  const { headerRows, rows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);
  const { filterValues } = pluginStates.filter;

  onMount(async () => {
    await setLogs()
    await setEquipments()
    await setPersons() 
    let raaah = personList && equipmentList && logsList?.map((log) => {
      const person = personList.find((person) => person.id === log.person_id);
      const equipment = equipmentList.find((equipment) => equipment.id === log.equipment_id);
      return { 
        name: equipment?.name,
        person: person?.name,
        km: log.km,
        job: log.job,
        description: log.description,
        date: format(new Date(log.created_at), "dd/MM/yyyy"),
      }
    });
    console.log(raaah);
    
    eskeler.set(raaah)
  });
</script>
<div>
  <Modal>
    <AddLogModal />
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
                  {#if props.filter?.render}
                    <div>
                      <Render of={props.filter.render} />
                    </div>
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

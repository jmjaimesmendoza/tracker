<script lang="ts">
  import { onMount } from "svelte";
  import { createTable, Subscribe, Render, createRender } from "svelte-headless-table";
  import { addSortBy, addColumnFilters } from "svelte-headless-table/plugins";
  import type { ColumnFilterFn } from 'svelte-headless-table/plugins';
  import Modal from 'svelte-simple-modal';
  import AddLogModal from "./AddLogModal.svelte";
  import { logStore } from "../../stores/logStore";
	import { equipmentStore } from "../../stores/equipmentStore";
	import { personStore, setPersons } from "../../stores/personStore";
	import { writable } from "svelte/store";
  import { format, parse } from "date-fns";
  import SelectFilter from '../../components/SelectFilter.svelte';
  import {bind} from "svelte-simple-modal"
  import EditLogForm from "./EditLogForm.svelte"

  $: personList = $personStore;

  $: equipmentList = $equipmentStore;
  
  $: logsList = $logStore;
  
  const modal = writable([])

  const matchFilter: ColumnFilterFn = ({ filterValue, value }) => {
    if (filterValue === undefined) return true;
    return filterValue === value;
  };

  const showPopupWithProps = (logId, newDate) => {
		modal.set(bind(EditLogForm, { logId, newDate: format(parse(newDate, "dd/MM/yyyy", new Date()), "yyyy-MM-dd") }));
	};

  const parsedLogs = writable([]);
  const table = createTable(parsedLogs, {
    sort: addSortBy({ disableMultisort: true }),
    filter: addColumnFilters(),
  });
  const columns = table.createColumns([
    table.column({
      header: "Id",
      accessor: "id",
    }),
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
        filter: {
          fn: matchFilter,
          render: ({ filterValue, preFilteredValues }) =>
            createRender(SelectFilter, { filterValue, preFilteredValues }),
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
        filter: {
          fn: matchFilter,
          render: ({ filterValue, preFilteredValues }) =>
            createRender(SelectFilter, { filterValue, preFilteredValues }),
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
      header: "Kilometros/Horas",
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
    let raaah = personList && equipmentList && logsList?.map((log) => {
      const person = personList.find((person) => person.id === log.person_id);
      const equipment = equipmentList.find((equipment) => equipment.id === log.equipment_id);
      return { 
        id: log.id,
        name: equipment?.name,
        person: person?.name,
        km: log.km,
        job: log.job,
        description: log.description,
        date: format(new Date(log.created_at.replace(/-/g, '/')), "dd/MM/yyyy"),
      }
    });
    parsedLogs.set(raaah)
  });
</script>
<div>
  <Modal show={$modal}>
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
              {#if cell.column.accessorKey === "date"}
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click={()=>showPopupWithProps(row.original.id,cell.value)}>EDIT</button>
              {/if}
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

<script lang="ts">
  import { onMount } from "svelte";
  import { createTable, Subscribe, Render, createRender } from "svelte-headless-table";
  import { addSortBy, addColumnFilters } from "svelte-headless-table/plugins";
  import type { ColumnFilterFn } from 'svelte-headless-table/plugins';
  import Modal from 'svelte-simple-modal';
  import AMModal from "./AMModal.svelte";
  import ABModal from "./ABModal.svelte";
  import { equipmentStore } from "../../stores/equipmentStore";
	import { get, writable } from "svelte/store";
  import _ from "lodash";
  import { invoke } from "@tauri-apps/api";
  import SelectFilter from "../../components/SelectFilter.svelte";
  const modelStore = writable([]);
  
  const table = createTable(modelStore, {
    sort: addSortBy({ disableMultisort: true }),
    filter: addColumnFilters()
  });

  const matchFilter: ColumnFilterFn = ({ filterValue, value }) => {
    if (filterValue === undefined) return true;
    return filterValue === value;
  };

  const parseModels = (models, brands) => {
    return models.map((model) => {
      const brand = brands.find((brand) => brand.id === model.brand_id);
      return {
        id: model.id,
        name: model.name,
        brand: brand.name,
      };
    });
  };
  onMount(async () => {
    const res = await invoke("get_models");
    const brandRequest = await invoke("get_brands");
    const models = JSON.parse(res);
    const brands = JSON.parse(brandRequest);
    const parsedModels = parseModels(models, brands); 
    
    modelStore.set(parsedModels);
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
      header: "Marca",
      accessor: "brand",
      plugins: {
        sort: {
          invert: true,
        },
        filter: {
          fn: matchFilter,
          render: ({ filterValue, preFilteredValues }) =>
            createRender(SelectFilter, { filterValue, preFilteredValues }),
        },
      }
    })
  ]);
  const { headerRows, rows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);
                                                                                               
</script>

<div>
  <Modal>
    <AMModal />
  </Modal>
  <Modal>
    <ABModal />
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

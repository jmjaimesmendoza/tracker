<script lang="ts">
  import "iconify-icon";
  import { onMount } from "svelte";
  import {
    createTable,
    Subscribe,
    Render,
    createRender,
  } from "svelte-headless-table";
  import { addSortBy, addColumnFilters } from "svelte-headless-table/plugins";
  import type { ColumnFilterFn } from "svelte-headless-table/plugins";
  import Modal, { bind } from "svelte-simple-modal";
  import AMModal from "./AMModal.svelte";
  import ABModal from "./ABModal.svelte";
  import SelectFilter from "../../components/SelectFilter.svelte";
  import { getModels, parsedModelStore } from "../../stores/modelStore";
  import type { ParsedModel } from "../../types/entities";
  import { writable } from "svelte/store";
  import AddModelForm from "./AddModelForm.svelte";

  const modelModal = writable([]);

  const table = createTable(parsedModelStore, {
    sort: addSortBy({ disableMultisort: true }),
    filter: addColumnFilters(),
  });

  const matchFilter: ColumnFilterFn = ({ filterValue, value }) => {
    if (filterValue === undefined) return true;
    return filterValue === value;
  };

  onMount(async () => {
    await getModels();
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
      },
    }),
  ]);
  const { headerRows, rows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);

  const openEditModal = (model: ParsedModel) => {
    modelModal.set(
      bind(AddModelForm, {
        id: model.id,
        name: model.name,
        brandIn: model.brand,
      })
    );
  };
</script>

<div>
  <Modal show={$modelModal}>
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
            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
              <button
                class="text-green-600 hover:text-green-900 bg-green-100 px-2 py-1 rounded-md"
                on:click={() => openEditModal(row.original)}
              >
                <iconify-icon icon="bxs:edit"></iconify-icon>
              </button>
            </td>
          </tr>
        </Subscribe>
      {/each}
    </tbody>
  </table>
</div>

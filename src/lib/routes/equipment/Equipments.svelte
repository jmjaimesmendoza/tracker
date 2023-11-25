<script lang="ts">
  import {
    createTable,
    Subscribe,
    Render,
    createRender,
  } from "svelte-headless-table";
  import { addSortBy } from "svelte-headless-table/plugins";
  import Modal, { bind } from "svelte-simple-modal";
  import AEModal from "./AEModal.svelte";
  import { equipmentStore, setEquipments } from "../../stores/equipmentStore";
  import _ from "lodash";

  const equipentModal = writable([]);

  onMount(async () => {
    await setEquipments();
  });
  const table = createTable(equipmentStore, {
    sort: addSortBy({ disableMultisort: true }),
  });
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import Image from "../../components/Image.svelte";
  import { onMount } from "svelte";
  import AddEquipmentForm from "./AddEquipmentForm.svelte";
  import { writable } from "svelte/store";
  const columns = table.createColumns([
    table.column({
      header: "Imagen",
      accessor: "file_path",
    }),
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

  const returnImage = (source: string) =>
    createRender(Image, {
      source: convertFileSrc(source),
    });

  const openEditModal = (eq: any) => {
    equipentModal.set(
      bind(AddEquipmentForm, {
        id: eq.id,
      })
    );
  };
</script>

<div>
  <Modal show={$equipentModal}>
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
                  {#if cell.id === "file_path"}
                    <Render of={returnImage(cell.value)} />
                  {:else}
                    <Render of={cell.render()} />
                  {/if}
                </td>
              </Subscribe>
            {/each}
            <td
              class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium"
            >
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

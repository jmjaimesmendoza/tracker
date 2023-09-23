<script lang="ts">
  export let data: {id: number, name: string, km: number}[] = [];
  let sortByColumn: string | null = null;
  let sortOrder: number = 1;

  function formatCell(value: any, column: string): string {
    if (column === 'km') {
      return parseInt(value).toLocaleString();
    } else {
      return value.toString();
    }
  }

  function sortData(column: string): void {
    if (column === sortByColumn) {
      sortOrder = -sortOrder;
    } else {
      sortByColumn = column;
      sortOrder = 1;
    }

    data = [...data].sort((a, b) => {
      const aValue = a[column];
      const bValue = b[column];

      if (column === 'km') {
        return (parseInt(aValue) - parseInt(bValue)) * sortOrder;
      } else {
        return aValue.toString().localeCompare(bValue.toString()) * sortOrder;
      }
    });
  }
</script>

<table class="min-w-full divide-y divide-gray-200">
  <thead>
    <tr>
      <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer" on:click={() => sortData('name')}>
        Name
        {#if sortByColumn === 'name'}
          {sortOrder === 1 ? "↑" : "↓"}
        {/if}
      </th>
      <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer" on:click={() => sortData('km')}>
        KM
        {#if sortByColumn === 'km'}
          {sortOrder === 1 ? "↑" : "↓"}
        {/if}
      </th>
    </tr>
  </thead>
  <tbody>
    {#each data as row (row.id)}
      <tr class="{row.id % 2 === 0 ? 'bg-gray-100' : 'bg-white'}">
        <td class="px-6 py-4 whitespace-nowrap">{formatCell(row.name, 'name')}</td>
        <td class="px-6 py-4 whitespace-nowrap">{formatCell(row.km, 'km')}</td>
      </tr>
    {/each}
  </tbody>
</table>

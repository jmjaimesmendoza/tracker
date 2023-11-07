<script lang="ts" context="module">
  export const getDistinct = (items: unknown[]): unknown[] => {
    return Array.from(new Set(items));
  };
</script>

<script lang="ts">
  import type { Readable, Writable } from 'svelte/store';

  export let filterValue: Writable<string | undefined>;
  export let preFilteredValues: Readable<unknown[]>;
  $: uniqueValues = getDistinct($preFilteredValues);
</script>

<select class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-green-500 focus:border-green-500 block p-2.5" bind:value={$filterValue} on:click|stopPropagation>
  <option value={undefined}>All</option>
  {#each uniqueValues as value}
    <option {value}>{value}</option>
  {/each}
</select>
<script>
  import { currentPath } from '../stores/stores';
	import { link } from 'svelte-routing';
</script>

<div class="bg-indigo-700 text-white w-64 flex-shrink-0 h-screen top-0 left-0 overflow-y-auto">
  <div class="p-4">
    <h1 class="text-2xl font-semibold">TrackTor</h1>
  </div>
  <nav class="mt-4">
    <ul>
      <li>
        <a href="/" class="block py-2 px-4 hover:bg-indigo-600 transition duration-200 {$currentPath === '/' ? 'bg-indigo-600' : ''}" use:link>Dashboard</a>
      </li>
      <li>
        <a href="/personas" class="block py-2 px-4 hover:bg-indigo-600 transition duration-200 {$currentPath === '/personas' ? 'bg-indigo-600' : ''}" use:link>Personas</a>
      </li>
      <li>
        <a href="/logs" class="block py-2 px-4 hover:bg-indigo-600 transition duration-200 {$currentPath === '/logs' ? 'bg-indigo-600' : ''}" use:link>logs</a>
      </li>
    </ul>
  </nav>
</div>

<script>
  import { currentPath } from '../stores/stores';
	import { link } from 'svelte-routing';
</script>

<div class="bg-green-700 text-white w-64 flex-shrink-0 h-screen top-0 left-0 overflow-y-auto">
  <div class="p-4">
    <h1 class="text-2xl font-semibold">TrackTor</h1>
  </div>
  <nav class="mt-4">
    <ul>
      <li>
        <a href="/" class="block py-2 px-4 hover:bg-green-400 transition duration-200 {$currentPath === '/' ? 'bg-green-400' : ''}" use:link>Registros</a>
      </li>
      <li>
        <a href="/equipos" class="block py-2 px-4 hover:bg-green-400 transition duration-200 {$currentPath === '/equipos' ? 'bg-green-400' : ''}" use:link>Equipos</a>
      </li>
      <li>
        <a href="/personas" class="block py-2 px-4 hover:bg-green-400 transition duration-200 {$currentPath === '/personas' ? 'bg-green-400' : ''}" use:link>Personas</a>
      </li>
      <li>
        <a href="/modelos" class="block py-2 px-4 hover:bg-green-400 transition duration-200 {$currentPath === '/modelos' ? 'bg-green-400' : ''}" use:link>Marcas y Modelos</a>
      </li>
    </ul>
  </nav>
</div>

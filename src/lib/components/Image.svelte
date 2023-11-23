<script lang="ts">
  export let source = "../../../public/404.png";
  export let altText = "alt text";
  export let height = "150px";
  export let width = "100%";
</script>

<div class="w-[150px]">
  <img src={source} alt={altText} {height} {width} />
</div>

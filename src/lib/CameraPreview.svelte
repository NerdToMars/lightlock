<script lang="ts">
  export let label: string = 'Camera Feed';
  export let imageSrc: string = '';
</script>

<div class="card w-full bg-base-100 shadow-xl">
  <figure>
    {#if imageSrc}
      <img src={imageSrc} alt={label} />
    {:else}
      <!-- fallback placeholder -->
      <div class="bg-gray-200 w-full h-48 flex items-center justify-center">
        <span class="text-gray-500">{label}</span>
      </div>
    {/if}
  </figure>
  <div class="card-body">
    <h2 class="card-title">{label}</h2>
    <!-- Additional info or buttons if needed -->
  </div>
</div>

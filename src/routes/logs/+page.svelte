<!-- src/routes/logs/+page.svelte -->
<script lang="ts">
    // Here you might fetch or load logs from an API or store
    let logs: string[] = [
      'Log entry 1',
      'Log entry 2',
      'Log entry 3'
    ];
  </script>
  
  <h1>Logs</h1>
  
  <ul>
    {#each logs as log}
      <li>{log}</li>
    {/each}
  </ul>
  
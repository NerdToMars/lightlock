<!-- src/routes/+page.svelte -->
<script lang="ts">
  import CameraPreview from '$lib/CameraPreview.svelte';
  import ProcessControl from '$lib/ProcessControl.svelte';
</script>

<h1>Welcome to the Home Page</h1>

<div class="camera-container">
  <CameraPreview />
  <CameraPreview />
</div>

<!-- The process control UI (start/stop/confirm/progress) -->
<ProcessControl />

<style>
.camera-container {
  display: flex;
  gap: 2rem;
  margin-bottom: 2rem;
}
</style>

<!-- src/routes/+page.svelte -->
<script lang="ts">
  import CameraPreview from '$lib/CameraPreview.svelte';
  import type { CameraStatus } from '$lib/CameraPreview.svelte';
  import ProcessControl from '$lib/ProcessControl.svelte';

  let cameraStatus: CameraStatus = {
    distance: 10,
    altitude: 10,
    battery: 0,
    isRecording: true,
  };
</script>

<div class="camera-container">
  <CameraPreview status={cameraStatus} />
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

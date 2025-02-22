<script lang="ts">
    import { processState, progressValue } from '$lib/stores/processStore';
    import { get } from 'svelte/store';
  
    // Local boolean to show/hide the confirm modal
    let showConfirm = false;
  
    function startProcess() {
      processState.set('running');
      progressValue.set(0);
      console.log('Process started');
    }
  
    // Called when user clicks "Stop" button
    function requestStop() {
      // Show the confirmation modal
      showConfirm = true;
    }
  
    // Called when user confirms "Yes" in the modal
    function confirmStop() {
      // Actual logic to stop the process
      processState.set('stopped');
      console.log('Process stopped by user');
      showConfirm = false;
    }
  
    // Called when user cancels "No" in the modal
    function cancelStop() {
      showConfirm = false;
    }
  
    // Example for re-starting or resetting
    function resetProcess() {
      processState.set('idle');
      progressValue.set(0);
      console.log('Process reset to idle');
    }
  </script>
  
  <!-- We watch the store values in the template -->
  {#if $processState === 'idle' || $processState === 'stopped'}
    <button class="btn btn-primary" on:click={startProcess}>
      Start
    </button>
  
  {:else if $processState === 'running'}
    <div class="flex items-center gap-4">
      <progress
        class="progress progress-primary w-56"
        value={$progressValue}
        max="100"
      />
      <button class="btn btn-error" on:click={requestStop}>
        Stop
      </button>
    </div>
  
  {:else if $processState === 'success'}
    <div class="alert alert-success shadow-lg mb-4">
      <span>Process completed successfully!</span>
    </div>
    <button class="btn btn-success" on:click={resetProcess}>
      Reset
    </button>
  {/if}
  
  <!-- DaisyUI modal toggled by showConfirm -->
  <div class="modal" class:modal-open={showConfirm}>
    <div class="modal-box">
      <h3 class="font-bold text-lg">Confirm Stop</h3>
      <p class="py-4">
        Are you sure you want to stop the process?
      </p>
      <div class="modal-action">
        <!-- "Yes" button -->
        <button class="btn btn-error" on:click={confirmStop}>
          Yes, Stop
        </button>
        <!-- "No" button -->
        <button class="btn" on:click={cancelStop}>
          Cancel
        </button>
      </div>
    </div>
  </div>
  
<!-- src/lib/ProcessControl.svelte -->
<script lang="ts">
    import { processState, progressValue, type ProcessState } from '$lib/stores/processStore';
    import { get } from 'svelte/store';
    import ConfirmModal from '$lib/ConfirmModal.svelte';
    import { onMount } from 'svelte';
  
    let showConfirm = false;
    type Timer = ReturnType<typeof setInterval>;
    let intervalId: Timer | null = null;
  
    // Type the progressValue get result
    const current = get(progressValue) as number;
  
    function startProcess() {
      processState.set('running');
      progressValue.set(0);
  
      // Mock progress increment
      intervalId = setInterval(() => {
        const current = get(progressValue);
        if (current < 100) {
          progressValue.set(current + 5);
        } else {
          // Reached 100% => success
          clearInterval(intervalId!);
          intervalId = null;
          processState.set('success');
        }
      }, 500);
    }
  
    function requestStop() {
      // Show the confirm modal
      showConfirm = true;
    }
  
    function confirmStop() {
      showConfirm = false;
      stopProcess();
    }
  
    function stopProcess() {
      // If user confirms
      if (intervalId) {
        clearInterval(intervalId);
        intervalId = null;
      }
      processState.set('stopped');
    }
  
    function resetProcess() {
      processState.set('idle');
      progressValue.set(0);
    }
  
    // Clean up intervals if this component is destroyed
    onMount(() => {
      return () => {
        if (intervalId) {
          clearInterval(intervalId);
        }
      };
    });
  </script>
  
  <!-- Reactive declarations for convenience -->
  {$processState}
  {$progressValue}
  
  <!-- Conditionally render UI based on process state -->
  {#if $processState === 'idle' || $processState === 'stopped'}
    <button on:click={startProcess} class="start-button">Start</button>
  {:else if $processState === 'running'}
    <div class="running-state">
      <progress max="100" value={$progressValue}></progress>
      <button on:click={requestStop} class="stop-button">Stop</button>
    </div>
  {:else if $processState === 'success'}
    <div class="success-state">
      <p>Process completed successfully!</p>
      <button on:click={resetProcess}>Reset</button>
    </div>
  {/if}
  
  <!-- Confirm modal for stopping process -->
  <ConfirmModal
    bind:open={showConfirm}
    message="Are you sure you want to stop the process?"
    on:confirm={confirmStop}
    on:cancel={() => showConfirm = false}
  />
  
  <style>
  .start-button {
    background-color: #444;
    color: white;
    padding: 0.75rem 1.5rem;
    border: none;
    cursor: pointer;
    border-radius: 4px;
  }
  
  .running-state {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  .stop-button {
    background-color: #c00;
    color: white;
    padding: 0.5rem 1rem;
    border: none;
    cursor: pointer;
    border-radius: 4px;
  }
  
  .success-state p {
    margin-bottom: 0.5rem;
  }
  
  .success-state button {
    background-color: #2ecc71;
    border: none;
    color: white;
    padding: 0.5rem 1rem;
    cursor: pointer;
    border-radius: 4px;
  }
  </style>
  
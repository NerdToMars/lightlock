<!-- src/lib/ConfirmModal.svelte -->
<script lang="ts">
    import { createEventDispatcher } from 'svelte';
  
    export let message: string = "Are you sure?";
    export let open: boolean = false;
  
    const dispatch = createEventDispatcher();
  
    function confirm() {
      dispatch('confirm');
    }
  
    function cancel() {
      dispatch('cancel');
    }
  </script>
  
  {#if open}
    <div class="modal-backdrop">
      <div class="modal">
        <p>{message}</p>
        <div class="buttons">
          <button on:click={confirm}>Yes</button>
          <button on:click={cancel}>No</button>
        </div>
      </div>
    </div>
  {/if}
  
  <style>
  .modal-backdrop {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0,0,0,0.5);
    display: flex;
    justify-content: center;
    align-items: center;
  }
  
  .modal {
    background: white;
    padding: 2rem;
    border-radius: 6px;
    min-width: 300px;
  }
  
  .buttons {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    margin-top: 1rem;
  }
  </style>
  
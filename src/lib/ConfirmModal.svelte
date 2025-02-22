<div class="modal" class:modal-open={showConfirm}>
    <div class="modal-box">
      <h3 class="font-bold text-lg">Confirmation</h3>
      <p>Are you sure you want to stop this process?</p>
      <div class="modal-action">
        <button class="btn btn-error" on:click={confirmStop}>Yes</button>
        <button class="btn" on:click={() => showConfirm = false}>Cancel</button>
      </div>
    </div>
  </div>
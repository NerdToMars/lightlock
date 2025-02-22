<script lang="ts">
    export let label: string = 'Camera Feed';
    // add more props or logic if needed
  </script>
  
  <div class="camera-preview">
    <div class="image-placeholder">
      <p>{label}</p>
    </div>
  </div>
  
  <style>
  .camera-preview {
    width: 100%;
    aspect-ratio: 16 / 9;
    background-color: #eee;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .image-placeholder {
    color: #999;
    font-size: 1rem;
  }
  </style>
  
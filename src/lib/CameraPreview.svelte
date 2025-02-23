<script context="module" lang="ts">
  export interface CameraStatus {
    distance: number;
    frameId: number;
    isRecording: boolean;
  }
</script>

<script lang="ts">
  import { onMount } from "svelte";
  import { invoke, Channel } from "@tauri-apps/api/core";

  // Basic props for camera info
  export let label: string = "Camera Feed";
  export let no_camera_feed_label: string = "No camera feed";
  let imageSrc: string = ""; // path/URL to camera image
  // let imageDivHtml: string =
    // "<img src='{imageSrc}' alt='{label}' class='w-full h-full object-cover' />"; // HTML for the image
  // Example status data to display
  export let status: CameraStatus = {
    distance: 0,
    frameId: 0,
    isRecording: false,
  };

  type DownloadEvent =
    | {
        event: "started";
        data: {
          imageData: string;
          downloadId: number;
          contentLength: number;
        };
      }
    | {
        event: "progress";
        data: {
          downloadId: number;
          chunkLength: number;
        };
      }
    | {
        event: "finished";
        data: {
          downloadId: number;
        };
      };

  export let int_cnt: number = 0;

  onMount(() => {
    // initCamera();
    const interval = setInterval(async () => {
      const onEvent = new Channel<DownloadEvent>();
      onEvent.onmessage = (message) => {
        if (message.event === "started") {
          status.frameId = message.data.downloadId;
          status = {
            ...status,
            frameId: message.data.downloadId,
          };
          imageSrc = `data:image/jpeg;base64,${message.data.imageData}`;
          // imageDivHtml = `<img src='${imageSrc}' alt='${label}' class='w-full h-full object-cover' />`;
        }

        no_camera_feed_label = `Camera Feed event: ${message.event}`;
        console.log("got camera feed ", message);
      };
      await invoke("fetch_camera_feed", { onEvent });
      int_cnt++;
      no_camera_feed_label = `Camera Feed event: ${int_cnt}`;
      // status.frameId = int_cnt;
      // status = {
      //   ...status,
      //   frameId: int_cnt,
      // };
    }, 100);
    return () => clearInterval(interval);
  });

  // If you want a clickable "Stop" or "Record" button,
  // add a function here (e.g., handleStop) and remove pointer-events-none below.
</script>

<!-- Outer container can be styled or placed in a card, etc. -->
<div class="relative w-full h-64 bg-black overflow-hidden">
  {#if imageSrc}
    <!-- {@html imageDivHtml} -->
    <!-- The camera feed -->
    <img
      src={imageSrc}
      alt={label}
      class="w-full h-full object-cover"
    />
  {:else}
    <!-- Fallback if no imageSrc -->
    <div
      class="absolute inset-0 flex items-center justify-center text-gray-400"
    >
      {no_camera_feed_label}
    </div>
  {/if}

  <!-- Overlay container covering the image -->
  <!-- pointer-events-none means clicks pass through (no interactive elements).
       Remove it if you want clickable buttons on the overlay. -->
  <div class="absolute inset-0 pointer-events-none">
    <!-- Example crosshair lines (vertical + horizontal center) -->
    <div class="absolute inset-0 flex items-center justify-center">
      <!-- Vertical line -->
      <div class="border-l border-white h-full"></div>
      <!-- Horizontal line -->
      <div class="border-t border-white w-full absolute top-1/2 left-0"></div>
    </div>

    <!-- Top-left info (distance, altitude, battery) -->
    <div class="absolute top-2 left-2 text-white text-sm space-y-1">
      <p>Distance: {status.distance} m</p>
      <p>FrameId: {status.frameId} m</p>
    </div>

    <!-- Top-right recording indicator -->
    {#if status.isRecording}
      <div
        class="absolute top-2 right-2 flex items-center gap-1 text-red-500 font-bold"
      >
        <!-- blinking dot -->
        <div class="animate-ping h-2 w-2 rounded-full bg-red-600"></div>
        <span>REC</span>
      </div>
    {/if}

    <!-- Example of a "Stop" button in the overlay:
         (Remove pointer-events-none if you want it clickable)
         <button
           class="btn btn-error absolute bottom-2 left-2"
           on:click={handleStop}
         >
           STOP
         </button>
    -->
  </div>
</div>

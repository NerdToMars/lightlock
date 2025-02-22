// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

// src/routes/+layout.ts
import type { LayoutLoad } from './$types';

// This runs on the server (and client, if you do SSR/hybrid) before
// rendering your layout. Return props for +layout.svelte if needed.
export const load: LayoutLoad = async ({ fetch }) => {
  // Example: fetch some global data or set up universal state
  const version = 'v1.0.0'; // or fetch from an API, etc.

  return {
    appVersion: version
  };
};

// a layout which as a sidebar and a content area, sidebar has navlinks containing icons and labels



// <script>
//   // This file is your layout; 
//   // it might not hold much logic—just structural code.
// </script>

// <div class="layout-container">
//   <aside class="sidebar">
//     <!-- Sidebar content (links, icons, etc.) -->
//     <NavLink icon="home" label="Home" />
//     <NavLink icon="file-text" label="Logs" />
//     <NavLink icon="user-cog" label="Admin" />
//     <NavLink icon="cog" label="Settings" />
//   </aside>
//   <main class="content">
//     <!-- SvelteKit will inject child pages here -->
//     <slot />
//   </main>
// </div>

// <style>
//   .layout-container {
//     display: flex;
//     height: 100vh; /* full screen layout */
//   }
//   .sidebar {
//     width: 60px; /* or 200px, etc. */
//     background-color: #f5f5f5; /* example color */
//     /* more styling, e.g., icons stacked vertically */
//   }
//   .content {
//     flex: 1; /* take the remaining space */
//     display: flex;
//     flex-direction: column;
//   }
// </style>

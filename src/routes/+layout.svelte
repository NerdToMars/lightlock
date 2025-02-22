<!-- src/routes/+layout.svelte -->
<script lang="ts">
    import "../output.css"; // Tailwind + daisyUI global styles
  
    // Import some icons from svelte-hero-icons
    import { 
      Icon,
      Home,
      ClipboardDocumentList,
      UserGroup,
      Cog6Tooth
    } from 'svelte-hero-icons';
  </script>
  
  <!-- Full viewport layout: left icon bar, right content -->
  <div class="flex h-screen" data-theme="light">
    <!-- Vertical nav bar (VS Code–style) -->
    <nav class="flex flex-col w-16 bg-base-200 items-center py-4 space-y-2">
      <!-- Home -->
      <a href="/"  class="btn btn-ghost btn-square tooltip tooltip-right flex items-center justify-center"
      data-tip="Home">
        
        <Icon src="{Home}" class="h-6 w-6" />
      </a>
  
      <!-- Logs -->
      <a href="/logs" class="btn btn-ghost btn-square tooltip tooltip-right flex items-center justify-center" data-tip="Logs">
        <Icon src="{ClipboardDocumentList}" class="h-6 w-6" />
      </a>
  
      <!-- Admin -->
      <a href="/admin" class="btn btn-ghost btn-square tooltip tooltip-right flex items-center justify-center" data-tip="Admin">
        <Icon src="{UserGroup}" class="h-6 w-6" />
      </a>
  
      <!-- Spacer pushes Settings to bottom -->
      <div class="flex-1"></div>
  
      <!-- Settings -->
      <a href="/settings" class="btn btn-ghost btn-square tooltip tooltip-right flex items-center justify-center" data-tip="Settings">
        <Icon src="{Cog6Tooth}" class="h-6 w-6" />
      </a>
    </nav>
  
    <!-- Main content area on the right -->
    <main class="flex-1 bg-base-100 p-4">
      <!-- SvelteKit child pages render here -->
      <slot />
    </main>
  </div>
  
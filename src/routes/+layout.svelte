<script lang="ts">
//   export let data; 
  // 'data' receives whatever you return from load() in +layout.ts
  // e.g., data.appVersion
</script>

<div class="layout">
  <aside class="sidebar">
    <!-- Navigation links -->
    <ul>
      <li><a href="/">Home</a></li>
      <li><a href="/logs">Logs</a></li>
      <li><a href="/admin">Admin</a></li>
      <li><a href="/settings">Settings</a></li>
    </ul>
  </aside>

  <main class="content">
    <header>
      <!-- Example usage of data from +layout.ts -->
      <!-- <small>App Version: {data?.appVersion}</small> -->
    </header>

    <!-- child route content goes here -->
    <slot />
  </main>
</div>

<style>
.layout {
  display: flex;
  height: 100vh;
}
.sidebar {
  width: 200px;
  background-color: #f5f5f5;
}
.content {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
}
</style>

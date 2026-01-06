<script setup lang="ts">
import { RouterView, RouterLink } from "vue-router";
</script>

<template>
  <div class="app">
    <nav class="navbar">
      <div class="nav-brand">
        <h2>NeoLan</h2>
      </div>
      <div class="nav-links">
        <RouterLink to="/peers" class="nav-link">节点列表</RouterLink>
        <RouterLink to="/settings" class="nav-link">设置</RouterLink>
      </div>
    </nav>
    <main class="main-content">
      <RouterView />
    </main>
  </div>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.navbar {
  background-color: #ffffff;
  border-bottom: 1px solid #e0e0e0;
  padding: 0 20px;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.nav-brand h2 {
  font-size: 1.5rem;
  color: #24c8db;
  font-weight: 600;
}

.nav-links {
  display: flex;
  gap: 20px;
}

.nav-link {
  text-decoration: none;
  color: #646cff;
  font-weight: 500;
  padding: 8px 16px;
  border-radius: 6px;
  transition: background-color 0.2s, color 0.2s;
}

.nav-link:hover {
  background-color: #f0f0f0;
  color: #535bf2;
}

.nav-link.router-link-active {
  background-color: #e8e8e8;
  color: #24c8db;
}

.main-content {
  flex: 1;
  padding: 20px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  .navbar {
    background-color: #1f1f1f;
    border-bottom-color: #3f3f3f;
  }

  .nav-link:hover {
    background-color: #3f3f3f;
    color: #24c8db;
  }

  .nav-link.router-link-active {
    background-color: #0f0f0f98;
    color: #24c8db;
  }
}
</style>
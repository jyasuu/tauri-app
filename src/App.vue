<template>
  <div>
    <NavMenu />
    <router-view></router-view>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import NavMenu from './components/NavMenu.vue';

export default defineComponent({
  components: {
    NavMenu,
  },
});
</script>

<style>
/* Global styles can be added here */
</style>

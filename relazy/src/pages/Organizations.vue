<template>
  <div>
    <h2>Organizations</h2>
    <ul>
      <li v-for="org in organizations" :key="org.id">
        <img :src="org.avatar_url" :alt="org.login" width="32" />
        {{ org.login }}
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

interface Organization {
  id: number;
  login: string;
  avatar_url: string;
}

const organizations = ref<Organization[]>([]);

onMounted(async () => {
  organizations.value = await invoke("get_organizations");
});
</script>

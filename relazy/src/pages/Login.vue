<template>
  <div>
    <h2>Login</h2>
    <input v-model="token" type="password" placeholder="GitHub Token" />
    <button @click="login">Login with GitHub</button>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const token = ref("");

async function login() {
  await invoke("login", { token: token.value });
  router.push({ name: "Organizations" });
}
</script>

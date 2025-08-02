<template>
  <div>
    <h2>Login</h2>
    <input v-model="token" placeholder="GitHub Token" type="password"/>
    <button @click="login">Login with GitHub</button>
  </div>
</template>

<script lang="ts" setup>
import {invoke} from "@tauri-apps/api/core";
import {ref} from "vue";
import {useRouter} from "vue-router";

const router = useRouter();
const token = ref("");

async function login() {
  await invoke("login", {token: token.value});
  router.push({name: "Organizations"});
}
</script>

<template>
  <div>
    <h1>Relazy</h1>
    <router-view></router-view>
  </div>
</template>

<script lang="ts" setup>
import {onMounted} from 'vue';
import {useRouter} from 'vue-router';

const router = useRouter();

onMounted(() => {
  // For now, we'll just navigate to the login page
  router.push({name: 'Login'});
});
</script>

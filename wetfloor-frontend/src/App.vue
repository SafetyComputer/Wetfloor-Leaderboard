<template>
  <div class="bg-gradient-to-b from-orange-100 to-red-100 -z-20 w-full h-full min-h-[100vh]">
    <Suspense>
      <RouterView></RouterView>
    </Suspense>
  </div>
</template>

<script setup>
import {RouterView} from 'vue-router'
</script>
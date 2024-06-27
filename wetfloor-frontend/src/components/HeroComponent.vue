<template>
  <div class="hidden lg:flex justify-center items-center font-mono">
    <h1 class="text-4xl lg:text-5xl xl:text-6xl -mr-10 font-bold">
      Wetfloor
    </h1>
    <img alt="Wetfloor" class="relative w-40 md:w-60 lg:w-80" src="../assets/wetfloor-classic.png"
         @click="$router.push('/') ">
    <h1 class="text-4xl lg:text-5xl xl:text-6xl -ml-10 font-bold">
      Leaderboard
    </h1>
  </div>
  <div class="flex  lg:hidden justify-center items-center font-mono">
    <img alt="Wetfloor" class="relative w-36 -ml-12 md:w-48 md:-mr-4" src="../assets/wetfloor-classic.png"
         @click="$router.push('/')">
    <div class="flex flex-col md:gap-4">
      <h1 class="text-3xl md:text-4xl font-bold">
        Wetfloor
      </h1>
      <h1 class="text-3xl md:text-4xl font-bold">
        Leaderboard
      </h1>
    </div>
  </div>
</template>
<script lang="ts" setup>
</script>
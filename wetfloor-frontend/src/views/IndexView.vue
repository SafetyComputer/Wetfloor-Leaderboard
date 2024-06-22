<template>
  <div class="container mx-auto flex flex-col gap-8">
    <HeroComponent></HeroComponent>
    <RouterLink
        class="block mx-auto p-2 border-2 w-[80%] text-2xl text-center rounded-xl border-amber-800 hover:bg-yellow-100 transition cursor-pointer"
        to="/add">
      Add Match Record
    </RouterLink>

    <div class="flex flex-col w-[80%] max-w-[800px] mx-auto bg-amber-50 rounded-xl shadow-2xl">
      <h2 class="text-3xl font-bold text-center font-serif py-4">Real Time Leaderboard
      </h2>
      <hr>
      <div class="grid grid-cols-3 gap-4 p-4 items-center text-center md:text-2xl">
        <div>1</div>
        <div>HAJAHA</div>
        <div>2000</div>
      </div>
    </div>
  </div>
</template>
<script lang="ts" setup>
import HeroComponent from "../components/HeroComponent.vue";
</script>
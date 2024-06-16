<template>
  <div class="bg-gradient-to-b from-orange-100 to-red-100 -z-20 w-full h-[100vh]">
    <div class="flex justify-center items-center">
      <h1 class="text-3xl lg:text-4xl xl:text-5xl -mr-12 font-bold">
        Wetfloor
      </h1>
      <img alt="Wetfloor" class="relative " src="./assets/img.png">
      <h1 class="text-3xl lg:text-4xl xl:text-5xl -ml-14 font-bold">
        Leaderboard
      </h1>
    </div>
    <hr class="my-4">

    <div
        class="mx-auto border-2 w-80 text-2xl text-center rounded border-amber-800 hover:bg-yellow-100 transition cursor-pointer">
      Add Match Record
    </div>

    <hr class="my-4">
    <h2 class="text-2xl font-bold text-center">Leaderboard</h2>
    <hr class="my-4">

    <div class="flex flex-col w-[80%]  max-w-[800px] mx-auto gap-10">
      <div
          class="border-2 border-amber-600 w-full mx-auto rounded-xl text-2xl font-bold flex justify-between p-4 items-center">
        <span class="text-red-500 text-3xl">1</span>
        <span class="">HAJAHA</span>
        <span class="text-yellow-600">2000</span>
      </div>
      <div
          class="border-2 border-amber-600 w-full mx-auto rounded-xl text-2xl font-bold flex justify-between p-4 items-center">
        <span class="text-red-500 text-3xl">1</span>
        <span class="">HAJAHA</span>
        <span class="text-yellow-600">2000</span>
      </div>
    </div>
  </div>

</template>
<script lang="ts" setup>
</script>
<script setup>

</script>

<template>
  <div class="bg-gradient-to-b from-orange-100 to-red-100 -z-20 w-full h-[100vh]">
    <div class="flex justify-center items-center">
      <h1 class="text-3xl lg:text-4xl xl:text-5xl -mr-12 font-bold">
        Wetfloor
      </h1>
      <img alt="Wetfloor" class="relative " src="../assets/img.png">
      <h1 class="text-3xl lg:text-4xl xl:text-5xl -ml-14 font-bold">
        Leaderboard
      </h1>
    </div>
    <hr class="my-4">
    <h2 class="text-2xl font-bold text-center">Add Match Record</h2>
    <hr class="my-4">

    <div class="flex flex-col justify-center items-center gap-8">
      <div>
        <label>Date</label>
        <input id="" name="" type="date">
      </div>

      <div>
        <div><label>Player 1</label>
          <select>
            <option disabled selected value="">Select Player 1</option>
            <option value="1">HAJAHA</option>
            <option value="2">主教</option>
          </select>
          <label for="">Score</label>
          <input id="" name="" type="number">
        </div>
        <div>
          <label>Player 2</label>
          <select>
            <option disabled selected value="">Select Player 2</option>
            <option value="1">HAJAHA</option>
            <option value="2">主教</option>
          </select>
          <label for="">Score</label>
          <input id="" name="" type="number">
        </div>
      </div>
      <div
          class="mx-auto border-2 w-80 text-2xl text-center rounded border-amber-800 hover:bg-yellow-100 transition cursor-pointer">
        <RouterLink to="/">Submit</RouterLink>
      </div>
    </div>
  </div>

</template>

<template>
  <div class="container mx-auto flex flex-col gap-8 pb-8">
    <HeroComponent></HeroComponent>
    <RouterLink
        class="block mx-auto p-2 border-2 w-[80%] max-w-[800px] text-2xl text-center rounded-xl border-amber-800 hover:bg-yellow-100 transition cursor-pointer"
        to="/match">
      Recent Matches
    </RouterLink>
    <RouterLink
        class="block mx-auto p-2 border-2 w-[80%] max-w-[800px] text-2xl text-center rounded-xl border-amber-800 hover:bg-yellow-100 transition cursor-pointer"
        to="/add">
      Add Match Record
    </RouterLink>

    <div class="flex flex-col w-[80%] max-w-[800px] mx-auto bg-amber-50 rounded-xl shadow-2xl">
      <h2 class="text-3xl font-bold text-center font-serif py-4">Real Time Leaderboard
      </h2>
      <hr>
      <div v-for="(player, index) in players" :key="player.id"
           class="grid grid-cols-3 gap-4 p-4 items-center text-center text-xl md:text-2xl">
        <img v-if="index === 0" alt="Gold Medal" class="w-12 lg:w-16 mx-auto"
             src="../assets/wetfloor-gold.png">
        <img v-else-if="index === 1" alt="Silver Medal" class="w-12 lg:w-16 mx-auto"
             src="../assets/wetfloor-silver.png">
        <img v-else-if="index === 2" alt="Bronze Medal" class="w-12 lg:w-16 mx-auto"
             src="../assets/wetfloor-bronze.png">
        <div v-else>#{{ index + 1 }}</div>
        <div :class="{
          'font-bold': index < 3,
          'text-yellow-600': index === 0,
          'text-gray-500': index ===1,
          'text-orange-800': index === 2
        }">{{ player.name }}
        </div>
        <div :class="{
          'font-bold': index < 3,
          'text-yellow-600': index === 0,
          'text-gray-500': index ===1,
          'text-orange-800': index === 2
        }">{{ player.elo }}
        </div>
      </div>
    </div>
  </div>
</template>
<script setup>
import HeroComponent from "../components/HeroComponent.vue";
import axios from "axios";
import {ref} from "vue";

let players = ref([]);

const getPlayers = async () => {
      return await axios.get(
          "https://149.104.27.233:35670/player", {
            headers: {
              "Access-Control-Allow-Origin": "*",
              "Access-Control-Allow-Methods": "GET, POST, PATCH, PUT, DELETE, OPTIONS",
              "Access-Control-Allow-Headers": "Origin, Content-Type, X-Auth-Token, Authorization, Accept,charset,boundary,Content-Length",
              'Access-Control-Allow-Credentials': 'true',
            }
          }
      )
          .then((res) => {
            players.value = res.data.sort((a, b) => b.elo - a.elo);
          })
          .catch((err) => {
            console.error(err);
          });
    }
;

getPlayers();
</script>
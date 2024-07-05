<template>
  <div class="container mx-auto flex flex-col gap-8 pb-8">
    <HeroComponent></HeroComponent>
    <div v-for="match in matches" :key="match.id"
         class="grid grid-cols-3 items-center text-center py-8 w-[80%] max-w-[800px] mx-auto bg-amber-50 rounded-xl shadow-2xl">
      <div>
        <div class="font-bold text-orange-600 text-4xl">{{ match.win_points }}</div>
        <div class="font-bold text-xl">{{ match.winner.name }}</div>
      </div>
      <div>
        <div class="font-bold text-orange-400 text-4xl">VS</div>
        <div>{{ moment(match.time).calendar() }}</div>
      </div>
      <div>
        <div class="font-bold text-gray-500 text-4xl">{{ match.lose_points }}</div>
        <div class="font-bold text-xl">{{ match.loser.name }}</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import HeroComponent from "../components/HeroComponent.vue";
import axios from "axios";
import {ref} from "vue";
import moment from "moment";

let matches = ref([]);

const getMatches = async () => {
  return await axios.get("/api/match",)
      .then((res) => {
        matches.value = res.data
            .sort((a, b) => moment(a.time) - moment(b.time))
            .reverse();
      })
      .catch((err) => {
        console.error(err);
      });
};

getMatches()
</script>

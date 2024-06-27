<template>
  <HeroComponent></HeroComponent>
  <h2 class="text-3xl xl:text-4xl 2xl:text-5xl py-8 font-bold text-center">Add Match Record</h2>

  <div
      class="flex flex-col justify-center items-center gap-8 mx-auto p-8 border-2 w-[80%] max-w-[800px] lg:text-2xl text-center rounded-xl border-amber-800">
    <div class="flex gap-8">
      <label>Date</label>
      <input v-model="date" class="w-40 lg:w-80 rounded shadow" type="datetime-local">
    </div>

    <div class="flex flex-col gap-8">
      <div class="flex flex-col lg:flex-row gap-8 items-center justify-center">
        <div class="space-x-2">
          <label>Player 1</label>
          <select v-model="player1" class="w-40 rounded shadow">
            <option disabled selected value="">Select Player 1</option>
            <option v-for="player in players" :key="player.id" :value="player.id">{{ player.name }}</option>
          </select>
        </div>
        <div class="space-x-2">
          <label for="">Score 1</label>
          <input v-model="score1" class="w-40 rounded shadow" type="number">
        </div>
      </div>
      <div class="flex flex-col lg:flex-row gap-8 items-center justify-center">
        <div class="space-x-2">
          <label>Player 2</label>
          <select v-model="player2" class="w-40 rounded shadow">
            <option disabled selected value="">Select Player 2</option>
            <option v-for="player in players" :key="player.id" :value="player.id">{{ player.name }}</option>
          </select>
        </div>
        <div class="space-x-2">
          <label for="">Score 2</label>
          <input v-model="score2" class="w-40 rounded shadow" type="number">
        </div>
      </div>
    </div>

  </div>
  <div
      class="mx-auto p-2 my-8 border-2 w-[80%] max-w-[800px] text-2xl text-center rounded-xl border-amber-800 hover:bg-yellow-100 transition cursor-pointer"
      @click="submitMatch">
    Submit
  </div>
</template>

<script setup>
import HeroComponent from "../components/HeroComponent.vue";
import axios from "axios";
import {ref} from "vue";
import router from "../router/index.js";

let players = ref([]);

let date = ref(null);
let player1 = ref(null);
let player2 = ref(null);
let score1 = ref(null);
let score2 = ref(null);

const getPlayers = async () => {
  return await axios.get("/api/player")
      .then((res) => {
        players.value = res.data.sort((a, b) => b.elo - a.elo);
      })
      .catch((err) => {
        console.error(err);
      });
};

const submitMatch = async () => {
  if (!player1.value || !player2.value || !score1.value || !score2.value) {
    alert("Please fill in all fields");
    return;
  }

  if (player1.value === player2.value) {
    alert("Players cannot play against themselves");
    return;
  }

  let winner = score1.value > score2.value ? player1.value : player2.value;
  let loser = score1.value > score2.value ? player2.value : player1.value;
  let winnerScore = score1.value > score2.value ? score1.value : score2.value;
  let loserScore = score1.value > score2.value ? score2.value : score1.value;

  return await axios.post(
      "/api/match",
      {
        time: date.value,
        winner: winner,
        loser: loser,
        win_points: winnerScore,
        lose_points: loserScore
      })
      .then((res) => {
        console.log(res);
        alert("Match submitted successfully");
        router.push("/");
      })
      .catch((err) => {
        console.error(err);
      });
};

getPlayers();
</script>

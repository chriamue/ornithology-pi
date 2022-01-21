<template>
  <div id="app">
    <BLE />
    blabla
  </div>
</template>

<script>
import BLE from "./components/BLE.vue";
export default {
  name: "App",
  components: {
    BLE,
  },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>

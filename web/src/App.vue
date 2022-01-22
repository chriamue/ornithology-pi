<template>
  <div id="app">
    <BLEDeviceList />
  </div>
</template>

<script>
import BLEDeviceList from "./components/BLEDeviceList.vue";
export default {
  name: "App",
  components: {
    BLEDeviceList,
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

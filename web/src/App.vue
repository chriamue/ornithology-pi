<template>
  <div id="app">
    <Header />
    <About />
    <BluetoothConnect />
    <Footer />
  </div>
</template>

<script>
import About from "./components/About.vue";
import BluetoothConnect from "./components/BluetoothConnect.vue";
import Footer from "./components/Footer.vue";
import Header from "./components/Header.vue";

export default {
  name: "App",
  components: {
    About,
    BluetoothConnect,
    Footer,
    Header,
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

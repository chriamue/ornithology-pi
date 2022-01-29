<template>
  <footer>
    <p>Copyright 2022 chriamue</p>
    <p>
      <font-awesome-icon :icon="['fab', 'github']" />
      <a href="https://github.com/chriamue/ornithology-pi"
        >chriamue/ornithology-pi</a
      >
    </p>
  </footer>
</template>
<script>
export default {
  name: "Footer",
  components: {},
};
</script>
<style scoped>
footer {
  background: darkgreen;
  padding: 6px;
  position: fixed;
  width: 100%;
  height: 100px;
  bottom: 0px;
}
p {
  color: lightgreen;
  text-align: center;
}
</style>
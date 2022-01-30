<template>
  <div class="sighting-count">
    <h3>Last Bird</h3>
    {{ species }}
  </div>
</template>
<script>
const characteristic = "00000000-0000-0000-000f-00dc0de00005";
import { defineComponent } from "vue";

export default defineComponent({
  name: "LastSighting",
  props: {
    service: Object,
  },
  data() {
    return {
      species: "unknown",
    };
  },
  components: {},
  watch: {
    service: {
      handler: function () {
        this.service
          .getCharacteristic(characteristic)
          .then(this.handleCharacteristic);
      },
    },
  },
  created() {
    this.service
      .getCharacteristic(characteristic)
      .then(this.handleCharacteristic);
  },
  methods: {
    handleCharacteristic(handle) {
      console.log("handle", handle);
      handle.readValue().then((value) => {
          const decoder = new TextDecoder("utf-8");
          this.species = decoder.decode(value);
      });
    },
  },
});
</script>
<style scoped>
</style>
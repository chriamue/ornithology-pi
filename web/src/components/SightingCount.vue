<template>
  <div class="sighting-count">
    <h3>Birds Counted</h3>
    {{ count }}
  </div>
</template>
<script>
const characteristic = "00000000-0000-0000-000f-00dc0de00004";
import { defineComponent } from "vue";

export default defineComponent({
  name: "SightingCount",
  props: {
    service: Object,
  },
  data() {
    return {
      count: 0,
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
          this.count = Number.parseInt(decoder.decode(value));
      });
    },
  },
});
</script>
<style scoped>
</style>
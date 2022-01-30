<template>
  <div>
    <button
      v-if="!service"
      @click="search"
      title="Connect to your Ornithology PI"
    >
      Connect via Bluetooth
    </button>
    <button
      v-if="service"
      @click="disconnect"
      title="Disconnect Ornithology PI"
    >
      Disconnect
    </button>
    <SightingCount v-if="service" :service="service" />
    <LastSighting v-if="service" :service="service" />
  </div>
</template>

<script>
import SightingCount from "./SightingCount.vue";
import LastSighting from "./LastSighting.vue";

export default {
  name: "BluetoothConnect",
  components: {
    SightingCount,
    LastSighting,
  },
  data() {
    return {
      services: ["00000000-0000-0000-000f-00dc0de00001"],
      server: null,
      service: null,
    };
  },
  methods: {
    search() {
      navigator.bluetooth
        .requestDevice({
          filters: [{ services: this.services }],
        })
        .then((device) => {
          console.log(device.name);
          console.log(device);
          this.handleConnectedDevices(device);

          return device.gatt.connect();
        })
        .then((server) => {
          this.server = server;
          server.getPrimaryServices().then(console.log);
          return server.getPrimaryService(this.services[0]);
        })
        .then(this.handleConnectedService)
        .catch((error) => {
          console.error(error);
        });
    },
    disconnect() {
      this.server.disconnect();
      this.server = null;
      this.service = null;
    },
    handleConnectedDevices(devices) {
      console.log("Connected devices:", devices);
    },
    handleConnectedService(service) {
      console.log("Connected service:", service);
      this.service = service;
      service.getCharacteristics().then(console.log);
    },
  },
};
</script>

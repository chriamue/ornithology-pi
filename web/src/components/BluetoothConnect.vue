<template>
  <div>
    <button @click="search" title="Connect to your Ornithology PI">Connect via Bluetooth</button>
  </div>
</template>

<script>
export default {
  name: "BluetoothConnect",
  components: {},
  data() {
    return {
      services: [0x1234],
    };
  },
  methods: {
    search() {
      navigator.bluetooth
        .requestDevice({ filters: [{ name: "ornithology-pi" }] })
        .then((device) => {
          console.log(device.name);
          console.log(device);

          return device.gatt.connect();
        })
        .then((server) => {
          console.log(server);
          //return server.getPrimaryService(0x1234)
        }) /*
        .then((service) => service.getCharacteristic("measurement_interval"))
        .then((characteristic) =>
          characteristic.getDescriptor("gatt.characteristic_user_description")
        )
        .then((descriptor) => descriptor.readValue())
        .then((value) => {
          const decoder = new TextDecoder("utf-8");
          console.log(`User Description: ${decoder.decode(value)}`);
        })*/
        .catch((error) => {
          console.error(error);
        });
    },
    handleScannedDevices(devices) {
      console.log("Scanned devices:", devices);
    },
    handleConnectedDevices(devices) {
      console.log("Connected devices:", devices);
    },
  },
};
</script>

<template>
  <div>
    <button @click="search" title="Connect to your Ornithology PI">
      Connect via Bluetooth
    </button>
  </div>
</template>

<script>
export default {
  name: "BluetoothConnect",
  components: {},
  data() {
    return {
      services: ['00000000-0000-0000-000f-00dc0de00001'],
    };
  },
  methods: {
    search() {
      navigator.bluetooth
        .requestDevice({
          filters: [{services: this.services}/*{ name: "ornithology-pi" }*/],
          //optionalServices: this.services,
        })
        .then((device) => {
          console.log(device.name);
          console.log(device);

          return device.gatt.connect();
        })
        .then((server) => {
          console.log(server);
          server.getPrimaryServices().then(console.log);
          return server.getPrimaryService(this.services[0]);
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

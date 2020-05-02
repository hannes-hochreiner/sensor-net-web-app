<template>
  <v-card>
    <v-card-title>{{equipment.info || equipment.id}}</v-card-title>
    <v-card-subtitle>{{formatDate(data[0].ts)}} @ {{data[0].rssi}}dB</v-card-subtitle>
    <v-card-text>
      <Sensor v-for="sensorId in Object.getOwnPropertyNames(dataBySensor).sort()"
        :config="config" :sensor="config.sensors[sensorId]" :key="sensorId"
        :data="dataBySensor[sensorId]"
      />
    </v-card-text>
  </v-card>
</template>
<script>
import Sensor from '@/components/Sensor.vue'

export default {
  name: 'Equipment',
  components: {
    Sensor
  },
  props: {
    equipment: Object,
    config: Object,
    data: Array
  },
  methods: {
    formatDate(date) {
      return `${date.getFullYear()}-${this.padLeft(date.getMonth() + 1, 2)}-${this.padLeft(date.getDate(), 2)} ${this.padLeft(date.getHours(), 2)}:${this.padLeft(date.getMinutes(), 2)}`;
    },
    padLeft(str, len) {
      let newStr = `${str}`;

      while (newStr.length < len) {
        newStr = '0' + newStr;
      }

      return newStr;
    }
  },
  computed: {
    dataBySensor: function() {
      return this.data.reduce((acc, curr) => {
        if (typeof acc[curr._sensor_id] == 'undefined') {
          acc[curr._sensor_id] = [];
        }

        acc[curr._sensor_id].push(curr);

        return acc;
      },{});
    }
  }
}
</script>

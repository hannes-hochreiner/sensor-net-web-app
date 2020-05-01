<template>
  <v-card>
    <v-card-title>{{equipment.info || equipment.id}}</v-card-title>
    <v-card-subtitle>sdusfiu34jks23</v-card-subtitle>
    <v-card-text>
      <Sensor v-for="sensorId in Object.getOwnPropertyNames(dataBySensor)"
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

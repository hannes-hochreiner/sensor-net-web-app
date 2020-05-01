<template>
  <div class="home">
    <div v-if="!$auth.loading">
      <!-- show login when not authenticated -->
      <v-btn v-if="!$auth.isAuthenticated" @click="login">Log in</v-btn>
      <div v-if="$auth.isAuthenticated">
        <v-btn @click="updateData">update</v-btn>
        <Equipment v-for="equipmentId in Object.getOwnPropertyNames(dataByEquipment)"
          :key="equipmentId" :config="config" :equipment="config.equipment[equipmentId]"
          :data="dataByEquipment[equipmentId]"/>
      </div>
    </div>
  </div>
</template>

<script>
import axios from "axios";
// @ is an alias to /src
import Equipment from '@/components/Equipment.vue'

export default {
  name: 'Home',
  components: {
    Equipment
  },
  data: () => ({
    config: {
      parameter_types: {},
      sensors: {},
      equipment: {}
    },
    data: []
    // config: {
    //   parameter_types: {
    //     'pt1': {_id: 'pt1', id: 'humidity', unit:'%'},
    //     'pt2': {_id: 'pt2', id: 'temperature', unit:'°C'},
    //     'pt3': {_id: 'pt3', id: 'pressure', unit:'mbar'}
    //   },
    //   sensors: {
    //     's1': {_id: 's1', id: 'be01', info:null},
    //     's2': {_id: 's2', id: 'be02', info:null},
    //   },
    //   equipment: {
    //     'e1': {_id: 'e1', id: '22343-234se2-2324', info: null},
    //     'e2': {_id: 'e2', id: '11343-234se2-2324', info: null}
    //   }
    // },
    // data: [
    //   {_measurement_id: 'm10', ts: '2020-04-20T12:00:00Z', index: 1234, rssi: -90.5, _equipment_id: 'e1', _parameter_id: 'p1', _parameter_type_id: 'pt1', _sensor_id: 's1', val:50.2 },
    //   {_measurement_id: 'm10', ts: '2020-04-20T12:00:00Z', index: 1234, rssi: -90.5, _equipment_id: 'e1', _parameter_id: 'p2', _parameter_type_id: 'pt2', _sensor_id: 's1', val:12.32 },
    //   {_measurement_id: 'm10', ts: '2020-04-20T12:00:00Z', index: 1234, rssi: -90.5, _equipment_id: 'e1', _parameter_id: 'p3', _parameter_type_id: 'pt3', _sensor_id: 's1', val:1002.32 },
    //   {_measurement_id: 'm11', ts: '2020-04-20T12:02:00Z', index: 1235, rssi: -90.5, _equipment_id: 'e1', _parameter_id: 'p1', _parameter_type_id: 'pt1', _sensor_id: 's1', val:54.2 },
    //   {_measurement_id: 'm11', ts: '2020-04-20T12:02:00Z', index: 1235, rssi: -90.5, _equipment_id: 'e1', _parameter_id: 'p2', _parameter_type_id: 'pt2', _sensor_id: 's1', val:14.32 },
    //   {_measurement_id: 'm11', ts: '2020-04-20T12:02:00Z', index: 1235, rssi: -90.5, _equipment_id: 'e1', _parameter_id: 'p3', _parameter_type_id: 'pt3', _sensor_id: 's1', val:1004.32 },
    //   {_measurement_id: 'm12', ts: '2020-04-20T12:03:00Z', index: 1236, rssi: -90.5, _equipment_id: 'e1', _parameter_id: 'p1', _parameter_type_id: 'pt1', _sensor_id: 's1', val:53.2 },
    //   {_measurement_id: 'm12', ts: '2020-04-20T12:03:00Z', index: 1236, rssi: -90.5, _equipment_id: 'e1', _parameter_id: 'p2', _parameter_type_id: 'pt2', _sensor_id: 's1', val:13.32 },
    //   {_measurement_id: 'm12', ts: '2020-04-20T12:03:00Z', index: 1236, rssi: -90.5, _equipment_id: 'e1', _parameter_id: 'p3', _parameter_type_id: 'pt3', _sensor_id: 's1', val:1003.32 },
    //   {_measurement_id: 'm13', ts: '2020-04-20T12:05:00Z', index: 1237, rssi: -90.5, _equipment_id: 'e1', _parameter_id: 'p1', _parameter_type_id: 'pt1', _sensor_id: 's1', val:55.2 },
    //   {_measurement_id: 'm13', ts: '2020-04-20T12:05:00Z', index: 1237, rssi: -90.5, _equipment_id: 'e1', _parameter_id: 'p2', _parameter_type_id: 'pt2', _sensor_id: 's1', val:16.32 },
    //   {_measurement_id: 'm13', ts: '2020-04-20T12:05:00Z', index: 1237, rssi: -90.5, _equipment_id: 'e1', _parameter_id: 'p3', _parameter_type_id: 'pt3', _sensor_id: 's1', val:999.32 },
    //   {_measurement_id: 'm2', ts: '2020-04-20T12:02:00Z', index: 12, rssi: -83.5, _equipment_id: 'e2', _parameter_id: 'p1', _parameter_type_id: 'pt1', _sensor_id: 's2', val:55.1 },
    //   {_measurement_id: 'm2', ts: '2020-04-20T12:02:00Z', index: 12, rssi: -83.5, _equipment_id: 'e2', _parameter_id: 'p2', _parameter_type_id: 'pt2', _sensor_id: 's2', val:20.54 },
    //   {_measurement_id: 'm2', ts: '2020-04-20T12:02:00Z', index: 12, rssi: -83.5, _equipment_id: 'e2', _parameter_id: 'p3', _parameter_type_id: 'pt3', _sensor_id: 's2', val:1012.42 }
    // ]
  }),
  methods: {
    // Log the user in
    login() {
      this.$auth.loginWithRedirect();
    },
    async updateData() {
      try {
        const token = await this.$auth.getTokenSilently();
        this.config.sensors = (await axios.get("/api/sensor", {headers: {Authorization: `Bearer ${token}`}})).result.reduce((acc,curr) => {
          acc[curr._id] = curr;
          return acc;
        }, {});
        console.log(this.config.sensors);
        this.config.parameter_types = (await axios.get("/api/parameter_type", {headers: {Authorization: `Bearer ${token}`}})).result.reduce((acc,curr) => {
          acc[curr._id] = curr;
          return acc;
        }, {});
        console.log(this.config.parameter_types);
        this.config.equipment = (await axios.get("/api/equipment", {headers: {Authorization: `Bearer ${token}`}})).result.reduce((acc,curr) => {
          acc[curr._id] = curr;
          return acc;
        }, {});
        console.log(this.config.equipment);
        let startTime = new Date();
        startTime.setUTCHours(startTime.getUTCHours() - 36);
        let endTime = new Date();
        this.data = (await axios.get("/api/measurement_data", {headers: {Authorization: `Bearer ${token}`, params: {startTime: startTime, endTime: endTime}}})).result.map(elem => {
          elem.ts = new Date(elem.ts);
          return elem;
        }).sort((elem1, elem2) => {
          if (elem1 == elem2) {
            return 0;
          } else if (elem1 < elem2) {
            return 1;
          } else {
            return -1;
          }
        });
        console.log(this.data);
      } catch (error) {
        console.log(error);
      }
    }
  },
  computed: {
    dataByEquipment: function() {
      return this.data.reduce((acc, curr) => {
        if (typeof acc[curr._equipment_id] == 'undefined') {
          acc[curr._equipment_id] = [];
        }

        acc[curr._equipment_id].push(curr);

        return acc;
      },{});
    }
  }
}
</script>

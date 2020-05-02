<template>
  <div class="home">
    <v-container fluid>
      <v-row>
        <v-col cols="12" md="6" lg="4" v-for="equipmentId in Object.getOwnPropertyNames(dataByEquipment).sort()"
            :key="equipmentId">
          <Equipment :config="config" :equipment="config.equipment[equipmentId]"
            :data="dataByEquipment[equipmentId]"/>
        </v-col>
      </v-row>
    </v-container>
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
  }),
  mounted() {
    let startTime = new Date();
    startTime.setUTCHours(startTime.getUTCHours() - 36);
    this.startTime = startTime;
    this.updateData.bind(this)();
    this.updateDataIntId = window.setInterval(this.updateData.bind(this), 1000 * 60);
  },
  beforeDestroy() {
    if (typeof this.updateDataIntId !== 'undefined') {
      window.clearInterval(this.updateDataIntId);
      delete this.updateDataIntId;
    }
  },
  methods: {
    async updateData() {
      try {
        const token = await this.$auth.getTokenSilently();
        this.config.sensors = (await axios.get("/api/sensor", {headers: {Authorization: `Bearer ${token}`}})).data.result.reduce((acc,curr) => {
          acc[curr._id] = curr;
          return acc;
        }, {});
        console.log(this.config.sensors);
        this.config.parameter_types = (await axios.get("/api/parameter_type", {headers: {Authorization: `Bearer ${token}`}})).data.result.reduce((acc,curr) => {
          acc[curr._id] = curr;
          return acc;
        }, {});
        console.log(this.config.parameter_types);
        this.config.equipment = (await axios.get("/api/equipment", {headers: {Authorization: `Bearer ${token}`}})).data.result.reduce((acc,curr) => {
          acc[curr._id] = curr;
          return acc;
        }, {});
        console.log(this.config.equipment);
        let endTime = new Date();
        this.data = (await axios.get("/api/measurement_data", {headers: {Authorization: `Bearer ${token}`}, params: {startTime: this.startTime, endTime: endTime}})).data.result.map(elem => {
          elem.ts = new Date(elem.ts);
          return elem;
        }).concat(this.data).sort((elem1, elem2) => {
          if (elem1.ts == elem2.ts) {
            return 0;
          } else if (elem1.ts < elem2.ts) {
            return 1;
          } else {
            return -1;
          }
        }).slice(0, 1501);
        this.startTime = endTime;
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

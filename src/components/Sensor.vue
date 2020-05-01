<template>
  <v-card>
    <v-card-title>
      {{sensor.info || sensor.id}}
    </v-card-title>
    <v-card-text>
        <v-expansion-panels>
        <v-expansion-panel v-for="parameterTypeId in Object.getOwnPropertyNames(dataByParameterType)" :key="parameterTypeId">
          <v-expansion-panel-header>
             <v-row no-gutters>
                <v-col cols="4" align="right">{{config.parameter_types[parameterTypeId].id}}</v-col>
                <v-col cols="4" align="right">{{dataByParameterType[parameterTypeId][0].val}}</v-col>
                <v-col cols="4" align="left">{{config.parameter_types[parameterTypeId].unit}}</v-col>
             </v-row>
          </v-expansion-panel-header>
          <v-expansion-panel-content>
            <LineChart :data="dataByParameterType[parameterTypeId]"/>
          </v-expansion-panel-content>
        </v-expansion-panel>
      </v-expansion-panels>
    </v-card-text>
  </v-card>
</template>
<script>
import LineChart from '@/components/LineChart.vue'

export default {
  name: 'Equipment',
  components: {
    LineChart
  },
  props: {
    sensor: Object,
    config: Object,
    data: Array
  },
  computed: {
    dataByParameterType: function() {
      return this.data.reduce((acc, curr) => {
        if (typeof acc[curr._parameter_type_id] == 'undefined') {
          acc[curr._parameter_type_id] = [];
        }
        acc[curr._parameter_type_id].push(curr);

        return acc;
      }, {});
    }
  }
}
</script>
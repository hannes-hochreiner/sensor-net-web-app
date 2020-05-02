<template>
  <v-card>
    <v-card-title>
      {{sensor.info || sensor.id}}
    </v-card-title>
    <v-card-text>
        <v-expansion-panels>
        <v-expansion-panel v-for="parameterTypeId in Object.getOwnPropertyNames(dataByParameterType).sort()" :key="parameterTypeId">
          <v-expansion-panel-header>
             <v-row no-gutters align="center">
                <v-col cols="2" align="right">
                  <v-img height="2em" width="2em" v-if="iconForParameterType(config.parameter_types[parameterTypeId].id)" :src="iconForParameterType(config.parameter_types[parameterTypeId].id)"></v-img>
                  <div v-if="!iconForParameterType(config.parameter_types[parameterTypeId].id)">{{config.parameter_types[parameterTypeId].id}}</div>
                </v-col>
                <v-col cols="5" align="right" class="value">{{Math.round(dataByParameterType[parameterTypeId][0].val)}}</v-col>
                <v-col cols="5" align="left" class="unit">{{config.parameter_types[parameterTypeId].unit}}</v-col>
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
  methods: {
    iconForParameterType(parameterTypeId) {
      return {
        'relativeHumidity': '/icon_hum.svg',
        'temperature': '/icon_temp.svg',
        'pressure': '/icon_press.svg'
      }[parameterTypeId];
    }
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
<style scoped>
  div.value {
    font-size: 1.9em;
    font-weight: 700;
  }

  div.unit {
    font-size: 1.9em;
    font-weight: 200;
  }

  .v-card__title {
    font-weight: 300;
  }
</style>

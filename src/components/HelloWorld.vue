<template>
  <v-container>
    <v-layout
      text-xs-center
      wrap
    >
      <v-flex mb-5 xs12>
        <v-progress-circular v-if="init"
          :indeterminate="true">
        </v-progress-circular>
      </v-flex>

      <v-expansion-panel v-if="!init">
        <v-expansion-panel-content
          v-for="equ in equipment"
          :key="equ._id"
        >
          <template v-slot:header>
            <v-container>
              <v-layout text-xs-center row>
                <v-flex xs12>
                  <h3>{{equ.info}}</h3>
                </v-flex>
              </v-layout>
              <v-layout row wrap>
                <v-layout xs12 md6 v-for="pt in parametertypes" :key="pt._id">
                  <v-flex xs4 md2>
                    <v-avatar v-if="pt.id == 'temperature'" :tile="true" :size="35">
                      <img src="icon_temp.svg" alt="avatar">
                    </v-avatar>
                    <v-avatar v-else-if="pt.id == 'pressure'" :tile="true" :size="35">
                      <img src="icon_press.svg" alt="avatar">
                    </v-avatar>
                    <v-avatar v-else-if="pt.id == 'humidity'" :tile="true" :size="35">
                      <img src="icon_hum.svg" alt="avatar">
                    </v-avatar>
                    <div v-else xs4 md2>{{pt.id}}</div>
                  </v-flex>
                  <v-flex xs5 md3 v-if="values[equ._id] && values[equ._id][pt._id]">{{formatNumber(values[equ._id][pt._id][values[equ._id][pt._id].length - 1].val)}}</v-flex>
                  <v-flex xs3 md1>{{pt.unit}}</v-flex>
                </v-layout>
              </v-layout>
            </v-container>
          </template>
          <v-container fluid v-for="pt in parametertypes" :key="pt._id">
            <v-sparkline v-if="values[equ._id] && values[equ._id][pt._id]"
              :value="values[equ._id][pt._id].map(function(e) { return e.val; })"
              :gradient="gradients[pt.id]"
              :smooth="10"
              :padding="8"
              :line-width="2"
              :stroke-linecap='round'
              :gradient-direction='top'
              auto-draw></v-sparkline>
          </v-container>
        </v-expansion-panel-content>
      </v-expansion-panel>
    </v-layout>
  </v-container>
</template>

<script>
  export default {
    props: ['init', 'equipment', 'parametertypes', 'parameters'],
    data: () => ({
      gradients: {
        'temperature': ['red', 'green', 'blue'],
        'pressure': ['red', 'orange', 'green'],
        'humidity': ['red', 'orange', 'green', 'orange', 'red']
      },
      values: {}
    }),
    methods: {
      'formatNumber': function (num) {
        let res = Math.round(num * 10) / 10;
        let tok = res.toString().split('.');

        if (tok.length == 1) {
          tok.push('0');
        }
        
        if (tok.length == 2) {
          while (tok[1].length < 1) {
            tok[1] += '0';
          }
        }

        return tok.join('.');
      },
      'updateValues': function() {
        let tmp = this.parameters.reduce((res, curr) => {
          if (typeof res[curr._equipment_id] === 'undefined') {
            res[curr._equipment_id] = {};
          }

          if (typeof res[curr._equipment_id][curr._parameter_type_id] === 'undefined') {
            res[curr._equipment_id][curr._parameter_type_id] = [];
          }

          res[curr._equipment_id][curr._parameter_type_id].push(curr);

          return res;
        }, {});

        for (let equid in tmp) {
          if (tmp.hasOwnProperty(equid)) {
            for (let ptid in tmp[equid]) {
              if (tmp[equid].hasOwnProperty(ptid)) {
                tmp[equid][ptid].sort((a, b) => {
                  return a.ts.localeCompare(b.ts);
                });
              }
            }
          }
        }

        this.values = tmp;
      }
    },
    beforeUpdate: function() {
      this.updateValues();
    },
    beforeMount: function() {
      this.updateValues();
    }
  }
</script>

<style>

</style>

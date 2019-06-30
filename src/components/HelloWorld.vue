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

      <v-expansion-panel>
        <v-expansion-panel-content
          v-for="equ in equipment"
          :key="equ._id"
        >
          <template v-slot:header>
            <container>
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
                  <v-flex xs5 md3>{{formatNumber(parameters.filter(e => e._equipment_id == equ._id && e._parameter_type_id == pt._id).sort((a, b) => { return a.ts.localeCompare(b.ts); })[0].val)}}</v-flex>
                  <v-flex xs3 md1>{{pt.unit}}</v-flex>
                </v-layout>
              </v-layout>
            </container>
          </template>
          <v-card>
            <v-card-text>===</v-card-text>
          </v-card>
        </v-expansion-panel-content>
      </v-expansion-panel>

    </v-layout>
  </v-container>
</template>

<script>
  export default {
    props: ['init', 'equipment', 'parametertypes', 'parameters'],
    data: () => ({}),
    methods: {
      'formatNumber': function (num) {
        let res = Math.round(num * 10) / 10;
        let tok = res.toString().split('.');

        while (tok[1].length < 2) {
          tok[1] += '0';
        }

        return tok.join('.');
      }
    }
  }
</script>

<style>

</style>

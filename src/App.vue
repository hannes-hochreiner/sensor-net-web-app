<template>
  <v-app>
    <v-toolbar app>
      <v-toolbar-title class="headline text-uppercase">
        <span>Sensor-Net</span>
        <span class="font-weight-light">Web-App</span>
      </v-toolbar-title>
    </v-toolbar>

    <v-content>
      <HelloWorld :init="init" :equipment="equipment" :parametertypes="parametertypes" :parameters="parameters"/>
    </v-content>
  </v-app>
</template>

<script>
import HelloWorld from './components/HelloWorld';
import Repo from './Repo';
import {default as axios} from 'axios';

export default {
  name: 'App',
  components: {
    HelloWorld
  },
  created: async function () {
    this.repo = new Repo(axios);
    this.equipment = await this.repo.getEquipment();
    this.parametertypes = await this.repo.getParameterTypes();
    let endTs = new Date();
    let startTs = new Date();
    startTs.setHours(endTs.getHours() - 30);
    this.parameters = await this.repo.getParameters(startTs, endTs);
    this.init = false;
    setInterval(async function() {
      let endTs = new Date();
      let startTs = new Date();
      startTs.setHours(endTs.getHours() - 30);
      this.parameters = await this.repo.getParameters(startTs, endTs);
    }.bind(this), 300000);
  },
  data () {
    return {
      init: true,
      equipment: [],
      parametertypes: [],
      parameters: []
    }
  }
}
</script>

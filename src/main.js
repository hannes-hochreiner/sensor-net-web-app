import Vue from 'vue'
import App from './App.vue'
import router from './router'
import axios from 'axios';

// Import the Auth0 configuration
// import { domain, clientId } from "../auth_config.json";

// Import the plugin here
import { Auth0Plugin } from "./auth";

console.log(process.env);

async function init() {
  const {domain, clientId} = (await axios.get('/auth_config.json')).data;
  
  // Install the authentication plugin here
  Vue.use(Auth0Plugin, {
    domain,
    clientId,
    onRedirectCallback: appState => {
      router.push(
        appState && appState.targetUrl
          ? appState.targetUrl
          : window.location.pathname
      );
    }
  });
  
  Vue.config.productionTip = false
  
  new Vue({
    router,
    render: h => h(App)
  }).$mount('#app')
}

init();

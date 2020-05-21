<template>
  <v-app>
    <v-navigation-drawer v-model="drawer" absolute temporary app>
      <v-list-item>
        <v-list-item-content>
          <v-list-item-title class="title">
            Sensor Net
          </v-list-item-title>
          <v-list-item-subtitle>
            WebApp
          </v-list-item-subtitle>
        </v-list-item-content>
      </v-list-item>

      <v-divider></v-divider>

      <v-list v-if="!$auth.loading">
        <v-list-item>
          <v-list-item-content>
            <v-btn v-if="!$auth.isAuthenticated" @click="login">Log in</v-btn>
            <v-btn v-if="$auth.isAuthenticated" @click="logout">Log out</v-btn>
          </v-list-item-content>
        </v-list-item>
      </v-list>

      <v-divider></v-divider>

      <v-list nav>
        <v-list-item link>
          <v-list-item-content>
            <v-list-item-title><router-link to="/">Home</router-link></v-list-item-title>
          </v-list-item-content>
        </v-list-item>
        <v-list-item link>
          <v-list-item-content>
            <v-list-item-title><router-link to="/about">About</router-link></v-list-item-title>
          </v-list-item-content>
        </v-list-item>
        <v-list-item link v-if="$auth.isAuthenticated">
          <v-list-item-content>
            <v-list-item-title><router-link to="/profile">Profile</router-link></v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-app-bar app>
      <v-app-bar-nav-icon @click="drawer = true"></v-app-bar-nav-icon>
      <v-toolbar-title>Sensor Net</v-toolbar-title>
    </v-app-bar>

    <!-- Sizes your content based upon application components -->
    <v-content>

      <!-- Provides the application the proper gutter -->
      <v-container fluid>

        <!-- If using vue-router -->
        <router-view></router-view>
      </v-container>
    </v-content>

    <v-footer app>
      <!-- -->
    </v-footer>
  </v-app>
</template>
<script>
export default {
  data: () => ({
    drawer: false,
  }),
  methods: {
    // Log the user in
    login() {
      this.$auth.loginWithRedirect();
    },
    // Log the user out
    logout() {
      this.$auth.logout({
        returnTo: window.location.origin
      });
    }
  }
}
</script>
<style>
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@100;400;700&display=swap');

#app {
  font-family: 'Roboto', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
</style>

<template>
  <div id="app">
    <SiteNavbar />
    <nuxt />
  </div>
</template>

<style lang="scss">
@import '@/assets/bulma-theme/theme.scss';
</style>

<script>
import SiteNavbar from '@/components/site/navbar.vue'

export default {
  components: {
    SiteNavbar
  }
}
</script>

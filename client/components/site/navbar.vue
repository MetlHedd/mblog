<template>
  <b-navbar type="is-black" wrapper-class="container">
    <template slot="brand">
      <b-navbar-item tag="router-link" :to="{ path: '/' }">
        <img src="~assets/images/logo.png" alt="To-do List" />
      </b-navbar-item>
    </template>

    <template slot="start">
      <b-navbar-item href="/">
        <span>
          Home
        </span>
      </b-navbar-item>
      <b-navbar-item href="/page/about">
        <span>
          About
        </span>
      </b-navbar-item>
      <b-navbar-item href="/page/contact">
        <span>
          Contact
        </span>
      </b-navbar-item>
    </template>

    <template slot="end">
      <b-navbar-item href="#">
        <span>
          <b-icon icon="twitter"></b-icon>
        </span>
      </b-navbar-item>
      <b-navbar-item href="#">
        <span>
          <b-icon icon="email"></b-icon>
        </span>
      </b-navbar-item>
      <b-navbar-item href="#">
        <span>
          <b-icon icon="instagram"></b-icon>
        </span>
      </b-navbar-item>
    </template>
  </b-navbar>
</template>

<script>
export default {
  name: 'SiteNavbar'
}
</script>

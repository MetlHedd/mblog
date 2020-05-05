<template>
  <div class="app">
    <section>
      <div class="column has-text-centered">
        <h1 class="title">m::blog</h1>
        <hr />
      </div>
    </section>
    <section class="container">
      <div class="card">
        <div class="card-content has-text-centered">
          <p v-if="page.title" class="title is-4 is-capitalized">
            {{ page.title }}
          </p>
          <p class="content">
            <markduck v-if="page.content" :markdown="page.content"></markduck>
          </p>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
import markduck from 'markduckjs'

export default {
  components: {
    markduck: markduck({})
  },
  data() {
    return {
      page: []
    }
  },
  mounted() {
    const pageParam = this.$route.params.slug

    this.$blogGetPage(pageParam).then((response) => {
      if (response.status === 'error') {
        return this.$nuxt.error({ statusCode: 404, message: response.content })
      } else {
        this.page = response
      }
    })
  },
  head() {
    return {
      title: 'm::blog'
    }
  }
}
</script>

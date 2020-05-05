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
        <div v-if="post.haveImage" class="card-image">
          <figure class="image is-4by3">
            <img
              :src="`http://localhost:8000/api/post/image/${post.title}`"
              alt="Placeholder image"
            />
          </figure>
        </div>
        <div class="card-content has-text-centered">
          <p v-if="post.title" class="title is-4 is-capitalized">
            {{ post.title.replace(/-/g, ' ') }}
          </p>
          <p class="subtitle is-6">On Day/Month/Year, by @author</p>
          <p class="content">
            <markduck v-if="post.content" :markdown="post.content"></markduck>
          </p>
        </div>
        <footer class="card-footer">
          <a href="#" class="card-footer-item">Share on Twitter</a>
          <a href="#" class="card-footer-item">Share on Telegram</a>
          <a href="#" class="card-footer-item">Copy Link</a>
        </footer>
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
      post: []
    }
  },
  mounted() {
    const postParam = this.$route.params.slug

    this.$blogGetPost(postParam).then((response) => {
      if (response.status === 'error') {
        return this.$nuxt.error({ statusCode: 404, message: response.content })
      } else {
        this.post = response
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

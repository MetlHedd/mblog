<template>
  <div class="app">
    <section>
      <div class="column has-text-centered">
        <h1 class="title">m::blog</h1>
      </div>
    </section>
    <section class="container">
      <div v-for="i in Math.ceil(posts.length / 3)" :key="i" class="columns">
        <div v-if="posts[(i - 1) * 3].title" class="column">
          <div class="card">
            <div v-if="posts[(i - 1) * 3].haveImage" class="card-image">
              <figure class="image is-4by3">
                <img
                  :src="
                    `
                    http://localhost:8000/api/post/image/${
                      posts[(i - 1) * 3].title
                    }
                    `
                  "
                  alt="Placeholder image"
                />
              </figure>
            </div>
            <div class="card-content has-text-centered">
              <p class="title is-4 is-capitalized">
                {{ posts[(i - 1) * 3].title.replace(/-/g, ' ') }}
              </p>
              <p class="subtitle is-6">On Day/Month/Year, by @author</p>
              <p>{{ removeMarkdown(posts[(i - 1) * 3].content) }}</p>
            </div>
            <footer class="card-footer">
              <a
                :href="`/post/${posts[(i - 1) * 3].title}`"
                class="card-footer-item"
              >
                Read More
              </a>
            </footer>
          </div>
        </div>
        <div v-if="posts[(i - 1) * 3 + 1]" class="column">
          <div class="card">
            <div v-if="posts[(i - 1) * 3 + 1].haveImage" class="card-image">
              <figure class="image is-4by3">
                <img
                  :src="
                    `
                    http://localhost:8000/api/post/image/${
                      posts[(i - 1) * 3 + 1].title
                    }
                    `
                  "
                  alt="Placeholder image"
                />
              </figure>
            </div>
            <div class="card-content has-text-centered">
              <p class="title is-4 is-capitalized">
                {{ posts[(i - 1) * 3 + 1].title.replace(/-/g, ' ') }}
              </p>
              <p class="subtitle is-6">On Day/Month/Year, by @author</p>
              <p>{{ removeMarkdown(posts[(i - 1) * 3 + 1].content) }}</p>
            </div>
            <footer class="card-footer">
              <a
                :href="`/post/${posts[(i - 1) * 3 + 1].title}`"
                class="card-footer-item"
              >
                Read More
              </a>
            </footer>
          </div>
        </div>
        <div v-if="posts[(i - 1) * 3 + 2]" class="column">
          <div class="card">
            <div v-if="posts[(i - 1) * 3 + 2].haveImage" class="card-image">
              <figure class="image is-4by3">
                <img
                  :src="
                    `
                    http://localhost:8000/api/post/image/${
                      posts[(i - 1) * 3 + 2].title
                    }
                    `
                  "
                  alt="Placeholder image"
                />
              </figure>
            </div>
            <div class="card-content has-text-centered">
              <p class="title is-4 is-capitalized">
                {{ posts[(i - 1) * 3 + 2].title.replace(/-/g, ' ') }}
              </p>
              <p class="subtitle is-6">On Day/Month/Year, by @author</p>
              <p>{{ removeMarkdown(posts[(i - 1) * 3 + 2].content) }}</p>
            </div>
            <footer class="card-footer">
              <a
                :href="`/post/${posts[(i - 1) * 3 + 2].title}`"
                class="card-footer-item"
              >
                Read More
              </a>
            </footer>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
const removeMd = require('remove-markdown')

export default {
  data() {
    return {
      posts: []
    }
  },
  mounted() {
    this.$blogGetAllPosts().then(async (response) => {
      if (response.files) {
        for (const index in response.files) {
          const element = response.files[index]

          await this.$blogGetPost(element).then((response) => {
            if (response.status !== 'error') {
              response.content = response.content.substring(0, 200) + '...'
              this.posts.push(response)
            }
          })
        }
      } else {
        return this.$nuxt.error({
          statusCode: 404,
          message: 'Could not search for posts'
        })
      }
    })
  },
  methods: {
    removeMarkdown(markdownCode) {
      return removeMd(markdownCode)
    }
  },
  head() {
    return {
      title: 'm::blog'
    }
  }
}
</script>

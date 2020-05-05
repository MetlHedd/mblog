import Vue from 'vue'
import axios from 'axios'

Vue.prototype.$blogGetPost = async (postName) => {
  return await axios
    .get(`/api/post/${postName}`)
    .catch((error) => {
      return error
    })
    .then((response) => {
      if (response.data) return response.data
      return {
        status: 'error',
        content: 'No data found from reponse'
      }
    })
}

Vue.prototype.$blogGetAllPosts = async () => {
  return await axios
    .get('/api/posts/')
    .catch((error) => {
      return error
    })
    .then((response) => {
      if (response.data) return response.data
      return {
        status: 'error',
        content: 'No data found from reponse'
      }
    })
}

Vue.prototype.$blogGetPage = async (postName) => {
  return await axios
    .get(`/api/page/${postName}`)
    .catch((error) => {
      return error
    })
    .then((response) => {
      if (response.data) return response.data
      return {
        status: 'error',
        content: 'No data found from reponse'
      }
    })
}

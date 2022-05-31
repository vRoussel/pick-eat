<template>
<nav class="pagination" role="navigation" aria-label="pagination">
  <a class="pagination-previous" :disabled="previous_page ? null : 1" @click="go_page(previous_page)">Précédent</a>
  <a class="pagination-next" :disabled="next_page ? null : 1" @click="go_page(next_page)">Suivant</a>
  <ul class="pagination-list">
    <li>
      <a class="pagination-link" :disabled="current_page < 3 ? 1 : null" @click="go_page(1)">1</a>
    </li>
    <li>
      <span class="pagination-ellipsis">&hellip;</span>
    </li>
    <li>
      <a class="pagination-link" :disabled="previous_page ? null : 1" @click="go_page(previous_page)">{{ previous_page }}</a>
    </li>
    <li>
      <a class="pagination-link is-current">{{ current_page }}</a>
    </li>
    <li>
      <a class="pagination-link" :disabled="next_page ? null : 1" @click="go_page(next_page)">{{ next_page }}</a>
    </li>
    <li>
      <span class="pagination-ellipsis">&hellip;</span>
    </li>
    <li>
      <a class="pagination-link" :disabled="current_page > max_page - 2 ? 1 : null" @click="go_page(max_page)">{{ max_page }}</a>
    </li>
  </ul>
</nav>
</template>

<script>
export default {
    name: 'pagination',
    props: {
        current_page: {
            type: Number
        },
        max_page: {
            type: Number
        },
        url_param: {
            type: String
        }

    },
    computed: {
        previous_page() {
            if (this.current_page == 1)
                return null
            else
                return this.current_page - 1
        },
        next_page() {
            if (this.current_page == this.max_page)
                return null
            else
                return this.current_page + 1
        },
    },
    methods: {
        go_page(page) {
            if (page === null) return;
            this.$router.push({ query: { ...this.$route.query, "page": page}, params: {noscroll: false} });
        },
    },
}
</script>

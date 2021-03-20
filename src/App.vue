<template>
</template>

<script>
import store from '@/store/store.js'

export default {
  name: 'App',
  components: {
  },
  data: function() {
    return {
    }
  },
  created() {
    fetch('http://127.0.0.1/api/v1/tags')
        .then(response => response.json())
        .then(json => {
            store.tags = json;
        });
    fetch('http://127.0.0.1/api/v1/categories')
        .then(response => response.json())
        .then(json => {
            store.categories = json
        });
    fetch('http://127.0.0.1/api/v1/units')
        .then(response => response.json())
        .then(json => {
            store.units = json
        });
    fetch('http://127.0.0.1/api/v1/ingredients')
        .then(response => response.json())
        .then(json => {
            store.ingredients = json
        });
    }

}
</script>

<template>
  <div class="flex my-4 mx-8 lg:mx-16 gap-x-8 gap-y-8 flex-col md:flex-row">
    <recipe-filters
      v-model:filters="filters"
      class="min-w-[13rem] xl:min-w-[16rem]"
    />
    <div
      v-if="total_count > 0"
      class="w-full"
    >
      <!--
            <p class="text-xl my-2">{{total_count}}  {{total_count > 1 ? "résultats" : "résultat"}}</p>
        -->
      <div class="p-4 gap-x-4 gap-y-6 lg:p-6 lg:gap-x-6 lg:gap-y-9 shadow-md shadow-accent rounded-md grid auto-rows-fr grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4">
        <div
          v-for="recipe in recipes"
          :key="'r' + recipe.id"
        >
          <recipe-list-item :recipe="recipe" />
        </div>
      </div>
      <div class="max-w-fit mx-auto my-8">
        <pagination
          v-model:current_page="page"
          :max_page="max_page"
        />
      </div>
    </div>
    <div v-else>
      <p class="text-xl mb-2">
        Aucune recette trouvée :(
      </p>
      <img :src="no_recipe_gif">
    </div>
  </div>
</template>

<script>
import Pagination from '@/components/Pagination.vue'
import RecipeFilters from '@/components/RecipeFilters.vue'
import RecipeListItem from '@/components/RecipeListItem.vue'
import {Filters} from '@/components/RecipeFilters.vue'

import { mapStores } from 'pinia'
import { useApiStore } from '@/store/api.js'

import no_recipe_gif from '@/assets/homer_hungry.gif'

export default {
    name: 'RecipeList',
    components: {
        Pagination,
        RecipeFilters,
        RecipeListItem
    },
    data: function() {
        return {
            recipes: [],
            per_page: 12,
            total_count: 0,
            no_recipe_gif: no_recipe_gif
        }
    },
    computed: {
        ...mapStores(useApiStore),
        from() {
            return (this.page - 1 ) * this.per_page + 1
        },
        to() {
            return this.page * this.per_page
        },
        max_page() {
            return Math.ceil(this.total_count / this.per_page) || 1
        },
        page: {
            get: function () {
                return parseInt(this.$route.query.page) || 1;
            },
            set: function (value) {
                this.$router.push({ query: { ...this.$route.query, page: value } });
            },
        },
        filters: {
            get: function () {
                let q = this.$route.query;
                return Filters(
                     q.search,
                     q.i ? q.i.split(',') : [],
                     q.t ? q.t.split(',') : [],
                     q.c ? q.c.split(',') : [],
                     q.s ? q.s.split(',') : []
                     );
            },
            set: function (f) {
                let q = {}
                if (f.search_query)
                    q.search = f.search_query
                if (f.ingredients.length > 0)
                    q.i = f.ingredients.join(',')
                if (f.tags.length > 0)
                    q.t = f.tags.join(',')
                if (f.categories.length > 0)
                    q.c = f.categories.join(',')
                if (f.seasons.length > 0)
                    q.s = f.seasons.join(',')

                this.$router.push({ name: 'recipe-list', query: q, params: {noscroll: true} });
            },
        }
    },
    watch: {
        '$route.query': {
            handler : function() {
                this.loadRecipes()
            }
        }
    },
    created() {
        this.loadRecipes()
    },
    methods: {
        loadRecipes() {
            this.apiStore.getRecipes(this.from,this.to,this.filters).then(result => {
                let [recipes, total_count] = result
                this.recipes = recipes
                this.total_count = total_count
            });
        },
        on_mobile() {
            return screen.width < 768;
        }
    }
}
</script>

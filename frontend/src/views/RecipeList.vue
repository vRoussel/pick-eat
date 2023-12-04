<template>
    <div class="flex my-4 mx-4 lg:mx-8 gap-x-8 gap-y-8 flex-col md:flex-row">
        <recipe-filters v-model:filters="filters" class="md:min-w-[16rem] md:max-w-[16rem]" />
        <div v-if="total_count > 0" class="w-full">
            <!--
            <p class="text-xl my-2">{{total_count}}  {{total_count > 1 ? "résultats" : "résultat"}}</p>
        -->
            <div
                class="p-4 gap-x-4 gap-y-6 lg:p-6 lg:gap-x-6 lg:gap-y-9 shadow-md shadow-accent rounded-md grid auto-rows-fr grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4"
            >
                <div v-for="(recipe, idx) in recipes" :key="'r' + recipe.id">
                    <recipe-list-item :recipe="recipe" :lazy="idx >= 2 && this.on_mobile()" />
                </div>
            </div>
            <div class="max-w-fit mx-auto my-8">
                <pagination v-model:current_page="page" :max_page="max_page" />
            </div>
        </div>
        <div v-else-if="total_count == 0">
            <p class="text-xl mb-2">Aucune recette trouvée :(</p>
            <video :src="no_recipe_video" autoplay loop muted playsinline />
        </div>
    </div>
</template>

<script>
import isEqual from 'lodash.isequal'
import Pagination from '@/components/Pagination.vue'
import RecipeFilters from '@/components/RecipeFilters.vue'
import RecipeListItem from '@/components/RecipeListItem.vue'
import { Filters } from '@/components/RecipeFilters.vue'

import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'

import no_recipe_video from '@/assets/homer_hungry.mp4'

export default {
    name: 'RecipeList',
    components: {
        Pagination,
        RecipeFilters,
        RecipeListItem,
    },
    data: function () {
        return {
            recipes: [],
            per_page: 12,
            total_count: null,
            no_recipe_video: no_recipe_video,
        }
    },
    computed: {
        ...mapStores(useFoodStore),
        from() {
            return (this.page - 1) * this.per_page + 1
        },
        to() {
            return this.page * this.per_page
        },
        max_page() {
            return Math.ceil(this.total_count / this.per_page) || 1
        },
        page: {
            get: function () {
                return parseInt(this.$route.query.page) || 1
            },
            set: function (value) {
                this.$router.push({ query: { ...this.$route.query, page: value, ns: undefined } })
            },
        },
        filters: {
            get: function () {
                let q = this.$route.query
                return Filters(
                    q.search,
                    q.i ? q.i.split(',') : [],
                    q.t ? q.t.split(',') : [],
                    q.c ? q.c.split(',') : [],
                    q.s ? q.s.split(',') : [],
                    q.a,
                    q.d ? q.d.split(',') : [],
                )
            },
            set: function (f) {
                let q = {}
                if (f.search_query) q.search = f.search_query
                if (f.ingredients.length > 0) q.i = f.ingredients.join(',')
                if (f.tags.length > 0) q.t = f.tags.join(',')
                if (f.categories.length > 0) q.c = f.categories.join(',')
                if (f.seasons.length > 0) q.s = f.seasons.join(',')
                if (f.account) q.a = f.account
                if (f.diets.length > 0) q.d = f.diets.join(',')

                // Avoid scrolling to top if we are changing filters
                q.ns = null
                this.$router.push({ name: 'recipe-list', query: q })
            },
        },
    },
    watch: {
        '$route.query': {
            handler: function () {
                // If saved_query exists, it means it hasn't been restore yet
                // It will be soon so there's no point to call loadRecipes before that
                if (this.$route.name == 'recipe-list' && this.saved_query == null) {
                    this.loadRecipes()
                }
            },
        },
    },
    created() {
        this.loadRecipes()
    },
    deactivated() {
        this.saved_query = this.last_query
    },
    async activated() {
        if (this.saved_query != null) {
            // Only restore saved_query if the current_one  has no query params
            if (
                Object.keys(this.$route.query).length == 0 &&
                !isEqual(this.saved_query, this.$route.query)
            ) {
                await this.$router.replace({ query: this.saved_query })
            }
            this.loadRecipes()
            this.saved_query = null
        }
    },
    methods: {
        loadRecipes() {
            this.foodStore.getRecipes(this.from, this.to, this.filters).then((result) => {
                let [recipes, total_count] = result
                this.recipes = recipes
                this.total_count = total_count
            })
            this.last_query = this.$route.query
        },
        on_mobile() {
            return window.innerWidth < 768
        },
    },
}
</script>

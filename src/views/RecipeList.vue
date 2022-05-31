<template>
    <div class="container my-4 px-4">
        <div class="columns">
            <div class="side column is-one-fifth-fullhd is-3-desktop is-4-tablet">
                <recipe-filters :initial_filters="this.url_filters" @search="runSearch"></recipe-filters>
            </div>
            <div class="column columns is-multiline is-align-content-flex-start">
                <div class="column is-full is-mobile">
                    <p class="is-size-4">{{total_count}}  {{total_count > 1 ? "résultats" : "résultat"}}</p>
                </div>
                <div class="column is-3-fullhd is-4-desktop is-6-tablet mb-4" v-for="recipe in recipes" :key="recipe.id">
                    <recipe-list-item :recipe=recipe></recipe-list-item>
                </div>
            </div>
        </div>
    </div>
    <pagination :current_page="page" :max_page="max_page" url_param="page"></pagination>
</template>

<script>
import Pagination from '@/components/Pagination.vue'
import RecipeFilters from '@/components/RecipeFilters.vue'
import RecipeListItem from '@/components/RecipeListItem.vue'

export default {
    name: 'recipe-list',
    inject: ["store"],
    components: {
        Pagination,
        RecipeFilters,
        RecipeListItem
    },
    props: {
        page: {
            type: Number,
            default: 1
        },
        url_filters: {
            type: Object,
            default: null
        }
    },
    data: function() {
        return {
            recipes: [],
            per_page: 20,
            total_count: 0,
        }
    },
    methods: {
        loadRecipes() {
            this.store.getRecipes(this.from,this.to,this.url_filters).then(result => {
                let [recipes, total_count] = result
                this.recipes = recipes
                this.total_count = total_count
            });
        },
        runSearch(filters) {
            let query = {}
            let f = filters
            if (f.search_query)
                query.search = f.search_query
            if (f.ingredients.length > 0)
                query.i = f.ingredients.join(',')
            if (f.tags.length > 0)
                query.t = f.tags.join(',')
            if (f.categories.length > 0)
                query.c = f.categories.join(',')
            if (f.seasons.length > 0)
                query.s = f.seasons.join(',')

            this.$router.replace({ name: 'recipe-list', query: query, params: {noscroll: true} });
        }
    },
    created() {
        this.loadRecipes()
    },
    computed: {
        from() {
            return (this.page - 1 ) * this.per_page + 1
        },
        to() {
            return this.page * this.per_page
        },
        max_page() {
            return Math.ceil(this.total_count / this.per_page) || 1
        },
        url_filters_json() {
            return JSON.stringify(this.url_filters)
        }
    },
    watch: {
        page: function() {
            //this.input_search = this.url_search
            this.loadRecipes()
        },
        url_filters_json : function() {
            this.loadRecipes()
        },
    },
}
</script>

<style lang="scss" scoped>
    @media screen and (max-width: 768px), print {
        .is-expanded-mobile {
            display: flex;
            flex-grow: 1;
        }
    }
</style>

<template>
    <div class="container my-4 px-4">
        <div class="columns">
            <div class="side column is-one-fifth-fullhd is-3-desktop is-4-tablet">
                <recipe-filters v-model:filters="filters"></recipe-filters>
            </div>
            <div class="column columns is-multiline is-align-content-flex-start">
                <div class="column is-full">
                    <p class="is-size-4 is-size-5-mobile">{{total_count}}  {{total_count > 1 ? "résultats" : "résultat"}}</p>
                </div>
                <div class="column is-full" v-if="this.total_count > 0">
                    <pagination :hide_previous_next="on_mobile()" :current_page="page" :max_page="max_page" url_param="page" ></pagination>
                </div>
                <div class="column is-3-fullhd is-4-desktop is-6-tablet mb-4" v-for="recipe in recipes" :key="recipe.id">
                    <recipe-list-item :recipe=recipe></recipe-list-item>
                </div>
                <div class="column is-full" v-if="this.total_count > 0">
                    <pagination :hide_previous_next="on_mobile()" :current_page="page" :max_page="max_page" url_param="page"></pagination>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import Pagination from '@/components/Pagination.vue'
import RecipeFilters from '@/components/RecipeFilters.vue'
import RecipeListItem from '@/components/RecipeListItem.vue'
import {Filters} from '@/components/RecipeFilters.vue'

export default {
    name: 'recipe-list',
    inject: ["store"],
    components: {
        Pagination,
        RecipeFilters,
        RecipeListItem
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
            this.store.getRecipes(this.from,this.to,this.filters).then(result => {
                let [recipes, total_count] = result
                this.recipes = recipes
                this.total_count = total_count
            });
        },
        on_mobile() {
            return screen.width < 768;
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
                console.log("get_filters")
                let q = this.$route.query;
                return new Filters(
                     q.search,
                     q.i ? q.i.split(',') : [],
                     q.t ? q.t.split(',') : [],
                     q.c ? q.c.split(',') : [],
                     q.s ? q.s.split(',') : []
                     );
            },
            set: function (f) {
                console.log("set_filters")
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

                this.$router.replace({ name: 'recipe-list', query: q, params: {noscroll: true} });
            },
        }
    },
    watch: {
        page: function() {
            //this.input_search = this.url_search
            this.loadRecipes()
        },
        filters : function() {
            this.loadRecipes()
        },
    }
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

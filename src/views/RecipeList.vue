<template>
    <div class="container my-4">
        <div class="columns is-desktop">
            <div class="column is-one-fifth-fullhd is-3-desktop">
                <live-search :initial_query="this.url_search" @search="runSearch" @clear="clearSearch"></live-search>
                <div class="field">
                    <label class="label">Ingrédients</label>
                        <Multiselect mode="tags" :options="this.store.state.ingredients" label="name" searchable :strict="false" trackBy="name" object valueProp="id" ref="multiselect" :closeOnSelect="false"/>
                </div>
                <div class="field">
                    <label class="label">Tags</label>
                <Multiselect mode="tags" :options="this.store.state.tags" label="name" searchable :strict="false" trackBy="name" object valueProp="id" ref="multiselect" :closeOnSelect="false"/>
                </div>
                <fieldset class="block">
                    <legend class="label">Saisons</legend>
                    <div class="control" v-for="s in this.store.state.seasons" :key="s.id">
                        <label class="checkbox">
                            <input type="checkbox">
                            {{ s.name }}
                        </label>
                    </div>
                </fieldset>
                <fieldset class="block">
                    <legend class="label">Catégories</legend>
                    <div class="control" v-for="c in this.store.state.categories" :key="c.id">
                        <label class="checkbox">
                            <input type="checkbox">
                            {{ c.name }}
                        </label>
                    </div>
                </fieldset>
            </div>
            <div class="column columns is-multiline">
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
import LiveSearch from '@/components/LiveSearch.vue'
import Multiselect from '@vueform/multiselect'
import RecipeListItem from '@/components/RecipeListItem.vue'

export default {
    name: 'recipe-list',
    inject: ["store"],
    components: {
        Pagination,
        LiveSearch,
        Multiselect,
        RecipeListItem
    },
    props: {
        page: {
            type: Number,
            default: 1
        },
        url_search: {
            type: String,
            default: null
        },
    },
    data: function() {
        return {
            recipes: [],
            per_page: 20,
            max_page: 1,
        }
    },
    methods: {
        loadRecipes() {
            this.store.getRecipes(this.from,this.to,this.url_search).then(result => {
                let [recipes, total_count] = result
                this.recipes = recipes
                this.max_page = Math.ceil(total_count / this.per_page) || 1
            });
        },
        runSearch(q) {
            this.$router.push({ name: 'recipe-list', query: { 'search': q } });
        },
        clearSearch() {
            this.$router.push({ name: 'recipe-list'});
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
        }
    },
    watch: {
        page: function() {
            this.input_search = this.url_search
            this.loadRecipes()
        },
        url_search : function() {
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

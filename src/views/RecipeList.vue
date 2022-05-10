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
                    <div class="card">
                        <div class="card-image">
                            <figure class="image is-square is-fullwidth">
                                    <img :src="recipe.image.replace('upload', 'upload/c_limit,h_512,w_limit,w_512') || PLACEHOLDER_IMG" class="is-clickable" @click="openRecipe(recipe.id)"/>
                            </figure>
                        </div>
                        <header class="card-header">
                            <p class="recipe-name card-header-title is-size-4-mobile
                            is-size-5-tablet" v-tooltip="recipe.name">
                                {{ recipe.name }}
                            </p>
                            <div class="card-header-icon" aria-label="favorite">
                              <span class="icon">
                                <i :class="recipe.is_favorite ? 'fa' : 'far'" class="fa-heart is-size-4-mobile is-size-5-tablet is-clickable" @click="toggleFavorite(recipe)"></i>
                              </span>
                            </div>
                        </header>
                    </div>
                </div>
            </div>
        </div>
    </div>
    <pagination :current_page="page" :max_page="max_page" url_param="page"></pagination>
</template>

<script>
import Pagination from '@/components/Pagination.vue'
import LiveSearch from '@/components/LiveSearch.vue'
import {PLACEHOLDER_IMG} from '@/utils/utils.js'
import Multiselect from '@vueform/multiselect'

export default {
    name: 'recipe-list',
    inject: ["store"],
    components: {
        Pagination,
        LiveSearch,
        Multiselect
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
        toggleFavorite(recipe) {
            this.store.toggleFavorite(recipe)
        },
        loadRecipes() {
            this.store.getRecipes(this.from,this.to,this.url_search).then(result => {
                let [recipes, total_count] = result
                this.recipes = recipes
                this.max_page = Math.ceil(total_count / this.per_page) || 1
            });
        },
        openRecipe(id) {
            this.$router.push({ name: 'recipe', params: { id } })
        },
        runSearch(q) {
            this.$router.push({ name: 'recipe-list', query: { 'search': q } });
        },
        clearSearch() {
            this.$router.push({ name: 'recipe-list'});
        }
    },
    created() {
        this.PLACEHOLDER_IMG = PLACEHOLDER_IMG
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
    .recipe-name {
        font-family: "Rounded_Elegance";
        padding: 0;
        margin: $card-header-padding;

        overflow-wrap: anywhere;
        -webkit-line-clamp: 3;
        display: -webkit-box;
        -webkit-box-orient: vertical;
        overflow: hidden;
        height: $body-line-height * $body-font-size * 3;
    }

    .fa-heart {
        color: red;
    }
    .card-header-icon {
        cursor: auto;
    }

    @media screen and (max-width: 768px), print {
        .is-expanded-mobile {
            display: flex;
            flex-grow: 1;
        }
    }
</style>

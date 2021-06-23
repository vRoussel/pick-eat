<template>
    <div class="container my-4 px-4">
        <div class="columns is-multiline mt-4">
            <div class="column is-one-fifth-fullhd is-3-desktop is-4-tablet my-4" v-for="recipe in recipes" :key="recipe.id">
                <div class="card">
                    <div class="card-image">
                        <figure class="image is-512x512">
                                <img :src="recipe.image.replace('upload', 'upload/c_limit,h_512,w_limit,w_512')"/>
                        </figure>
                    </div>
                    <header class="card-header">
                        <p class="card-header-title is-size-4-mobile is-size-5-tablet">
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
    <pagination :current_page="page" :max_page="max_page" base_url="/recipes" url_param="page"></pagination>
</template>

<script>
import Pagination from '@/components/Pagination.vue'

export default {
    name: 'recipe-list',
    inject: ["store"],
    components: {
        Pagination
    },
    props: {
        page: {
            type: Number,
            default: 1
        }
    },
    data: function() {
        return {
            recipes: [],
            per_page: 4,
            max_page: 1
        }
    },
    methods: {
        getRecipes(from,to) {
            this.store.getRecipes(from,to).then(result => {
                let [recipes, total_count] = result
                this.recipes = recipes
                this.max_page = Math.ceil(total_count / this.per_page)
            });
        },
        toggleFavorite(recipe) {
            this.store.toggleFavorite(recipe)
        },
        loadRecipes() {
            let from = (this.page - 1 ) * this.per_page + 1
            let to = this.page * this.per_page
            this.getRecipes(from,to)
        }
    },
    mounted() {
        this.loadRecipes()
    },
    watch: {
        page: function(val) {
            this.$route.query.page = val
            this.loadRecipes()
        }
    }
}
</script>

<style>
    .recipe-name {
        font-size: 1em;
        font-family: "Rounded_Elegance";
    }

    .fa-heart {
        color: red;
    }
    .card-header-icon {
        cursor: auto;
    }
    .card {
        display:flex;
        height: 100%;
        flex-direction: column;
    }
    .card-header {
        flex-grow: 1;
    }
</style>

<template>
    <div class="container my-4 px-4">
        <div class="field has-addons">
            <div class="control has-icons-left has-icons-right is-expanded-mobile">
                <input @input="e => input_search = e.target.value" class="input is-rounded" type="text" placeholder="Trouver une recette" :value="input_search">
                <span class="icon is-right" v-if="this.input_search">
                   <i class="fas fa-times is-clickable" @click="clearSearch()"></i>
                </span>
                <span class="icon is-left">
                   <i class="fas fa-search"></i>
                </span>
            </div>
        </div>
        <div class="columns is-multiline mt-4">
            <div class="column is-one-fifth-fullhd is-3-desktop is-4-tablet my-4" v-for="recipe in recipes" :key="recipe.id">
                <div class="card">
                    <div class="card-image">
                        <figure class="image is-square is-fullwidth">
                                <img :src="recipe.image.replace('upload', 'upload/c_limit,h_512,w_limit,w_512')" class="is-clickable" @click="openRecipe(recipe.id)"/>
                        </figure>
                    </div>
                    <header class="card-header">
                        <p class="recipe-name card-header-title is-size-4-mobile is-size-5-tablet">
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
    <pagination :current_page="page" :max_page="max_page" url_param="page"></pagination>
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
            input_search : this.url_search,
            timer: null
        }
    },
    methods: {
        toggleFavorite(recipe) {
            this.store.toggleFavorite(recipe)
        },
        loadRecipes() {
            this.store.getRecipes(this.from,this.to,this.input_search).then(result => {
                let [recipes, total_count] = result
                this.recipes = recipes
                this.max_page = Math.ceil(total_count / this.per_page) || 1
            });
        },
        openRecipe(id) {
            this.$router.push({ name: 'recipe', params: { id } })
        },
        runSearch(query) {
            if (this.timer) {
                clearTimeout(this.timer)
                this.timer = null
            }
            this.timer = setTimeout(() => {
                if (query == null || query == "") {
                    this.$router.push({ name: 'recipe-list'});
                } else {
                    this.$router.push({ name: 'recipe-list', query: { 'search': query } });
                }
            }, 400)
        },
        clearSearch() {
            this.input_search = null
            this.$router.push({ name: 'recipe-list' });
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
            //this.input_search = val
            this.loadRecipes()
        },
        input_search : function(val) {
            this.runSearch(val)
        }
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
    }

    .card-header:hover p {
        -webkit-line-clamp: none;
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

    @media screen and (max-width: 768px), print {
        .is-expanded-mobile {
            display: flex;
            flex-grow: 1;
        }
    }
</style>

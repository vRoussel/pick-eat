<template>
        <div class="container is-max-desktop px-4" v-if="recipe">
           <div class="box">
                <div class="columns is-mobile">
                    <div class="column is-4-tablet is-6-mobile">
                        <figure class="image is-256x256">
                                <img :src="recipe.image.replace('upload', 'upload/c_limit,h_512,w_limit,w_512')"/>
                        </figure>
                    </div>
                    <div class="column">
                        <p class="is-size-4-mobile is-size-2-tablet">{{ recipe.name }}</p>
                        <br>
                        <p class="is-size-6-mobile is-size-4-tablet">
                        <span class="icon"><i class="fas fa-clock"></i></span> {{ recipe.prep_time_min }} min
                        <br>
                        <span class="icon"><i class="fas fa-fire"></i></span> {{ recipe.cook_time_min }} min
                        </p>
                    </div>
                </div>
                <div class="columns is-centered ">
                    <div class="column is-4-tablet">
                        <table class="table is-fullwidth">
                            <thead>
                                <tr class="has-text-centered"><th colspan="2">Ingrédients</th></tr>
                            </thead>
                            <tbody>
                                <tr v-for="ingr in recipe.q_ingredients" :key="ingr.id">
                                    <td class="has-text-right">{{ ingr.quantity }} {{ ingr.unit ? ingr.unit.short_name : "" }}</td>
                                    <td>{{ ingr.name }}</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    <div class="column">
                        <table class="table">
                            <thead>
                                <tr class="has-text-centered"><th colspan="2">Etapes</th></tr>
                            </thead>
                            <tbody>
                                <tr v-for="(step,index) in recipe.instructions" :key="index">
                                    <td>{{ index + 1 }}</td>
                                    <td>{{ step }}</td>
                                </tr>
                            </tbody>
                        </table>

                    </div>
                </div>
            </div>
        </div>
    <!--
            <div class="columns is-multiline mt-4">
                <div class="column is-3-desktop is-4-tablet" v-for="recipe in recipes" :key="recipe.id">
                <div class="card">
                    <div class="card-image">
                        <figure class="image is-512x512">
                                <img :src="recipe.image.replace('upload', 'upload/c_limit,h_512,w_limit,w_512')"/>
                        </figure>
                    </div>
                    <header class="card-header">
                        <p class="card-header-title recipe-name">
                            {{ recipe.name }}
                        </p>
                        <div class="card-header-icon" aria-label="favorite">
                          <span class="icon">
                            <i :class="recipe.is_favorite ? 'fa' : 'far'" class="fa-heart is-clickable" @click="toggleFavorite(recipe)"></i>
                          </span>
                        </div>
                    </header>
                </div>
            </div>
    </div>
    -->
</template>

<script>
export default {
    name: 'recipe',
    inject: ["store"],
    props: {
        id: {
            type: Number,
        }
    },
    data: function() {
        return {
            recipe: null,
        }
    },
    methods: {
        toggleFavorite(recipe) {
            this.store.toggleFavorite(recipe)
        },
        loadRecipe() {
            this.store.getOneRecipe(this.id).then(result => {
                this.recipe = result
                document.title = this.recipe.name + ' - PickEat'
            });
        }
    },
    created() {
        this.loadRecipe()
    },
    //watch: {
    //    page: function(val) {
    //        this.$route.query.page = val
    //        this.loadRecipes()
    //    }
    //}
}
</script>

<style>
    .recipe-name {
        font-size: 2em;
        font-family: "Rounded_Elegance";
    }

    .fa-heart {
        color: red;
    }
    .card-header-icon {
        cursor: auto;
    }
</style>

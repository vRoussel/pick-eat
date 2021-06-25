<template>
   <div class="box" v-if="recipe">
        <div class="columns is-mobile">
            <div class="column is-4-tablet is-6-mobile">
                <figure class="image">
                        <img :src="recipe.image.replace('upload', 'upload/c_limit,h_512,w_limit,w_512')"/>
                </figure>
            </div>
            <div class="column">
                <div class="columns is-mobile">
                    <div class="column">
                        <span class="recipe-name is-size-4-mobile is-size-2-tablet">{{ recipe.name }}</span>
                    </div>
                    <div class="column is-narrow">
                        <span class="icon">
                          <i class="fa fa-pencil-alt is-size-5-mobile is-size-3-tablet is-clickable" @click="editRecipe()"></i>
                        </span>
                    </div>
                </div>
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
</template>

<script>
export default {
    name: 'recipe-view',
    inject: ["store"],
    props: {
        recipe: {
            type : Object,
        }
    },
    methods: {
        editRecipe() {
            this.$emit('edit')
            console.log('hello there1')
        }
    },
    emits: ['edit']
}
</script>

<style>
    .recipe-name {
        font-family: "Rounded_Elegance";
    }

    .fa-heart {
        color: red;
    }
</style>

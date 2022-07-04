<template>
   <div class="box" v-if="recipe">
        <div class="has-text-right">
            <span class="icon">
              <i class="primary_icon fa fa-pencil-alt is-size-6-mobile is-size-5-tablet is-clickable" @click="editRecipe()"></i>
            </span>
        </div>
        <div class="columns is-mobile has-text-centered my-0">
            <div class="column is-6-mobile is-4-tablet">
                <figure class="image">
                    <img :src="image" width="512"/>
                </figure>
            </div>
            <div class="column is-flex is-flex-direction-column is-justify-content-space-evenly" id="recipe-name-column">
                <p ref="recipe_name" v-tooltip="overflown ? recipe.name : null" class="recipe-name is-size-5-mobile is-size-2-tablet">{{ recipe.name }}</p>
                <season-icons class="is-size-4-mobile is-size-3-tablet" :seasons="this.recipe.seasons"></season-icons>
                <p class="is-size-6-mobile is-size-5-tablet">
                <span class="icon"><i class="primary_icon fas fa-clock"></i></span> {{ recipe.prep_time_min }} min
                <br/>
                <span class="icon"><i class="primary_icon fas fa-fire"></i></span> {{ recipe.cook_time_min }} min
                </p>
            </div>
        </div>
        <div class="tags is-centered mt-4 mb-0">
            <span class="tag is-medium is-rounded is-primary is-light" v-for="tag in recipe.tags" :key="tag.id">
                {{ tag.name }}
            </span>
        </div>
        <div class="columns is-centered mt-0">
            <div class="column is-5-tablet is-4-desktop">
                <table class="table is-fullwidth">
                    <thead>
                        <tr class="has-text-centered is-size-4-tablet is-size-5-mobile">
                            <th v-if="recipe.n_shares > 0" colspan="2" rowspan="2">Ingrédients ({{ recipe.n_shares }} <span class="icon"><i class="fas fa-cookie-bite"></i></span>)</th>
                            <th v-else colspan="2" rowspan="2">Ingrédients</th>
                        </tr>
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
                        <tr class="has-text-centered is-size-4-tablet is-size-5-mobile"><th colspan="2">Étapes</th></tr>
                    </thead>
                    <tbody>
                        <tr v-for="(step,index) in recipe.instructions" :key="index">
                            <td class="instruction_index">{{ index + 1 }}</td>
                            <td>{{ step }}</td>
                        </tr>
                    </tbody>
                </table>

            </div>
        </div>
    </div>
</template>

<script>
import SeasonIcons from '@/components/SeasonIcons.vue'
import {PLACEHOLDER_IMG, isOverflown} from '@/utils/utils.js'

export default {
    name: 'recipe-view',
    components: {
        SeasonIcons,
    },
    inject: ["store"],
    props: {
        recipe: {
            type : Object,
        }
    },
    data: function() {
        return {
            overflown: false,
        }
    },
    methods: {
        editRecipe() {
            this.$emit('edit')
        }
    },
    computed : {
        image() {
            if (this.recipe.image === "")
                return PLACEHOLDER_IMG
            else
                return this.recipe.image.replace("/upload", "/upload/c_limit,h_512,w_512");
        }
    },
    mounted() {
        //https://jefrydco.id/en/blog/safe-access-vue-refs-undefined
        const interval = setInterval(() => {
            if (this.$refs.recipe_name) {
                this.overflown = isOverflown(this.$refs.recipe_name)
                clearInterval(interval)
            }
        }, 100)
    },
    emits: ['edit']
}
</script>

<style lang="scss" scoped>
    th, .recipe-name {
        font-family: "Rounded_Elegance";
        font-weight: bold;
    }

    th, .primary_icon, .instruction_index {
        color: $primary !important;
    }

    .recipe-name {
        overflow-wrap: anywhere;
        -webkit-line-clamp: 2;
        display: -webkit-box;
        -webkit-box-orient: vertical;
        overflow: hidden;
        height: $body-line-height * $body-font-size * 2;
    }

    .fa-heart {
        color: red;
    }

    #recipe-name-column {
         @include until($tablet) {
            padding-left: 0;
            padding-right: 0;
         }
    }
</style>
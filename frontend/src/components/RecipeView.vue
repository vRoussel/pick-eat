<template>
<!--
   <div class="box" v-if="recipe">
        <div class="has-text-right">
            <span class="icon">
              <i class="primary_icon fa fa-pencil-alt is-size-6-mobile is-size-5-tablet is-clickable" @click="editRecipe()"></i>
            </span>
        </div>
-->
<div v-if="recipe" class="flex flex-col w-full max-w-5xl mx-auto p-2 sm:p-4 md:p-6 gap-y-4 sm:gap-y-6 shadow-secondary shadow-md rounded-xl relative">
    <span class="icon absolute right-2 top-2 text-xl sm:right-4 sm:top-4 sm:text-2xl md:right-6 md:top-6 md:text-3xl">
      <ion-icon class="text-primary cursor-pointer" name="create-outline" @click="editRecipe()"></ion-icon>
    </span>
    <div class="flex gap-2 sm:gap-x-4 md:gap-x-6">
        <div class="basis-1/2 sm:basis-2/5 md:basis-1/3">
            <img :src="image" class="rounded-xl"/>
        </div>
        <div class="flex flex-col basis-1/2 justify-evenly items-center mx-auto">
            <p ref="recipe_name" v-tooltip="overflown ? recipe.name : null" class="recipe-name text-primary text-center text-lg sm:text-3xl md:text-4xl lg:text-5xl">{{ recipe.name }}</p>
            <season-icons :seasons="this.recipe.seasons" class="text-xl sm:text-2xl md:text-3xl lg:text-4xl gap-x-1"></season-icons>
            <p class="text-3xl">
                <span class="inline-flex items-center gap-x-1 text-base sm:text-lg md:text-xl lg:text-2xl"><ion-icon name="time" class="text-primary"></ion-icon> {{ recipe.prep_time_min }} min</span>
                <br/>
                <span class="inline-flex items-center gap-x-1 text-base sm:text-lg md:text-xl lg:text-2xl"><ion-icon name="flame" class="text-primary" ></ion-icon>{{ recipe.cook_time_min }} min</span>
            </p>
        </div>
    </div>
    <div class="flex gap-2 flex-wrap justify-center">
        <span class="badge badge-outline badge-primary badge-md sm:badge-lg" v-for="tag in recipe.tags" :key="tag.id">
            {{ tag.name }}
        </span>
    </div>
    <div class="flex flex-wrap sm:flex-nowrap gap-x-4 sm:gap-x-6 md:gap-x-8 items-start space-y-8 sm:space-y-0">
        <table class="table table-compact basis-2/5 md:basis-1/3 shrink-0 grow sm:grow-0">
            <thead>
                <tr class="text-center">
                    <th v-if="recipe.n_shares > 0" colspan="2" class="bg-transparent !text-primary"><span class="inline-flex items-center text-lg" >Ingrédients ({{ recipe.n_shares }} <ion-icon class="pl-0.5" name="person"></ion-icon>)</span></th>
                    <th v-else colspan="2">Ingrédients</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="ingr in recipe.q_ingredients" :key="ingr.id">
                    <td class="!text-right">{{ ingr.quantity }} {{ ingr.unit ? ingr.unit.short_name : "" }}</td>
                    <td>{{ ingr.name }}</td>
                </tr>
            </tbody>
        </table>
        <table class="table table-compact grow">
            <thead class="border-primary">
                <tr class="text-center"><th class="bg-transparent !text-primary font-bold text-lg" colspan="2">Étapes</th></tr>
            </thead>
            <tbody>
                <tr class="border-0" v-for="(step,index) in recipe.instructions" :key="index">
                    <td class="text-primary border-0 font-bold">{{ index + 1 }}</td>
                    <td class="whitespace-pre-wrap border-0 !align-middle">{{ step }}</td>
                </tr>
            </tbody>
        </table>
    </div>
</div>
<!--
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
-->
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
            if (this.recipe == null || this.recipe.image === "")
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

<style scoped>
    th, .recipe-name {
        font-family: "Rounded_Elegance";
        font-weight: bold;
    }

    .recipe-name {
        overflow-wrap: anywhere;
        -webkit-line-clamp: 2;
        display: -webkit-box;
        -webkit-box-orient: vertical;
        overflow: hidden;
    }
</style>

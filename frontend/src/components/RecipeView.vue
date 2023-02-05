<template>
  <!--
   <div class="box" v-if="recipe">
        <div class="has-text-right">
            <span class="icon">
              <i class="primary_icon fa fa-pencil-alt is-size-6-mobile is-size-5-tablet is-clickable" @click="editRecipe()"></i>
            </span>
        </div>
-->
  <div
    v-if="recipe"
    class="flex flex-col w-full max-w-5xl mx-auto p-4 md:p-6 gap-y-8 border border-primary rounded-xl relative"
  >
    <span class="icon absolute right-1 top-1 text-xl sm:right-4 sm:top-4 sm:text-2xl md:right-6 md:top-6 md:text-3xl">
      <Icon
        class="text-primary cursor-pointer"
        :icon="icons.pencil"
        @click="editRecipe()"
      />
    </span>
    <div class="flex flex-wrap sm:flex-nowrap gap-y-6 gap-x-4 md:gap-x-6">
      <div class="basis-full sm:basis-2/5 md:basis-1/3 p-2">
        <img
          :src="image"
          class="rounded-xl"
        >
      </div>
      <div class="flex flex-col basis-full gap-y-2 sm:basis-1/2 justify-around items-center mx-auto">
        <p
          ref="recipe_name"
          v-tooltip="overflown ? recipe.name : null"
          class="recipe-name text-primary text-center font-bold text-xl sm:text-3xl md:text-4xl lg:text-5xl"
        >
          {{ recipe.name }}
        </p>
        <season-icons
          :seasons="recipe.seasons"
          class="text-2xl md:text-3xl lg:text-4xl gap-x-1"
        />
        <p class="space-x-4">
          <span class="icon inline-flex items-center gap-x-1 text-sm sm:text-lg md:text-xl lg:text-2xl"><Icon
            :icon="icons.knife"
            :rotate="3"
            class="text-primary"
          /> {{ recipe.prep_time_min }} min</span>
          <span class="icon inline-flex items-center gap-x-1 text-sm sm:text-lg md:text-xl lg:text-2xl"><Icon
            :icon="icons.cooking_pot"
            class="text-primary"
          />{{ recipe.cook_time_min }} min</span>
        </p>
        <p class="text-xs sm:text-sm italic">Ajoutée par <router-link class="link-primary" :to="'/recipes?a=' + recipe.author.id">{{ recipe.author.display_name }}</router-link></p>
      </div>
    </div>
    <div class="flex gap-2 flex-wrap justify-center mb-2 sm:mb-4">
      <router-link
        v-for="tag in recipe.tags"
        :key="tag.id"
        :to="'/recipes?t=' + tag.id"
        class="badge badge-outline badge-primary badge-md sm:badge-lg"
      >
        {{ tag.name }}
      </router-link>
    </div>
    <div class="flex flex-wrap sm:flex-nowrap gap-x-4 sm:gap-x-6 md:gap-x-8 items-start space-y-8 sm:space-y-0">
      <table class="table table-compact basis-2/5 md:basis-1/3 shrink-0 grow sm:grow-0">
        <thead>
          <tr class="text-center">
            <th
              colspan="2"
              class="text-primary-content !bg-primary text-lg"
            >
              <span v-if="recipe.n_shares > 0" class="icon inline-flex items-center">Ingrédients ({{ recipe.n_shares }} <Icon
                class="pl-0.5"
                :icon="icons.person"
              />)</span>
              <span v-else>Ingrédients</span>
            </th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="ingr in recipe.q_ingredients"
            :key="ingr.id"
            class="border-b"
          >
            <td class="!text-right">
              {{ ingr.quantity }} {{ ingr.unit ? ingr.unit.short_name : "" }}
            </td>
            <td>{{ ingr.name }}</td>
          </tr>
        </tbody>
      </table>
      <table class="table table-compact table-fixed grow w-full">
      <colgroup>
        <col class="w-8" />
        <col/>
      </colgroup>
        <thead>
          <tr class="text-center border-b border-primary">
            <th
              class="bg-transparent !text-primary text-lg"
              colspan="2"
            >
              Étapes
            </th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(step,index) in recipe.instructions"
            :key="index"
            class="border-0"
          >
            <td class="text-primary border-0 font-bold w-8">
              {{ index + 1 }}
            </td>
            <td class="whitespace-pre-wrap break-words border-0 !align-middle">
              {{ step }}
            </td>
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
import {isOverflown} from '@/utils/utils.js'

export default {
    name: 'RecipeView',
    components: {
        SeasonIcons,
    },
    inject: ["icons"],
    props: {
        recipe: {
            type : Object,
        }
    },
    emits: ['edit'],
    data: function() {
        return {
            overflown: false,
        }
    },
    computed : {
        image() {
            if (this.recipe == null || this.recipe.image === "")
                return this.icons.camera
            else
                return this.recipe.image.replace("/upload", "/upload/c_limit,h_512,w_512");
        }
    },
    mounted() {
        //https://jefrydco.id/en/blog/safe-access-vue-refs-undefined
        const interval = setInterval(() => {
            if (this.recipe) {
                this.overflown = isOverflown(this.$refs.recipe_name)
                clearInterval(interval)
            }
        }, 100)
    },
    methods: {
        editRecipe() {
            this.$emit('edit')
        }
    }
}
</script>

<style scoped>
    th, .recipe-name {
        font-family: "Rounded_Elegance";
    }

    .recipe-name {
        overflow-wrap: anywhere;
        -webkit-line-clamp: 2;
        display: -webkit-box;
        -webkit-box-orient: vertical;
        overflow: hidden;
    }
</style>

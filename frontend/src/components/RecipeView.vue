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
    class="flex flex-col w-full max-w-5xl mx-auto p-4 md:p-6 lg:p-8 xl:p-12 gap-y-12 border border-primary rounded-xl relative"
  >
    <span v-if="this.allowed_to_modify" class="icon absolute right-1 top-1 text-xl sm:right-4 sm:top-4 sm:text-2xl md:right-6 md:top-6 md:text-3xl">
      <Icon
        class="text-primary cursor-pointer"
        :icon="icons.pencil"
        @click="editRecipe()"
      />
    </span>
    <div class="flex flex-wrap sm:flex-nowrap gap-y-12 gap-x-4 md:gap-x-6">
      <div class="basis-full sm:basis-2/5 md:basis-1/3 p-2 sm:p-0">
        <img
          :src="image"
          class="rounded-xl"
        >
      </div>
      <div class="flex flex-col basis-full sm:basis-1/2 justify-between items-center mx-auto gap-y-2 sm:gap-y-0">
        <p
          ref="recipe_name"
          v-tooltip="overflown ? recipe.name : null"
          class="recipe-name text-primary text-center font-bold text-2xl sm:text-3xl md:text-4xl lg:text-5xl"
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
        <p v-if="this.is_vegan">
          <span class="icon inline-flex items-center gap-x-1 text-sm sm:text-lg md:text-xl lg:text-2xl"><Icon
            :icon="icons.vegan"
            class="text-primary text-xl sm:text-2xl md:text-3xl lg:text-4xl"
          /> Vegan</span>
        </p>
        <p v-else-if="this.is_vege">
          <span class="icon inline-flex items-center gap-x-1 text-sm sm:text-lg md:text-xl lg:text-2xl"><Icon
            :icon="icons.vege"
            class="text-primary text-xl sm:text-2xl md:text-3xl lg:text-4xl"
          /> Végétarien</span>
        </p>
        <p class="text-xs sm:text-sm italic text-center">
            Ajoutée par {{ recipe.author.display_name }}
            <br><router-link class="link-primary" :to="'/recipes?a=' + recipe.author.id">voir toutes ses recettes</router-link>
        </p>
      </div>
    </div>
    <div class="flex gap-2 flex-wrap justify-center">
      <router-link
        v-for="tag in recipe.tags"
        :key="tag.id"
        :to="'/recipes?t=' + tag.id"
        class="badge badge-outline badge-primary badge-md sm:badge-lg"
      >
        {{ tag.name }}
      </router-link>
    </div>
    <div class="flex flex-wrap sm:flex-nowrap gap-x-4 sm:gap-x-6 md:gap-x-8 items-start gap-y-12">
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
            class="border-b border-base-200"
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
    <blockquote v-if="this.recipe.notes" class="pl-4 sm:pl-6 self-center w-full">
      <p class="text-gray-400 whitespace-pre-wrap break-words"><em>
        « {{ this.recipe.notes }} »
        <p class="text-right mr-6">{{ this.recipe.author.display_name }}</p>
      </em></p>
    </blockquote>
  </div>
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
        },
        is_vege() {
            return this.recipe.diets.find(d => d.label == 'vegetarian')
        },
        is_vegan() {
            return this.recipe.diets.find(d => d.label == 'vegan')
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

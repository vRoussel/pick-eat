<template>
  <div
    class="card card-compact h-full card-bordered hover:border-primary transition ease-in-out hover:scale-105 border-accent border-2 cursor-pointer"
    @click="openRecipe(recipe.id)"
  >
    <figure class="relative">
        <img class="rounded-t-xl" :src="recipe.image.replace('upload', 'upload/c_limit,h_512,w_limit,w_512') || icons.camera">
    </figure>
    <Icon
      v-if="this.is_vegan"
      :icon="this.icons.vegan"
      class="text-3xl absolute text-primary  right-2 top-2 bg-primary-content rounded-full ring-primary ring-2"
    />
    <Icon
      v-else-if="this.is_vege"
      :icon="this.icons.vege"
      class="text-3xl absolute text-primary right-2 top-2 bg-primary-content rounded-full ring-primary ring-2"
    />
    <div class="card-body divide-y-2 divide-accent !pb-0">
      <div class="card-actions justify-evenly">
        <span class="icon">
          <Icon
            :icon="heart_svg"
            class="transition ease-in-out hover:scale-125 text-2xl text-red-600"
            @click.stop="toggleFavorite(recipe)"
          />
        </span>
        <span class="icon">
          <Icon
            :icon="cart_svg"
            class="transition ease-in-out hover:scale-125 text-2xl text-primary"
            @click.stop="toggleCart(recipe)"
          />
        </span>
      </div>
      <div class="py-4">
        <h2
          ref="recipe_name"
          v-tooltip="overflown ? recipe.name : null"
          class="text-xl recipe-name grow text-center"
        >
          {{ recipe.name }}
        </h2>
      </div>
    </div>
  </div>
</template>

<script>
import {isOverflown} from '@/utils/utils.js'
import { mapStores } from 'pinia'
import { useCartStore } from '@/store/cart.js'
import { useFoodStore } from '@/store/food.js'

export default {
    name: 'RecipeListItem',
    inject: ["icons"],
    props: {
        recipe: {
            type: Object,
            default: null,
        }
    },
    data: function() {
        return {
            overflown: false,
        }
    },
    computed: {
        ...mapStores(useCartStore, useFoodStore),
        heart_svg() {
            return this.recipe.is_favorite ? this.icons.heart : this.icons.heart_outline
        },
        cart_svg() {
            return this.cartStore.hasRecipe(this.recipe.id) ? this.icons.cart : this.icons.cart_outline
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
            if (this.$refs.recipe_name) {
                this.overflown = isOverflown(this.$refs.recipe_name)
                clearInterval(interval)
            }
        }, 100)
    },
    methods: {
        toggleFavorite(recipe) {
            this.foodStore.toggleFavorite(recipe)
        },
        openRecipe(id) {
            this.$router.push({ name: 'recipe', params: { id } })
        },
        toggleCart(recipe) {
            if (this.cartStore.hasRecipe(recipe.id)) {
                this.cartStore.removeRecipe(recipe.id)
            } else {
                this.cartStore.addRecipe(recipe, recipe.n_shares)
            }
        }
    },
}
</script>

<style scoped>
    .recipe-name {
        font-family: "Rounded_Elegance";

        overflow-wrap: anywhere;
        -webkit-line-clamp: 3;
        display: -webkit-box;
        -webkit-box-orient: vertical;
        -webkit-box-pack: center;
        overflow: hidden;
    }
</style>

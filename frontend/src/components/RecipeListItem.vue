<template>
    <div class="card card-compact h-full card-bordered hover:border-primary transition ease-in-out hover:scale-105 border-accent border-2 cursor-pointer" @click="openRecipe(recipe.id)">
        <div class="card-image">
            <figure><img :src="recipe.image.replace('upload', 'upload/c_limit,h_512,w_limit,w_512') || PLACEHOLDER_IMG"/></figure>
        </div>
        <div class="card-body flex flex-row items-center">
            <h2 ref="recipe_name" class="text-xl recipe-name grow" v-tooltip="overflown ? recipe.name : null">
                {{ recipe.name }}
            </h2>
            <div class="card-actions" aria-label="favorite">
              <span class="icon text-xl">
                <ion-icon :name="recipe.is_favorite ? 'heart' : 'heart-outline'" class="transition ease-in-out hover:scale-125 text-2xl text-red-600" @click.stop="toggleFavorite(recipe)"></ion-icon>
              </span>
            </div>
        </div>
    </div>
</template>

<script>
import {PLACEHOLDER_IMG, isOverflown} from '@/utils/utils.js'

export default {
    name: 'recipe-list-item',
    inject: ["store"],
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
    methods: {
        toggleFavorite(recipe) {
            this.store.toggleFavorite(recipe)
        },
        openRecipe(id) {
            this.$router.push({ name: 'recipe', params: { id } })
        },
    },
    created() {
        this.PLACEHOLDER_IMG = PLACEHOLDER_IMG
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

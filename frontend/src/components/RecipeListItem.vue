<template>
    <div class="card">
        <div class="card-image">
            <figure class="image is-square is-fullwidth">
                    <img :src="recipe.image.replace('upload', 'upload/c_limit,h_512,w_limit,w_512') || PLACEHOLDER_IMG" class="is-clickable" @click="openRecipe(recipe.id)"/>
            </figure>
        </div>
        <header class="card-header">
            <p ref="recipe_name" class="recipe-name card-header-title is-size-4-mobile
            is-size-5-tablet" v-tooltip="overflown ? recipe.name : null">
                {{ recipe.name }}
            </p>
            <div class="card-header-icon" aria-label="favorite">
              <span class="icon">
                <i :class="recipe.is_favorite ? 'fa' : 'far'" class="fa-heart is-size-4-mobile is-size-5-tablet is-clickable" @click="toggleFavorite(recipe)"></i>
              </span>
            </div>
        </header>
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

<style lang="scss" scoped>
@import "~bulma/sass/components/card.sass";
@import "~bulma/sass/base/generic.sass";
    .recipe-name {
        font-family: "Rounded_Elegance";
        padding: 0;
        margin: $card-header-padding;

        overflow-wrap: anywhere;
        -webkit-line-clamp: 3;
        display: -webkit-box;
        -webkit-box-orient: vertical;
        overflow: hidden;
        height: $body-line-height * $body-font-size * 3;
    }

    .fa-heart {
        color: red;
    }
    .card-header-icon {
        cursor: auto;
    }
</style>

<script setup>
import { inject, computed } from 'vue'
import { useRouter } from 'vue-router'

import { useCartStore } from '@/store/cart.js'
import { useFoodStore } from '@/store/food.js'
import { useAuthStore } from '@/store/auth.js'
import { useNotifStore } from '@/store/notif.js'

const cartStore = useCartStore()
const foodStore = useFoodStore()
const authStore = useAuthStore()
const notifStore = useNotifStore()
const router = useRouter()

const icons = inject('icons')

const props = defineProps({
    recipe: {
        type: Object,
        default: null,
    },
    lazy: {
        type: Boolean,
        default: false,
    },
})

const heart_svg = computed(() => {
    return props.recipe.is_favorite ? icons.heart : icons.heart_outline
})

const cart_svg = computed(() => {
    return cartStore.hasRecipe(props.recipe.id) ? icons.cart : icons.cart_outline
})

const is_vege = computed(() => {
    return props.recipe.diets.find((d) => d.label == 'vegetarian')
})

const is_vegan = computed(() => {
    return props.recipe.diets.find((d) => d.label == 'vegan')
})

function openRecipe(id) {
    router.push({ name: 'recipe', params: { id } })
}
</script>

<template>
    <router-link :to="'/recipe/' + props.recipe.id" :aria-label="props.recipe.name">
        <div
            class="card card-compact h-full card-bordered hover:border-primary transition ease-in-out hover:scale-105 border-accent border-2"
        >
            <picture v-if="props.recipe.image" class="relative">
                <source
                    type="image/avif"
                    :srcset="
                        props.recipe.image.replace('upload', 'upload/c_limit,h_512,w_512,f_avif')
                    "
                />
                <source
                    type="image/webp"
                    :srcset="
                        props.recipe.image.replace('upload', 'upload/c_limit,h_512,w_512,f_webp')
                    "
                />
                <img
                    class="rounded-t-xl w-[512px] aspect-square"
                    :loading="props.lazy ? 'lazy' : 'eager'"
                    :alt="props.recipe.name"
                    :src="props.recipe.image.replace('upload', 'upload/c_limit,h_512,w_512')"
                />
            </picture>
            <picture v-else class="relative">
                <img
                    class="rounded-t-xl w-[512px] aspect-square"
                    :loading="props.lazy ? 'lazy' : 'eager'"
                    alt="icone de caméra"
                    :src="icons.camera"
                />
            </picture>
            <Icon
                v-if="props.recipe.is_private"
                :icon="icons.private"
                class="text-3xl absolute text-primary left-2 top-2 bg-primary-content rounded-full ring-primary ring-2"
            />
            <Icon
                v-if="is_vegan"
                :icon="icons.vegan"
                class="text-3xl absolute text-primary right-2 top-2 bg-primary-content rounded-full ring-primary ring-2"
            />
            <Icon
                v-else-if="is_vege"
                :icon="icons.vege"
                class="text-3xl absolute text-primary right-2 top-2 bg-primary-content rounded-full ring-primary ring-2"
            />
            <div class="card-body divide-y-2 divide-accent !pb-0">
                <div class="card-actions justify-evenly">
                    <span class="icon">
                        <Icon
                            :icon="heart_svg"
                            class="transition ease-in-out hover:scale-125 text-2xl text-red-600"
                            @click.stop.prevent="foodStore.toggleFavorite(props.recipe)"
                        />
                    </span>
                    <span class="icon">
                        <Icon
                            :icon="cart_svg"
                            class="transition ease-in-out hover:scale-125 text-2xl text-primary"
                            @click.stop.prevent="cartStore.toggleRecipe(props.recipe)"
                        />
                    </span>
                </div>
                <div class="py-4">
                    <h2 class="text-xl recipe-name line-clamp-3 break-words grow text-center">
                        {{ props.recipe.name }}
                    </h2>
                </div>
            </div>
        </div>
    </router-link>
</template>

<style scoped>
.recipe-name {
    font-family: 'Rounded_Elegance';
}
</style>

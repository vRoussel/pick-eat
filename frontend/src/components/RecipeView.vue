<script setup>
import { inject, ref, computed, onMounted } from 'vue'

import { useAuthStore } from '@/store/auth.js'
import { useFoodStore } from '@/store/food.js'
import { useCartStore } from '@/store/cart.js'
import SeasonIcons from '@/components/SeasonIcons.vue'

import { isOverflown, qty_scaled } from '@/utils/utils.js'

const authStore = useAuthStore()
const foodStore = useFoodStore()
const cartStore = useCartStore()

const props = defineProps({
    recipe: Object,
})

const is_vege = computed(() => {
    return props.recipe.diets.find((d) => d.label == 'vegetarian')
})

const is_vegan = computed(() => {
    return props.recipe.diets.find((d) => d.label == 'vegan')
})

const allowed_to_modify = computed(() => {
    return (
        authStore.is_logged_in &&
        (authStore.account.id == props.recipe.author_id || authStore.is_admin)
    )
})

const overflown = ref(false)
const recipe_name_el = ref(null)
onMounted(() => {
    //https://jefrydco.id/en/blog/safe-access-vue-refs-undefined
    const interval = setInterval(() => {
        if (props.recipe) {
            overflown.value = isOverflown(recipe_name_el.value)
            clearInterval(interval)
        }
    }, 100)
})

const emit = defineEmits(['edit'])
function editRecipe() {
    emit('edit')
}

const icons = inject('icons')
function image(format = null) {
    let replacement = '/upload/c_limit,h_512,w_512'
    if (format == 'avif' || format == 'webp') {
        replacement += `,f_${format}`
    }
    return props.recipe.image.replace('/upload', replacement)
}

const asked_shares = ref(props.recipe.n_shares)
const shares_ratio = computed(() => {
    return asked_shares.value / props.recipe.n_shares
})

function increase_shares() {
    asked_shares.value += 1
}

function decrease_shares() {
    if (asked_shares.value > 1) asked_shares.value -= 1
}

const fav_btn_icon = computed(() => {
    return props.recipe.is_favorite ? icons.heart : icons.heart_outline
})

const fav_btn_text = computed(() => {
    return props.recipe.is_favorite ? 'Retirer des favoris' : 'Ajouter aux favoris'
})

const cart_btn_icon = computed(() => {
    return cartStore.hasRecipe(props.recipe.id) ? icons.cart : icons.cart_outline
})

const cart_btn_text = computed(() => {
    return cartStore.hasRecipe(props.recipe.id)
        ? 'Retirer de la liste de courses'
        : 'Ajouter à la liste de courses'
})
</script>

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
        v-if="props.recipe"
        class="flex flex-col w-full max-w-5xl mx-auto p-4 md:p-6 lg:p-8 xl:p-12 gap-y-12 border border-primary rounded-xl"
    >
        <div class="flex flex-wrap sm:flex-nowrap gap-y-12 gap-x-4 md:gap-x-6">
            <div class="basis-full sm:basis-2/5 md:basis-1/3 p-2 sm:p-0">
                <a :href="props.recipe.image" v-if="props.recipe.image">
                    <picture>
                        <source type="image/avif" :srcset="image('avif')" />
                        <source type="image/webp" :srcset="image('webp')" />
                        <img
                            :src="image()"
                            :alt="'Photo de ' + props.recipe.name"
                            class="rounded-xl w-[512px] aspect-square"
                        />
                    </picture>
                </a>
                <img
                    v-else
                    :src="icons.camera"
                    alt="icone de caméra"
                    class="rounded-xl w-[512px] aspect-square"
                />
            </div>
            <div
                class="flex flex-col basis-full sm:basis-1/2 justify-between items-center mx-auto gap-y-3 sm:gap-y-0"
            >
                <div :data-tip="props.recipe.name" :class="overflown ? 'tooltip' : ''">
                    <h1
                        ref="recipe_name_el"
                        class="recipe-name line-clamp-2 break-words text-primary text-center font-bold text-3xl lg:text-4xl"
                    >
                        {{ props.recipe.name }}
                    </h1>
                </div>
                <season-icons
                    :seasons="props.recipe.seasons"
                    class="text-3xl sm:text-2xl md:text-3xl lg:text-4xl gap-x-1"
                />
                <p class="space-x-6">
                    <span class="icon inline-flex items-center gap-x-2 text-md lg:text-xl">
                        <Icon :icon="icons.knife" :rotate="3" class="text-primary" />
                        {{ props.recipe.prep_time_min }} min
                    </span>
                    <span class="icon inline-flex items-center gap-x-2 text-md lg:text-xl">
                        <Icon :icon="icons.cooking_pot" class="text-primary" />{{
                            props.recipe.cook_time_min
                        }}
                        min
                    </span>
                </p>
                <p v-if="is_vegan">
                    <span class="icon inline-flex items-center gap-x-2 text-md lg:text-xl">
                        <Icon :icon="icons.vegan" class="text-primary text-xl lg:text-3xl" />
                        Vegan
                    </span>
                </p>
                <p v-else-if="is_vege">
                    <span class="icon inline-flex items-center gap-x-2 text-md lg:text-xl">
                        <Icon :icon="icons.vege" class="text-primary text-2xl lg:text-3xl" />
                        Végétarien
                    </span>
                </p>
                <p class="text-sm italic text-center">
                    Ajoutée par {{ props.recipe.author_name }} <br /><router-link
                        class="link-primary"
                        :to="'/recipes?a=' + props.recipe.author_id"
                    >
                        voir toutes ses recettes
                    </router-link>
                </p>
            </div>
        </div>
        <div class="flex gap-2 flex-wrap justify-center">
            <router-link
                v-for="tag in props.recipe.tags"
                :key="tag.id"
                :to="'/recipes?t=' + tag.id"
                class="badge badge-outline badge-primary badge-md sm:badge-lg"
            >
                {{ tag.name }}
            </router-link>
        </div>
        <div
            class="flex flex-wrap md:flex-nowrap gap-x-4 md:gap-x-8 items-start justify-center gap-y-12"
        >
            <table class="table table-compact basis-3/4 md:basis-1/3 shrink-0 grow sm:grow-0">
                <thead>
                    <tr class="text-center">
                        <th
                            colspan="2"
                            class="text-primary-content !bg-primary text-lg rounded-t-xl"
                        >
                            <span class="flex flex-row items-center justify-around">
                                <button
                                    class="text-xl text-primary bg-white rounded-full"
                                    @click.prevent.stop="decrease_shares"
                                    aria-label="Décrémenter le nombre de parts"
                                >
                                    <Icon :icon="icons.minus" />
                                </button>
                                <span
                                    >Ingrédients<br />({{ asked_shares }}
                                    {{ props.recipe.shares_unit }})</span
                                >
                                <button
                                    class="text-xl text-primary bg-white rounded-full"
                                    @click.prevent.stop="increase_shares"
                                    aria-label="Incrémenter le nombre de parts"
                                >
                                    <Icon :icon="icons.plus" />
                                </button>
                            </span>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr
                        v-for="ingr in props.recipe.q_ingredients"
                        :key="ingr.id"
                        class="border-b border-base-200"
                    >
                        <td class="!text-right">
                            {{
                                ingr.quantity ? qty_scaled(ingr.quantity, shares_ratio, 0.25) : ''
                            }}
                            {{ ingr.unit ? ingr.unit.short_name : '' }}
                        </td>
                        <td>{{ ingr.name }}</td>
                    </tr>
                </tbody>
            </table>
            <table class="table table-compact table-fixed grow w-full">
                <colgroup>
                    <col class="w-8" />
                    <col />
                </colgroup>
                <thead class="h-20">
                    <tr class="text-center border-b border-primary">
                        <th class="bg-transparent !text-primary text-lg" colspan="2">Étapes</th>
                    </tr>
                </thead>
                <tbody>
                    <tr
                        v-for="(step, index) in props.recipe.instructions"
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
        <blockquote v-if="props.recipe.notes" class="pl-4 sm:pl-6 self-center w-full">
            <p class="text-gray-400 whitespace-pre-wrap break-words">
                <em>
                    « {{ props.recipe.notes }} »
                    <p class="text-right mr-6">{{ props.recipe.author_name }}</p>
                </em>
            </p>
        </blockquote>
        <div class="flex flex-col md:flex-row gap-y-2 *:lg:btn-wide justify-evenly">
            <button
                v-if="allowed_to_modify"
                type="button"
                class="btn btn-outline btn-primary"
                @click="editRecipe"
            >
                Modifier la recette
                <span class="text-xl">
                    <Icon :icon="icons.pencil" />
                </span>
            </button>
            <button
                type="button"
                class="btn btn-outline btn-primary"
                @click="foodStore.toggleFavorite(props.recipe)"
            >
                {{ fav_btn_text }}
                <span class="text-xl">
                    <Icon :icon="fav_btn_icon" />
                </span>
            </button>
            <button
                type="button"
                class="btn btn-outline btn-primary"
                @click="cartStore.toggleRecipe(props.recipe, asked_shares)"
            >
                {{ cart_btn_text }}
                <span class="text-xl">
                    <Icon :icon="cart_btn_icon" />
                </span>
            </button>
        </div>
    </div>
</template>

<style scoped>
th,
.recipe-name {
    font-family: 'Rounded_Elegance';
}
</style>

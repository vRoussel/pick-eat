<script setup>
import { shallowRef, ref, watch, onMounted, computed, inject } from 'vue'
import Swal from 'sweetalert2'
import { object, string, number, array } from 'yup'

import ToggleButtons from '@/components/ToggleButtons.vue'
import IngredientPicker from '@/components/IngredientPicker.vue'
import ImageChooser from '@/components/ImageChooser.vue'
import NewTagModal from '@/components/NewTagModal.vue'
import NewCategoryModal from '@/components/NewCategoryModal.vue'
import NumberInput from '@/components/NumberInput.vue'
import { useFoodStore } from '@/store/food.js'
import { useAuthStore } from '@/store/auth.js'
import { useNotifStore } from '@/store/notif.js'
import { useCartStore } from '@/store/cart.js'

import { handle_form_api_errors, handle_form_local_errors } from '@/utils/utils.js'

const foodStore = useFoodStore()
const authStore = useAuthStore()
const notifStore = useNotifStore()
const cartStore = useCartStore()

const icons = inject('icons')

const validator = object().shape({
    name: string().required('Le nom de la recette est obligatoire'),
    prep_time: number()
        .required('Le temps de préparation est obligatoire')
        .typeError('Le temps de préparation est vide ou invalide')
        .positive('Le temps de préparation doit être supérieur à 0 minute')
        .integer(),
    cook_time: number()
        .required('Le temps de cuisson est obligatoire')
        .typeError('Le temps de cuisson est vide ou invalide')
        .min(0, 'Le temps de cuisson ne peut pas être négatif')
        .integer(),
    shares: number()
        .required('Le nombre de parts est obligatoire')
        .typeError('Le nombre de parts est vide ou invalide')
        .positive('Le nombre de parts doit être positif')
        .integer('Le nombre de parts doit être un nombre entier'),
    shares_unit: string()
        .required('La dénomination des parts est obligatoire')
        .max(15, 'La dénomination des parts ne peut pas dépasser 15 caractères'),
    categories: array()
        .transform((value) => Array.from(value))
        .min(1, 'Selectionnez au moins une catégorie'),
    seasons: array()
        .transform((value) => Array.from(value))
        .min(1, 'Selectionnez au moins une saison'),
    ingredients: array()
        .min(1, 'Ajoutez au moins un ingrédient'),
    instructions: string().required('Ajoutez les étapes pour réaliser la recette'),
    // Mandatory image is too annoying, find a better way
    //image_url: string()
    //        .required("Ajoutez une photo de la recette")
    //        .min(1, "Ajoutez une photo de la recette")
})

const props = defineProps({
    existing_recipe: Object
});

const emit = defineEmits(['done'])

const fields = ref({
    name: '',
    prep_time: null,
    cook_time: null,
    shares: 0,
    shares_unit: "personnes",
    instructions: '',
    categories: new Set(),
    tags: new Set(),
    seasons: new Set(),
    ingredients: new Array(),
    diets: new Set(),
    notes: '',
    image_url: '',
    is_private: false,
})

const errors = ref({
    name: null,
    prep_time: null,
    cook_time: null,
    shares: null,
    shares_unit: null,
    instructions: null,
    categories: null,
    tags: null,
    seasons: null,
    ingredients: null,
    diets: null,
    notes: null,
    image_url: null,
})

const NewTagModal_ = shallowRef(NewTagModal)
const NewCategoryModal_ = shallowRef(NewCategoryModal)

const update_mode = computed(() => {
    return props.existing_recipe != null
})

function addWatchers() {
    watch(() => fields.value.diets, (new_val, old_val) => {
        let added_vegan = !old_val.has(2) && new_val.has(2)
        let removed_vege = old_val.has(1) && !new_val.has(1)
        if (added_vegan) {
            new_val.add(1)
        } else if (removed_vege) {
            new_val.delete(2)
        }
    })

    watch(() => fields.value.categories, () => {
        validate('categories')
    })
    watch(() => fields.value.seasons, () => {
        validate('seasons')
    })
    watch(() => fields.value.ingredients, () => {
        validate('ingredients')
    })
    watch(() => fields.value.shares, () => {
        validate('shares')
    })

    watch(fields, (value) => {
        localStorage.setItem("recipeform", JSON.stringify({
            ...value,
            categories: Array.from(value.categories),
            tags: Array.from(value.tags),
            seasons: Array.from(value.seasons),
            diets: Array.from(value.diets)
        }))
    }, { deep: true })
}

// Mandatory image is too annoying, find a better way
//image_url() {
//    this.validate('image_url')
//}
onMounted(() => {
    fillForm()
    addWatchers()
})

function sendRecipe() {
    let f = fields.value
    let recipe = {
        name: f.name,
        q_ingredients: f.ingredients,
        category_ids: Array.from(f.categories),
        tag_ids: Array.from(f.tags),
        season_ids: Array.from(f.seasons),
        diet_ids: Array.from(f.diets),
        prep_time_min: f.prep_time,
        cook_time_min: f.cook_time,
        image: f.image_url,
        instructions: f.instructions.split(/\r?\n/).filter((i) => i),
        notes: f.notes,
        n_shares: f.shares,
        shares_unit: f.shares_unit,
        is_private: f.is_private,
    }
    for (var ingr of recipe.q_ingredients) {
        if (ingr.quantity == null)
            ingr.unit_id = null
    }

    errors.value = {}
    validator
        .validate(fields.value, { abortEarly: false })
        .then(() => {
            if (!update_mode.value) {
                foodStore
                    .sendNewRecipe(recipe)
                    .then((recipe) => {
                        Swal.fire({
                            title: 'Recette ajoutée',
                            icon: 'success',
                        })
                        emit('done', recipe)
                        clearForm()
                    })
                    .catch((err) => {
                        handle_form_api_errors(err.response, errors.value, notifStore)
                    })
            } else {
                foodStore
                    .updateRecipe(props.existing_recipe.id, recipe)
                    .then(() => {
                        if (cartStore.hasRecipe(props.existing_recipe.id)) {
                            cartStore.updateRecipe(props.existing_recipe.id, recipe)
                        }
                        emit('done')
                    })
                    .catch((err) => {
                        handle_form_api_errors(err.response, errors.value, notifStore)
                    })
            }
        })
        .catch((err) => {
            handle_form_local_errors(err.inner, errors.value, notifStore)
        })
}

function cancel() {
    emit('done')
}

function fillForm() {
    let backup = JSON.parse(localStorage.getItem("recipeform"))
    if (props.existing_recipe) {
        let other = props.existing_recipe

        let f = fields.value
        f.name = other.name
        f.ingredients = other.q_ingredients.map((ingr) => {
            return {
                id: ingr.id,
                unit_id: ingr.unit ? ingr.unit.id : null,
                quantity: ingr.quantity
            }
        })
        f.categories = new Set(other.categories.map((c) => c.id))
        f.tags = new Set(other.tags.map((t) => t.id))
        f.seasons = new Set(other.seasons.map((s) => s.id))
        f.diets = new Set(other.diets.map((d) => d.id))
        f.prep_time = other.prep_time_min
        f.cook_time = other.cook_time_min
        f.image_url = other.image
        f.instructions = other.instructions.join('\n')
        f.notes = other.notes
        f.shares = other.n_shares
        f.shares_unit = other.shares_unit
        f.is_private = other.is_private
    } else if (backup) {
        try {
            fields.value = {
                ...backup,
                categories: new Set(backup.categories),
                tags: new Set(backup.tags),
                seasons: new Set(backup.seasons),
                diets: new Set(backup.diets)
            }
            errors.value = {}
        } catch (e) {
            console.error("Unable to restore recipe form from local storage")
        }
    }
}

function clearForm() {
    let f = fields.value
    f.name = ''
    f.prep_time = null
    f.cook_time = null
    f.shares = 0
    f.shares_unit = "personnes"
    f.instructions = ''
    f.categories.clear()
    f.tags.clear()
    f.seasons.clear()
    f.ingredients.clear()
    f.diets.clear()
    f.notes = ''
    f.image_url = ''
    f.is_private = false
}

function validate(field) {
    validator
        .validateAt(field, fields.value)
        .then(() => {
            errors.value[field] = null
        })
        .catch((err) => {
            setTimeout(() => (errors.value[field] = err.message), 200)
        })
}
</script>

<template>
    <form id="recipe-form" autocomplete="off" class="gap-y-8 flex flex-col items-start max-w-3xl mx-auto px-2 my-4"
        @submit.prevent="sendRecipe">
        <div class="form-control mt-6">
            <button type="button" class="btn btn-outline btn-primary btn-wide" @click="clearForm">
                Réinitialiser les champs
                <span class="text-xl">
                    <Icon :icon="icons.reset" />
                </span>
            </button>
        </div>
        <div class="form-control w-full">
            <label class="label">
                <span class="label-text">Nom</span>
            </label>
            <input v-model="fields.name" type="text" class="input input-bordered w-full"
                :class="errors.name && '!input-error'" @blur="validate('name')" />
            <label v-if="errors.name" class="label">
                <span class="label-text-alt text-error">{{ errors.name }}</span>
            </label>
        </div>

        <div class="flex flex-wrap sm:flex-nowrap w-full gap-y-5 gap-x-5">
            <div class="flex flex-col justify-around justify-self-start grow sm:grow-0 basis-4/12">
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">Temps de préparation</span>
                    </label>
                    <input v-model="fields.prep_time" type="number" class="input input-bordered w-full"
                        placeholder="(en minutes)" :class="errors.prep_time && '!input-error'"
                        @blur="validate('prep_time')" />
                    <label v-if="errors.prep_time" class="label">
                        <span class="label-text-alt text-error">{{ errors.prep_time }}</span>
                    </label>
                </div>

                <div class="form-control">
                    <label class="label">
                        <span class="label-text">Temps de cuisson</span>
                    </label>
                    <input v-model="fields.cook_time" type="number" class="input input-bordered w-full"
                        placeholder="(en minutes)" :class="errors.cook_time && '!input-error'"
                        @blur="validate('cook_time')" />
                    <label v-if="errors.cook_time" class="label">
                        <span class="label-text-alt text-error">{{ errors.cook_time }}</span>
                    </label>
                </div>

                <div class="form-control w-full">
                    <label class="label">
                        <span class="label-text">Nombre de parts</span>
                    </label>
                    <div class="flex flex-row items-start gap-x-5">
                        <div class="form-control">
                            <number-input v-model.number="fields.shares" :min="0" :badvalue="errors.shares != null" />
                            <label v-if="errors.shares" class="label">
                                <span class="label-text-alt text-error">{{ errors.shares }}</span>
                            </label>
                        </div>
                        <div class="form-control">
                            <input v-model="fields.shares_unit" class="input input-bordered"
                                :class="errors.shares_unit && '!input-error'" @blur=" validate('shares_unit')"
                                placeholder="parts, personnes, crêpes, etc" />
                            <label v-if="errors.shares_unit" class="label">
                                <span class="label-text-alt text-error">{{ errors.shares_unit }}</span>
                            </label>
                        </div>
                    </div>
                </div>
            </div>
            <div class="form-control grow order-first sm:order-none justify-self-center">
                <image-chooser v-model:image_url="fields.image_url" />
                <label v-if="errors.image_url" class="label mx-auto">
                    <span class="label-text-alt text-error">{{ errors.image_url }}</span>
                </label>
            </div>
        </div>

        <div class="form-control w-full">
            <label class="label">
                <span class="label-text">Catégories</span>
            </label>
            <toggle-buttons v-model:picked="fields.categories" :choices="foodStore.categories"
                :extendable="authStore.is_admin" :extend-modal-component="NewCategoryModal_"
                :class="errors.categories && '!border-error'" />
            <label v-if="errors.categories" class="label">
                <span class="label-text-alt text-error">{{ errors.categories }}</span>
            </label>
        </div>

        <div class="form-control w-full">
            <label class="label">
                <span class="label-text">Tags</span>
            </label>
            <toggle-buttons v-model:picked="fields.tags" :choices="foodStore.tags" :extendable="authStore.is_admin"
                :extend-modal-component="NewTagModal_" />
        </div>

        <div class="form-control w-full">
            <label class="label">
                <span class="label-text">Saisons</span>
            </label>
            <toggle-buttons v-model:picked="fields.seasons" :choices="foodStore.seasons"
                :class="errors.seasons && '!border-error'" />
            <label v-if="errors.seasons" class="label">
                <span class="label-text-alt text-error">{{ errors.seasons }}</span>
            </label>
        </div>

        <div class="form-control w-full">
            <label class="label">
                <span class="label-text">Régimes alimentaires</span>
            </label>
            <toggle-buttons v-model:picked="fields.diets" :choices="foodStore.diets" />
        </div>

        <div class="form-control w-full">
            <label class="label">
                <span class="label-text">Ingrédients</span>
            </label>
            <ingredient-picker ref="ingredients" v-model:picked="fields.ingredients"
                :class="errors.ingredients && '!input-error !border-2'" />
            <label v-if="errors.ingredients" class="label">
                <span class="label-text-alt text-error">{{ errors.ingredients }}</span>
            </label>
        </div>

        <div class="form-control w-full">
            <label class="label">
                <span class="label-text">Étapes</span>
            </label>
            <textarea v-model="fields.instructions" class="textarea textarea-bordered h-40"
                placeholder="Une étape par ligne" :class="errors.instructions && '!input-error'"
                @blur="validate('instructions')" />
            <label v-if="errors.instructions" class="label">
                <span class="label-text-alt text-error">{{ errors.instructions }}</span>
            </label>
        </div>

        <div class="form-control w-full">
            <label class="label">
                <span class="label-text">Notes</span>
            </label>
            <textarea v-model="fields.notes" class="textarea textarea-bordered h-32" />
        </div>

        <div class="form-control w-full">
            <label class="label cursor-pointer justify-start gap-x-4">
                <input v-model="fields.is_private" type="checkbox" class="checkbox checkbox-sm checkbox-accent" />
                <span class="label-text">Recette privée</span>
            </label>
        </div>

        <div class="flex flex-row md:flex-row gap-y-2 sm:*:btn-wide *:btn-lg justify-evenly w-full">
            <button v-if="update_mode" type="button" class="btn btn-primary btn-outline" @click="cancel">
                Annuler
            </button>
            <button class="btn btn-primary">
                {{ update_mode ? 'Valider' : 'Ajouter' }}
            </button>
        </div>
    </form>
</template>

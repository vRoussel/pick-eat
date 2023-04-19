<template>
  <form
    id="recipe-form"
    autocomplete="off"
    class="gap-y-8 flex flex-col items-center max-w-3xl mx-auto px-2 my-4"
    @submit.prevent="sendRecipe"
  >
    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Nom</span>
      </label>
      <input
        v-model="name"
        type="text"
        class="input input-bordered w-full"
        :class="errors.name && '!input-error'"
        @blur="validate('name')"
      >
      <label
        v-if="errors.name"
        class="label"
      >
        <span class="label-text-alt text-error">{{ errors.name }}</span>
      </label>
    </div>
    <div class="flex flex-wrap sm:flex-nowrap w-full gap-y-5 gap-x-5">
      <div class="flex flex-col justify-around justify-self-start grow sm:grow-0 basis-4/12">
        <div class="form-control">
          <label class="label">
            <span class="label-text">Temps de préparation</span>
          </label>
          <label class="input-group">
            <input
              v-model="prep_time"
              type="number"
              class="input input-bordered w-full"
              :class="errors.prep_time && '!input-error'"
              @blur="validate('prep_time')"
            >
            <span class="bg-accent text-accent-content">minutes</span>
          </label>
          <label
            v-if="errors.prep_time"
            class="label"
          >
            <span class="label-text-alt text-error">{{ errors.prep_time }}</span>
          </label>
        </div>

        <div class="form-control">
          <label class="label">
            <span class="label-text">Temps de cuisson</span>
          </label>
          <label class="input-group">
            <input
              v-model="cook_time"
              type="number"
              class="input input-bordered w-full"
              :class="errors.cook_time && '!input-error'"
              @blur="validate('cook_time')"
            >
            <span class="bg-accent text-accent-content">minutes</span>
          </label>
          <label
            v-if="errors.cook_time"
            class="label"
          >
            <span class="label-text-alt text-error">{{ errors.cook_time }}</span>
          </label>
        </div>

        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">Parts</span>
          </label>
          <number-input
            v-model="shares"
            :min="0"
            :badvalue="errors.shares != null"
          />
          <label
            v-if="errors.shares"
            class="label"
          >
            <span class="label-text-alt text-error">{{ errors.shares }}</span>
          </label>
        </div>
      </div>
      <div class="form-control grow order-first sm:order-none justify-self-center">
        <image-chooser v-model:image_url="image_url" />
        <label
          v-if="errors.image_url"
          class="label mx-auto"
        >
          <span class="label-text-alt text-error">{{ errors.image_url }}</span>
        </label>
      </div>
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Catégories</span>
      </label>
      <toggle-buttons
        v-model:picked="categories"
        :choices="foodStore.categories"
        :extendable="authStore.is_admin"
        :extend-modal-component="meta.NewCategoryModal_"
        :class="errors.categories && '!border-error'"
      />
      <label
        v-if="errors.categories"
        class="label"
      >
        <span class="label-text-alt text-error">{{ errors.categories }}</span>
      </label>
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Tags</span>
      </label>
      <toggle-buttons
        v-model:picked="tags"
        :choices="foodStore.tags"
        :extendable="authStore.is_admin"
        :extend-modal-component="meta.NewTagModal_"
      />
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Saisons</span>
      </label>
      <toggle-buttons
        v-model:picked="seasons"
        :choices="foodStore.seasons"
        :class="errors.seasons && '!border-error'"
      />
      <label
        v-if="errors.seasons"
        class="label"
      >
        <span class="label-text-alt text-error">{{ errors.seasons }}</span>
      </label>
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Régimes alimentaires</span>
      </label>
      <toggle-buttons
        v-model:picked="diets"
        :choices="foodStore.diets"
      />
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Ingrédients</span>
      </label>
      <ingredient-picker
        ref="ingredients"
        v-model:picked="ingredients"
        :class="errors.ingredients && '!input-error !border-2'"
        @createIngredient="openNewIngredientForm"
        @createUnit="openNewUnitForm"
      />
      <label
        v-if="errors.ingredients"
        class="label"
      >
        <span class="label-text-alt text-error">{{ errors.ingredients }}</span>
      </label>
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Étapes</span>
      </label>
      <textarea
        v-model="instructions"
        class="textarea textarea-bordered h-40"
        placeholder="Une étape par ligne"
        :class="errors.instructions && '!input-error'"
        @blur="validate('instructions')"
      />
      <label
        v-if="errors.instructions"
        class="label"
      >
        <span class="label-text-alt text-error">{{ errors.instructions }}</span>
      </label>
    </div>


    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Notes</span>
      </label>
      <textarea
        v-model="notes"
        class="textarea textarea-bordered h-32"
      />
    </div>

    <button class="btn btn-primary w-full btn-lg">
      {{ update_mode ? 'Valider' : 'Ajouter' }}
    </button>
    <button
      v-if="update_mode"
      type="button"
      class="btn btn-accent w-full btn-lg"
      @click="cancel"
    >
      Annuler
    </button>
  </form>
</template>

<script>
import ToggleButtons from '@/components/ToggleButtons.vue' 
import IngredientPicker from '@/components/IngredientPicker.vue'
import ImageChooser from '@/components/ImageChooser.vue'
import NewTagModal from '@/components/NewTagModal.vue'
import NewCategoryModal from '@/components/NewCategoryModal.vue'
import NumberInput from '@/components/NumberInput.vue'
import Swal from 'sweetalert2'
import {shallowRef} from 'vue'
import {handle_form_api_errors, handle_form_local_errors} from '@/utils/utils.js'
import { object, string, number, array } from "yup";

import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'
import { useAuthStore } from '@/store/auth.js'
import { useNotifStore } from '@/store/notif.js'

const validator = object().shape({
    name: string()
            .required("Le nom de la recette est obligatoire"),
    prep_time: number()
            .required("Le temps de préparation est obligatoire")
            .typeError("Le temps de préparation est vide ou invalide")
            .positive("Le temps de préparation doit être supérieur à 0 minute")
            .integer(),
    cook_time: number()
            .required("Le temps de cuisson est obligatoire")
            .typeError("Le temps de cuisson est vide ou invalide")
            .min(0, "Le temps de cuisson ne peut pas être négatif")
            .integer(),
    shares: number()
            .required("Le nombre de parts est obligatoire")
            .typeError("Le nombre de parts est vide ou invalide")
            .positive("Le nombre de parts doit être positif")
            .integer("Le nombre de parts doit être un nombre entier"),
    categories: array()
            .transform((value) => Array.from(value))
            .min(1, "Selectionnez au moins une catégorie"),
    seasons: array()
            .transform((value) => Array.from(value))
            .min(1, "Selectionnez au moins une saison"),
    ingredients: array()
            .transform((value) => Array.from(value))
            .min(1, "Ajoutez au moins un ingrédient"),
    instructions: string()
            .required("Ajouter les étapes pour réaliser la recette"),
    image_url: string()
            .required("Ajoutez une photo de la recette")
            .min(1, "Ajoutez une photo de la recette")
})

export default {
    name: 'RecipeForm',
    components: {
      ToggleButtons,
      IngredientPicker,
      ImageChooser,
      NumberInput,
    },
    props: {
        existing_recipe: {
            type: Object
        }
    },
    emits: ['done'],
    data: function() {
        return {
            name: "",
            prep_time: null,
            cook_time: null,
            shares: 0,
            instructions: "",
            categories: new Set(),
            tags: new Set(),
            seasons: new Set(),
            ingredients: new Map(),
            diets: new Set(),
            notes: "",
            image_url: "",
            meta: {
                NewTagModal_: shallowRef(NewTagModal),
                NewCategoryModal_: shallowRef(NewCategoryModal),
            },
            errors: {
                name: null,
                prep_time: null,
                cook_time: null,
                shares: null,
                instructions: null,
                categories: null,
                tags: null,
                seasons: null,
                ingredients: null,
                diets: null,
                notes: null,
                image_url: null
            }
        }
    },
    computed: {
        ...mapStores(useFoodStore, useAuthStore, useNotifStore),
        update_mode() {
            return this.existing_recipe != null
        },
        insert_mode() {
            return !this.update_mode
        },
    },
    watch: {
        existing_recipe: function() {
            this.fillForm()
        },
        diets: {
            handler(new_val, old_val) {
                let added_vegan = !old_val.has(2) && new_val.has(2)
                let removed_vege = old_val.has(1) && !new_val.has(1)
                if (added_vegan) {
                    new_val.add(1)
                }
                else if (removed_vege) {
                    new_val.delete(2)
                }
            }
        },
        categories() {
            this.validate('categories')
        },
        seasons() {
            this.validate('seasons')
        },
        ingredients() {
            this.validate('ingredients')
        },
        shares() {
            this.validate('shares')
        },
        image_url() {
            this.validate('image_url')
        }
    },
    mounted() {
        this.fillForm()
    },
    methods: {
        sendRecipe() {
            let recipe = {
                "name": this.name,
                "q_ingredients": Array.from(this.ingredients.values()),
                "category_ids": Array.from(this.categories),
                "tag_ids": Array.from(this.tags),
                "season_ids": Array.from(this.seasons),
                "diet_ids": Array.from(this.diets),
                "prep_time_min": this.prep_time,
                "cook_time_min": this.cook_time,
                "image": this.image_url,
                "instructions": this.instructions.split(/\r?\n/).filter(i => i),
                "notes": this.notes,
                "n_shares": this.shares,
            }
            for (var ingr of recipe.q_ingredients) {
                if (ingr.quantity == null)
                    ingr.unit_id = null;
            }

            validator
                .validate(this, { abortEarly: false })
                .then(() => {
                    this.errors = {};
                    if (this.insert_mode) {
                        this.foodStore.sendNewRecipe(recipe)
                            .then((recipe) => {
                                Swal.fire({
                                  title: 'Recette ajoutée',
                                  icon: 'success'
                                })
                                this.$emit('done', recipe)
                            }) 
                            .catch(err => {
                                handle_form_api_errors(err.response, this.errors, this.notifStore)
                            })
                    } else {
                        this.foodStore.updateRecipe(this.existing_recipe.id, recipe)
                            .then(() => {
                                this.$emit('done')
                            }) 
                            .catch(err => {
                                handle_form_api_errors(err.response, this.errors, this.notifStore)
                            })

                    }
                })
                .catch(err => {
                    handle_form_local_errors(err.inner, this.errors, this.notifStore)
                });
        },
        cancel() {
            this.$emit('done')
        },
        openNewIngredientForm(input) {
            this.$refs.modal.openNewIngredientForm(input)
        },
        openNewUnitForm(input) {
            this.$refs.modal.openNewUnitForm(input)
        },
        addIngredient(new_ingredient) {
            this.$refs.ingredients.add_ingr(new_ingredient)
        },
        setUnit(new_unit) {
            this.$refs.ingredients.set_current_unit(new_unit)
        },
        fillForm() {
            if (this.existing_recipe) {
                let other = this.existing_recipe

                this.name = other.name
                this.ingredients = new Map(other.q_ingredients.map(ingr => [ingr.id, {id: ingr.id, unit_id: ingr.unit ? ingr.unit.id : null, quantity: ingr.quantity}]))
                this.categories = new Set(other.categories.map( c => c.id))
                this.tags = new Set(other.tags.map( t => t.id))
                this.seasons = new Set(other.seasons.map( s => s.id))
                this.diets = new Set(other.diets.map( d => d.id))
                this.prep_time = other.prep_time_min
                this.cook_time = other.cook_time_min
                this.image_url = other.image
                this.instructions = other.instructions.join('\n')
                this.notes = other.notes
                this.shares = other.n_shares
            }
        },
        validate(field) {
            console.log(this.prep_time)
            validator.validateAt(field, this)
            .then(() => {
                this.errors[field] = null;
            })
            .catch(err => {
                setTimeout(() => this.errors[field] = err.message, 200)
            });
        }
    }
}
</script>

<template>
  <form
    id="recipe-form"
    autocomplete="off"
    class="space-y-4 flex flex-col items-center max-w-3xl mx-auto px-2 my-4"
    @submit.prevent="sendRecipe"
  >
    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Nom</span>
      </label>
      <input
        v-model="new_recipe.name"
        type="text"
        class="input input-bordered w-full"
      >
    </div>
    <div class="flex flex-wrap sm:flex-nowrap items-stretch justify-between w-full gap-y-5 gap-x-5">
      <div class="form-control grow sm:grow-0">
        <image-chooser v-model:image_url="new_recipe.image_url" />
      </div>
      <div class="flex flex-col justify-evenly grow">
        <div class="form-control">
          <label class="label">
            <span class="label-text">Temps de préparation</span>
          </label>
          <label class="input-group">
            <input
              v-model="new_recipe.prep_time"
              type="number"
              class="input input-bordered w-full"
            >
            <span class="bg-accent text-accent-content">minutes</span>
          </label>
        </div>

        <div class="form-control">
          <label class="label">
            <span class="label-text">Temps de cuisson</span>
          </label>
          <label class="input-group">
            <input
              v-model="new_recipe.cook_time"
              type="number"
              class="input input-bordered w-full"
            >
            <span class="bg-accent text-accent-content">minutes</span>
          </label>
        </div>

        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">Parts</span>
          </label>
          <number-input
            v-model="new_recipe.shares"
            :min="0"
          />
        </div>
      </div>
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Étapes</span>
      </label>
      <textarea
        v-model="new_recipe.instructions"
        class="textarea textarea-bordered h-40"
        placeholder="Une étape par ligne"
      />
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Catégories</span>
      </label>
      <toggle-buttons
        v-model:picked="new_recipe.categories"
        :choices="foodStore.categories"
        extendable
        :extend-modal-component="NewCategoryModal_"
      />
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Tags</span>
      </label>
      <toggle-buttons
        v-model:picked="new_recipe.tags"
        :choices="foodStore.tags"
        extendable
        :extend-modal-component="NewTagModal_"
      />
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Saisons</span>
      </label>
      <toggle-buttons
        v-model:picked="new_recipe.seasons"
        :choices="foodStore.seasons"
      />
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Ingrédients</span>
      </label>
      <ingredient-picker
        ref="ingredients"
        v-model:picked="new_recipe.ingredients"
        @createIngredient="openNewIngredientForm"
        @createUnit="openNewUnitForm"
      />
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Notes</span>
      </label>
      <textarea
        v-model="new_recipe.notes"
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

import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'

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
            new_recipe: {
                name: "",
                prep_time: 0,
                cook_time: 0,
                shares: 0,
                instructions: "",
                categories: new Set(),
                tags: new Set(),
                seasons: new Set(),
                ingredients: new Map(),
                notes: "",
                image_url: "",
            },
            NewTagModal_: shallowRef(NewTagModal),
            NewCategoryModal_: shallowRef(NewCategoryModal),
        }
    },
    computed: {
        ...mapStores(useFoodStore),
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
        }
    },
    mounted() {
        this.fillForm()
    },
    methods: {
        sendRecipe() {
            const r = this.new_recipe;
            let recipe = {
                "name": r.name,
                "q_ingredients": Array.from(r.ingredients.values()),
                "category_ids": Array.from(r.categories),
                "tag_ids": Array.from(r.tags),
                "season_ids": Array.from(r.seasons),
                "prep_time_min": r.prep_time,
                "cook_time_min": r.cook_time,
                "image": r.image_url,
                "instructions": r.instructions.split(/\r?\n/).filter(i => i),
                "notes": r.notes,
                "n_shares": r.shares,
            }
            for (var ingr of recipe.q_ingredients) {
                if (ingr.quantity == null)
                    ingr.unit_id = null;
            }

            if (this.insert_mode) {
                this.foodStore.sendNewRecipe(recipe)
                    .then((recipe) => {
                        Swal.fire({
                          title: 'Recette ajoutée',
                          icon: 'success'
                        })
                        this.$emit('done', recipe)
                    }) 
                    .catch((e) => {
                        console.error(e)
                        Swal.fire({
                          title: 'Erreur',
                          text: e.statusText,
                          icon: 'error'
                        })
                    })
            } else {
                this.foodStore.updateRecipe(this.existing_recipe.id, recipe)
                    .then(() => {
                        this.$emit('done')
                    }) 
                    .catch((e) => {
                        console.error(e)
                        Swal.fire({
                          title: 'Erreur',
                          text: e.statusText,
                          icon: 'error'
                        })
                    })

            }
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
                this.new_recipe = {
                    name: other.name,
                    ingredients: new Map(other.q_ingredients.map(ingr => [ingr.id, {id: ingr.id, unit_id: ingr.unit ? ingr.unit.id : null, quantity: ingr.quantity}])),
                    categories: new Set(other.categories.map( c => c.id)),
                    tags: new Set(other.tags.map( t => t.id)),
                    seasons: new Set(other.seasons.map( s => s.id)),
                    prep_time: other.prep_time_min,
                    cook_time: other.cook_time_min,
                    image_url: other.image,
                    instructions: other.instructions.join('\n'),
                    notes: other.notes,
                    shares: other.n_shares,
                }
            }
        }
    }
}
</script>

<style>
.is-circular {
    border-radius: 50% !important;
}
</style>

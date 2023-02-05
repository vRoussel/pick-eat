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
        v-model="name"
        type="text"
        class="input input-bordered w-full"
      >
    </div>
    <div class="flex flex-wrap sm:flex-nowrap items-stretch justify-between w-full gap-y-5 gap-x-5">
      <div class="form-control grow sm:grow-0">
        <image-chooser v-model:image_url="image_url" />
      </div>
      <div class="flex flex-col justify-evenly grow">
        <div class="form-control">
          <label class="label">
            <span class="label-text">Temps de préparation</span>
          </label>
          <label class="input-group">
            <input
              v-model="prep_time"
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
              v-model="cook_time"
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
            v-model="shares"
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
        v-model="instructions"
        class="textarea textarea-bordered h-40"
        placeholder="Une étape par ligne"
      />
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Catégories</span>
      </label>
      <toggle-buttons
        v-model:picked="categories"
        :choices="foodStore.categories"
        extendable
        :extend-modal-component="meta.NewCategoryModal_"
      />
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Tags</span>
      </label>
      <toggle-buttons
        v-model:picked="tags"
        :choices="foodStore.tags"
        extendable
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
      />
    </div>

    <div class="form-control w-full">
      <label class="label">
        <span class="label-text">Ingrédients</span>
      </label>
      <ingredient-picker
        ref="ingredients"
        v-model:picked="ingredients"
        @createIngredient="openNewIngredientForm"
        @createUnit="openNewUnitForm"
      />
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
            meta: {
                NewTagModal_: shallowRef(NewTagModal),
                NewCategoryModal_: shallowRef(NewCategoryModal),
            }
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
            let recipe = {
                "name": this.name,
                "q_ingredients": Array.from(this.ingredients.values()),
                "category_ids": Array.from(this.categories),
                "tag_ids": Array.from(this.tags),
                "season_ids": Array.from(this.seasons),
                "prep_time_min": this.prep_time,
                "cook_time_min": this.cook_time,
                "image": this.image_url,
                "instructions": this.instructions.split(/\this?\n/).filter(i => i),
                "notes": this.notes,
                "n_shares": this.shares,
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

                this.name = other.name
                this.ingredients = new Map(other.q_ingredients.map(ingr => [ingr.id, {id: ingr.id, unit_id: ingr.unit ? ingr.unit.id : null, quantity: ingr.quantity}]))
                this.categories = new Set(other.categories.map( c => c.id))
                this.tags = new Set(other.tags.map( t => t.id))
                this.seasons = new Set(other.seasons.map( s => s.id))
                this.prep_time = other.prep_time_min
                this.cook_time = other.cook_time_min
                this.image_url = other.image
                this.instructions = other.instructions.join('\n')
                this.notes = other.notes
                this.shares = other.n_shares
            }
        }
    }
}
</script>

<template>
        <form @submit.prevent="sendRecipe" id="recipe-form" autocomplete="off">
            <div class="field">
                <label class="label">Nom</label>
                <div class="control">
                    <input v-model="new_recipe.name" class="input" type="text" name="name" id="name" required>
                </div>
            </div>

            <div class="field">
                <label class="label">Temps de préparation (minutes)</label>
                <div class="control">
                    <input v-model.number="new_recipe.prep_time" class="input" type="number" min=0 step="1" name="prep-time" id="prep-time">
                </div>
            </div>

            <div class="field">
                <label class="label">Temps de cuisson (minutes)</label>
                <div class="control">
                    <input v-model.number="new_recipe.cook_time" class="input" type="number" min=0 step="1" name="cook-time" id="cook-time">
                </div>
            </div>

            <div class="field">
                <label class="label">Parts</label>
                <div class="control">
                    <input v-model.number="new_recipe.shares" class="input" type="number" min=0 step="1" name="shares" id="shares">
                </div>
            </div>

            <div class="field">
                <label class="label">Etapes</label>
                <textarea v-model="new_recipe.instructions" class="textarea" name="cook-time" id="cook-time" placeholder="Une étape par ligne"></textarea>
            </div>

            <fieldset class="block">
                <legend class="label">Catégories</legend>
                <toggle-buttons :choices="store.state.categories" v-model:picked="new_recipe.categories" extendable :extendComponent="NewCategory_">
                </toggle-buttons>
            </fieldset>

            <fieldset class="block">
                <legend class="label">Tags</legend>
                <toggle-buttons :choices="store.state.tags" v-model:picked="new_recipe.tags" extendable :extendComponent="NewTag_">
                </toggle-buttons>
            </fieldset>

            <fieldset class="block">
                <legend class="label">Saisons</legend>
                <toggle-buttons :choices="store.state.seasons" v-model:picked="new_recipe.seasons">
                </toggle-buttons>
            </fieldset>


            <fieldset class="block">
                <legend class="label">Ingrédients</legend>
                <ingredient-picker v-model:picked="new_recipe.ingredients" @createIngredient="openNewIngredientForm" @createUnit="openNewUnitForm" ref="ingredients">
                </ingredient-picker>
            </fieldset>

            <div class="field">
                <label class="label">Notes</label>
                <textarea v-model="new_recipe.notes" class="textarea" name="notes" id="notes"></textarea>
            </div>

            <div class="field">
                <image-chooser v-model:image_url="this.new_recipe.image_url"></image-chooser>
            </div>

            <button class="button is-primary is-large is-fullwidth">{{ update_mode ? 'Modifier' : 'Ajouter' }}</button>
            <button class="button is-light is-large is-fullwidth" v-if="update_mode" @click="cancel">Annuler</button>
        </form>
</template>

<script>
import ToggleButtons from '@/components/ToggleButtons.vue'
import IngredientPicker from '@/components/IngredientPicker.vue'
import ImageChooser from '@/components/ImageChooser.vue'
import NewTag from '@/components/NewTag.vue'
import NewCategory from '@/components/NewCategory.vue'
import Swal from 'sweetalert2'
import {shallowRef} from 'vue'

export default {
    name: 'recipe-form',
    inject: ["store"],
    components: {
      ToggleButtons,
      IngredientPicker,
      ImageChooser,
    },
    props: {
        existing_recipe: {
            type: Object
        }
    },
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
                image_url: ""
            },
            NewTag_: shallowRef(NewTag),
            NewCategory_: shallowRef(NewCategory),
        }
    },
    methods: {
        sendRecipe() {
            const r = this.new_recipe;
            let recipe = {
                "name": r.name,
                "q_ingredient_ids": Array.from(r.ingredients.values()),
                "category_ids": Array.from(r.categories),
                "tag_ids": Array.from(r.tags),
                "season_ids": Array.from(r.seasons),
                "prep_time_min": r.prep_time,
                "cook_time_min": r.cook_time,
                "image": r.image_url,
                "instructions": r.instructions.split(/\r?\n/),
                "notes": r.notes,
                "n_shares": r.shares,
                "is_favorite": false
            }
            for (var ingr of recipe.q_ingredient_ids) {
                if (ingr.quantity == null)
                    ingr.unit_id = null;
            }

            if (this.insert_mode) {
                this.store.addRecipe(recipe)
                    .then(() => {
                        Swal.fire({
                          title: 'Recette ajoutée',
                          icon: 'success'
                        })
                        this.$emit('done')
                    }) 
                    .catch((e) => {
                        Swal.fire({
                          title: 'Erreur',
                          text: e.statusText,
                          icon: 'error'
                        })
                    })
            } else {
                this.store.updateRecipe(this.existing_recipe.id, recipe)
                    .then(() => {
                        this.$emit('done')
                    }) 
                    .catch((e) => {
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
    },
    mounted() {
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
                is_favorite: other.is_favorite,

            }
        }
    },
    computed: {
        update_mode() {
            return this.existing_recipe != null
        },
        insert_mode() {
            return !this.update_mode
        },
    },
    emits: ['done']
}
</script>

<style>
.is-circular {
    border-radius: 50% !important;
}
</style>

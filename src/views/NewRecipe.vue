<template>
    <div class="columns is-centered is-mobile">
        <form @submit.prevent="sendRecipe" id="recipe-form" class="column is-half-desktop is-three-quarters-mobile" autocomplete="off">
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
                <toggle-buttons class="my-3" :choices="store.state.categories" v-model:picked="new_recipe.categories" extendable @addChoice="openNewCategoryForm">
                </toggle-buttons>
            </fieldset>

            <fieldset class="block">
                <legend class="label">Tags</legend>
                <toggle-buttons class="my-3" :choices="store.state.tags" v-model:picked="new_recipe.tags" extendable @addChoice="openNewTagForm">
                </toggle-buttons>
            </fieldset>

            <fieldset class="block">
                <legend class="label">Seasons</legend>
                <toggle-buttons class="my-3" :choices="store.state.seasons" v-model:picked="new_recipe.seasons">
                </toggle-buttons>
            </fieldset>


            <fieldset class="block">
                <legend class="label">Ingrédients</legend>
                <div class="level is-mobile mb-2">
                    <div class="level-left">
                    <button type="button" class="button is-rounded is-info is-outlined is-small level-item" @click="openNewIngredientForm">Ingrédient manquant ?</button>
                    <button type="button" class="button is-rounded is-info is-outlined is-small level-item" @click="openNewUnitForm">Unité manquante ?</button>
                    </div>
                </div>
                <ingredient-picker v-model:picked="new_recipe.ingredients">
                </ingredient-picker>
            </fieldset>

            <div class="field">
                <label class="label">Notes</label>
                <textarea v-model="new_recipe.notes" class="textarea" name="notes" id="notes"></textarea>
            </div>

            <div class="field">
                <image-chooser v-model:image_url="this.new_recipe.image_url"></image-chooser>
            </div>

            <dynamic-modal v-model:currentComponent="currentModalContent"></dynamic-modal>

            <button class="button is-primary is-large is-fullwidth">Ajouter</button>
        </form>
    </div>
</template>

<script>
import ToggleButtons from '@/components/ToggleButtons.vue'
import IngredientPicker from '@/components/IngredientPicker.vue'
import ImageChooser from '@/components/ImageChooser.vue'
import DynamicModal from '@/components/DynamicModal.vue'

export default {
    name: 'new-recipe-form',
    inject: ["store"],
    components: {
      ToggleButtons,
      IngredientPicker,
      ImageChooser,
      DynamicModal,
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
            currentModalContent: null,
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
                "n_shares": r.shares
            }
            for (var ingr of recipe.q_ingredient_ids) {
                if (ingr.quantity == null)
                    ingr.unit_id = null;
            }
            this.store.addRecipe(recipe)
        },
        openNewTagForm() {
            this.currentModalContent = "NewTag"
        },
        openNewCategoryForm() {
            this.currentModalContent = "NewCategory"
        },
        openNewIngredientForm() {
            this.currentModalContent = "NewIngredient"
        },
        openNewUnitForm() {
            this.currentModalContent = "NewUnit"
        }
    }
}
</script>

<style>
.is-circular {
    border-radius: 50% !important;
}
</style>

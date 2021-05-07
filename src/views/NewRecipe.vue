<template>
    <div class="columns is-centered is-mobile">
        <form @submit.prevent="sendRecipe" id="recipe-form" class="column is-half-desktop is-three-quarters-mobile">
            <div class="field">
                <label class="label">Nom</label>
                <div class="control">
                    <input v-model="new_recipe.name" class="input" type="text" name="name" id="name" required>
                </div>
            </div>

            <div class="field">
                <label class="label">Description</label>
                <textarea v-model="new_recipe.description" class="textarea" name="desc" id="desc"></textarea>
            </div>

            <div class="field">
                <label class="label">Temps de préparation</label>
                <div class="control">
                    <input v-model.number="new_recipe.prep_time" class="input" type="number" min=0 step="1" name="prep-time" id="prep-time">
                </div>
            </div>

            <div class="field">
                <label class="label">Temps de cuisson</label>
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
                <textarea v-model="new_recipe.instructions" class="textarea" name="cook-time" id="cook-time"></textarea>
            </div>

            <div class="field" v-if="store.state.tags.length > 0">
                <label class="label">Tags</label>
                <toggle-buttons class="my-3" :choices="store.state.tags" v-model:picked="new_recipe.tags" extendable @add="addTag">
                </toggle-buttons>
            </div>

            <div class="field" v-if="store.state.categories.length > 0">
                <label class="label">Catégories</label>
                <toggle-buttons class="my-3" :choices="store.state.categories" v-model:picked="new_recipe.categories">
                </toggle-buttons>
            </div>

            <div class="field" v-if="store.state.ingredients.length > 0">
                <label class="label">Ingrédients</label>
                <ingredient-picker v-model:picked="new_recipe.ingredients">
                </ingredient-picker>
            </div>

            <div class="field">
                <image-chooser v-model:image_url="this.new_recipe.image_url"></image-chooser>
            </div>

            <button class="button is-primary is-large is-fullwidth">Ajouter</button>
        </form>
    </div>
</template>

<script>
import ToggleButtons from '@/components/ToggleButtons.vue'
import IngredientPicker from '@/components/IngredientPicker.vue'
import ImageChooser from '@/components/ImageChooser.vue'

export default {
    name: 'new-recipe-form',
    inject: ["store"],
    components: {
      ToggleButtons,
      IngredientPicker,
      ImageChooser,
    },
    data: function() {
        return {
            new_recipe: {
                name: "",
                description: "",
                prep_time: 0,
                cook_time: 0,
                shares: 0,
                instructions: "",
                tags: new Set(),
                categories: new Set(),
                ingredients: new Map(),
                image_url: ""
            }
        }
    },
    methods: {
        sendRecipe() {
            const r = this.new_recipe;
            let recipe = {
                "name": r.name,
                "desc": r.description,
                "q_ingredient_ids": Array.from(r.ingredients.values()),
                "category_ids": Array.from(r.categories),
                "tag_ids": Array.from(r.tags),
                "prep_time_min": r.prep_time,
                "cook_time_min": r.cook_time,
                "image": r.image_url,
                "instructions": r.instructions.split(/\r?\n/),
                "n_shares": r.shares
            }
            for (var ingr of recipe.q_ingredient_ids) {
                if (ingr.quantity == null)
                    ingr.unit_id = null;
            }
            this.store.addRecipe(recipe)
        },
        async addTag(tag_name) {
            let tag = {
                "name": tag_name,
            }
            try {
                await this.store.addTag(tag)
                this.store.getTags()
            }
            catch (error) {
                console.error(error)
            }
        }
    }
}
</script>

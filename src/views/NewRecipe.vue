<template>
    <div class="columns is-centered is-mobile">
        <form @submit.prevent="send" id="recipe-form" class="column is-half-desktop is-three-quarters-mobile">
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
                    <input v-model.number="new_recipe.prep_time" class="input" type="number" name="prep-time" id="prep-time">
                </div>
            </div>

            <div class="field">
                <label class="label">Temps de cuisson</label>
                <div class="control">
                    <input v-model.number="new_recipe.cook_time" class="input" type="number" name="cook-time" id="cook-time">
                </div>
            </div>

            <div class="field">
                <label class="label">Parts</label>
                <div class="control">
                    <input v-model.number="new_recipe.shares" class="input" type="number" name="shares" id="shares">
                </div>
            </div>

            <div class="field">
                <label class="label">Etapes</label>
                <textarea v-model="new_recipe.instructions" class="textarea" name="cook-time" id="cook-time"></textarea>
            </div>

            <div class="field" v-if="store.tags.length > 0">
                <label class="label">Tags</label>
                <toggle-buttons class="my-3" :choices="store.tags" v-model:picked="new_recipe.tags">
                </toggle-buttons>
            </div>

            <div class="field" v-if="store.categories.length > 0">
                <label class="label">Catégories</label>
                <toggle-buttons class="my-3" :choices="store.categories" v-model:picked="new_recipe.categories">
                </toggle-buttons>
            </div>

            <div class="field" v-if="store.ingredients.length > 0">
                <label class="label">Ingrédients</label>
                <ingredient-picker v-model:picked="new_recipe.ingredients">
                </ingredient-picker>
            </div>

            <div class="field">
            <button @click="imageWidget.open()" type="button" class="cloudinary-button button is-fullwidth" id="upload_widget">Photo</button>
            </div>
            <div class="img-preview">
            </div>

            <button class="button is-primary is-large is-fullwidth">Ajouter</button>
        </form>
    </div>
</template>

<script>
import ToggleButtons from '@/components/ToggleButtons.vue'
import IngredientPicker from '@/components/IngredientPicker.vue'
import store from '@/store/store.js'

export default {
    name: 'new-recipe-form',
    components: {
      ToggleButtons,
      IngredientPicker
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
            },
            imageWidget: this.createImageWidget(),
            store: store
        }
    },

    methods: {
        send() {
            console.log("Sending");
            const r = this.new_recipe;
            const url = 'http://127.0.0.1/api/v1/recipes';
            const options = {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                body: JSON.stringify({
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
                })
            };
            console.log(options.body);
            fetch(url, options)
            .then(response => {
                console.log(response.status);
            });
        },
        createImageWidget() {
            return window.cloudinary.createUploadWidget({
                cloudName: 'pickeat',
                uploadPreset: 'devel1',
                cropping: true,
                thumbnails: '.img-preview',
                showAdvanced_options: true},
                (error, result) => {
                    if (result.event == "success") {
                        this.new_recipe.image_url = result.info.secure_url
                        console.log(result)
                    }
                }
            )
        }
    }
}
</script>

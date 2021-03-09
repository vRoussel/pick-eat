<template>
    <form @submit.prevent="send" id="recipe-form" >
        <div class="field">
            <label class="label">Nom</label>
            <div class="control">
                <input v-model="new_recipe.name" class="input" type="text" name="name" id="name" required>
            </div>
        </div>

        <div class="field">
            <label class="label">Description</label>
                <textarea v-model="new_recipe.description" class="textarea" name="desc" id="desc"></textarea>
            <div class="control">
            </div>
        </div>

        <div class="field">
            <label class="label">Temps de préparation</label>
                <input v-model.number="new_recipe.prep_time" class="input" type="number" name="prep-time" id="prep-time">
            <div class="control">
            </div>
        </div>

        <div class="field">
            <label class="label">Temps de cuisson</label>
                <input v-model.number="new_recipe.cook_time" class="input" type="number" name="cook-time" id="cook-time">
            <div class="control">
            </div>
        </div>

        <div class="field">
            <label class="label">Parts</label>
                <input v-model.number="new_recipe.shares" class="input" type="number" name="shares" id="shares">
            <div class="control">
            </div>
        </div>

        <div class="field">
            <label class="label">Etapes</label>
                <textarea v-model="new_recipe.instructions" class="textarea" name="cook-time" id="cook-time"></textarea>
            <div class="control">
            </div>
        </div>


        <div class="field" v-if="ingredients.length > 0">
            <fieldset id="ingredients" class="box">
                <legend class="subtitle">Ingrédients</legend>
                <template v-for="ingredient in ingredients" :key="ingredient.id">
                    <input type="checkbox" v-model="new_recipe.ingredients" :value="picked_ingredient(ingredient)" :id="ingredient.name">
                    <label class="checkbox" :for="ingredient.name">{{ ingredient.name }}</label>
                    <input v-if="ingredient.id in picked_ingredients" v-model.number="picked_ingredients[ingredient.id].quantity" class="input" type="number" name="count" id="count">
                    <select v-if="ingredient.id in picked_ingredients" v-model="picked_ingredients[ingredient.id].unit_id">
                        <option v-for="unit in units" :value="unit.id" :key="unit.id">{{ unit.full_name }}</option>
                    </select>
                </template>
            </fieldset>
        </div>

        <div class="field">
        <button @click="imageWidget.open()" type="button" class="cloudinary-button" id="upload_widget">Photo</button>
        </div>
        <div class="yoyoyo">
        </div>

        <button class="button is-primary is-large">Ajouter</button>
    </form>

            <div class="field" v-if="tags.length > 0">
                <legend class="subtitle">Tags</legend>
                <toggle-buttons :choices="tags" v-model:picked="new_recipe.tags">
                </toggle-buttons>
            </div>

            <div class="field" v-if="categories.length > 0">
                <legend class="subtitle">categories</legend>
                <toggle-buttons :choices="categories" v-model:picked="new_recipe.categories">
                </toggle-buttons>
            </div>
</template>

<script>
import ToggleButtons from './ToggleButtons.vue'

export default {
    name: 'new-recipe-form',
    components: {
      ToggleButtons
    },
    props: {
        categories: {
            type: Array,
            default: () => []
        },
        tags: {
            type: Array,
            default: () => []
        },
        ingredients: {
            type: Array,
            default: () => []
        },
        units: {
            type: Array,
            default: () => []
        }
    },

    data: function() {
        return {
            //TODO should this exactly match expected post structure ?
            // or should we map somehow in send func
            new_recipe: {
                name: "",
                description: "",
                prep_time: 0,
                cook_time: 0,
                shares: 0,
                instructions: "",
                tags: new Set(),
                categories: new Set(),
                ingredients: [],
                image_url: ""
            },
            imageWidget: this.createImageWidget()
        }
    },

    computed: {
        picked_ingredients() {
            var result = this.new_recipe.ingredients.reduce(function(map, obj) {
                map[obj.id] = obj;
                return map;
            }, {});
            return result;
        }
    },

    methods: {
        picked_ingredient (ingr) {
            return this.picked_ingredients[ingr.id] || {
                id: ingr.id,
                unit_id: ingr.default_unit.id,
                quantity: ""
            }
        },
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
                    "q_ingredient_ids": r.ingredients,
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
                thumbnails: '.yoyoyo',
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

<!-- Add "scoped" attribute to limit CSS to this component only -->
<!--
<style scoped>
#recipe-form fieldset {
        grid-column: 1/-1;
        display: grid;
        grid-template-columns: repeat(auto-fit, 20px 200px );
        align-items: center;
    }

#recipe-form fieldset#ingredients {
        grid-column: 1/-1;
        display: grid;
        grid-template-columns: 20px max-content 100px 100px;
        align-items: center;
        grid-column-gap: 10px;
    }

#recipe-form fieldset#ingredients input[type="checkbox"] {
        grid-column-start: 1;
    }

#recipe-form fieldset label {
        justify-self: start;
    }

#recipe-form fieldset input[type="checkbox"] {
        justify-self: end;
        margin-right: 10px;
    }
</style>
-->

<template>
    <form @submit.prevent="sendCategory" class="column is-half-desktop is-three-quarters-mobile" autocomplete="off">
    <div class="box">
        <div class="field">
            <label class="label">Nom de la catégorie</label>
            <input v-model="name" class="input" type="text" name="name" id="name" ref="categName" required>
        </div>
        <div class="field is-grouped">
          <div class="control">
            <button class="button is-primary" type="submit">Ok</button>
          </div>
          <div class="control">
            <button class="button" type="button" @click="cancel">Annuler</button>
          </div>
        </div>
    </div>
    </form>
</template>

<script>
export default {
    name: 'new-category',
    inject: ["store"],
    data: function() {
        return {
            name: null,
        }
    },
    methods: {
        cancel() {
            this.$emit('done')
        },
        sendCategory() {
            let category = {
                "name": this.name,
            }
            this.store.addCategory(category)
                .catch((e) => console.error(e))
                .then((new_categ) => {
                    this.$emit('created', new_categ)
                    this.$emit('done')
                })
        },
    },
    mounted() {
        this.$refs.categName.focus()
    },
    emits: ['done', 'created']
}
</script>

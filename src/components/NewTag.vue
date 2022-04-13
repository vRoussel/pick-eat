<template>
    <form @submit.prevent="sendTag" class="column is-half-desktop is-three-quarters-mobile" autocomplete="off">
    <div class="box">
        <div class="field">
            <label class="label">Nom du tag</label>
            <input v-model="name" class="input" type="text" name="name" id="name" ref="tagName" required>
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
    name: 'new-tag',
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
        sendTag() {
            let tag = {
                "name": this.name,
            }
            this.store.addTag(tag)
                .catch((e) => console.error(e))
                .then((new_tag) => {
                    this.$emit('created', new_tag)
                    this.$emit('done')
                })
        },
    },
    mounted() {
        this.$refs.tagName.focus()
    },
    emits: ['done', 'created']
}
</script>

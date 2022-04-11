<template>
    <form @submit.prevent="sendUnit" class="column is-half-desktop is-three-quarters-mobile" autocomplete="off">
    <div class="box">
        <div class="field">
            <label class="label">Nom complet</label>
            <input v-model="full_name" class="input" type="text" name="full_name" id="full_name" ref="unitName" required>
        </div>
        <div class="field">
            <label class="label">Nom court</label>
            <input v-model="short_name" class="input" type="text" name="short_name" id="short_name" required>
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
    name: 'new-unit',
    inject: ["store"],
    data: function() {
        return {
            full_name: this.input,
            short_name: null,
        }
    },
    props: {
        input: null
    },
    methods: {
        cancel() {
            this.$emit('done')
        },
        sendUnit() {
            let unit = {
                "full_name": this.full_name,
                "short_name": this.short_name,
            }
            this.store.addUnit(unit)
                .catch((e) => console.error(e))
            this.$emit('done')
        },
    },
    mounted() {
        this.$refs.unitName.focus()
    },
    emits: ['done']
}
</script>

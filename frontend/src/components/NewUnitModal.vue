<template>
    <input v-model="modal_opened" type="checkbox" :id="modal_id" class="modal-toggle" />
    <div class="modal" @click.self="this.modal_opened=false" @keyup.esc.stop="this.modal_opened=false" tabindex="-1">
        <div class="modal-box relative overflow-visible">
            <form @submit.prevent="sendUnit" autocomplete="off">
                <div class="form-control w-full">
                    <label class="label">
                        <span class="label-text">Nom complet</span>
                    </label>
                    <input v-model="full_name" class="input input-bordered w-full" type="text" name="full_name" id="full_name" ref="unitName" required>
                </div>
                <div class="form-control w-full">
                    <label class="label">
                        <span class="label-text">Nom court</span>
                    </label>
                    <input v-model="short_name" class="input input-bordered w-full" type="text" name="short_name" id="short_name" required>
                </div>
            </form>
            <div class="modal-action">
                <label :for="modal_id" class="btn btn-primary btn-sm btn-wide mx-auto" @click="sendUnit">Ajouter</label>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: 'new-unit-modal',
    inject: ["store"],
    data: function() {
        return {
            full_name: this.input,
            short_name: null,
            modal_opened: false
        }
    },
    props: {
        modal_id: {
            required: true
        },
        input: null
    },
    methods: {
        sendUnit() {
            let unit = {
                "full_name": this.full_name,
                "short_name": this.short_name,
            }
            this.store.addUnit(unit)
                .catch((e) => console.error(e))
                .then((new_unit) => {
                    this.$emit('created', new_unit)
                    this.modal_opened = false
                })
        }
    },
    watch: {
        input: function() {
            this.full_name = this.input;
        },
        modal_opened(val) {
            if (val) {
                this.$refs.unitName.focus()
            } else {
                this.full_name = null
                this.short_name = null
                this.$emit('closed')
            }
        }
    },
    emits: ['closed', 'created']
}
</script>

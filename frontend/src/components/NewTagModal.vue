<template>
    <input v-model="modal_opened" type="checkbox" :id="modal_id" class="modal-toggle" />
    <div class="modal" @click.self="this.modal_opened=false" @keyup.esc.stop="this.modal_opened=false" tabindex="-1">
        <div class="modal-box relative overflow-visible">
            <form @submit.prevent="sendTag" autocomplete="off">
                <div class="form-control w-full">
                    <label class="label">
                        <span class="label-text">Nom</span>
                    </label>
                    <input v-model="name" class="input input-bordered w-full" type="text" name="name" id="name" ref="tagName" required>
                </div>
            </form>
            <div class="modal-action">
                <label :for="modal_id" class="btn btn-primary btn-sm btn-wide mx-auto" @click="sendTag">Ajouter</label>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: 'new-tag-modal',
    inject: ["store"],
    data: function() {
        return {
            name: null,
            modal_opened: false
        }
    },
    props: {
        modal_id: {
            required: true
        }
    },
    methods: {
        sendTag() {
            let tag = {
                "name": this.name,
            }
            this.store.addTag(tag)
                .catch((e) => console.error(e))
                .then((new_tag) => {
                    this.$emit('created', new_tag)
                    this.modal_opened = false
                })
        },
    },
    watch: {
        modal_opened(val) {
            if (val) {
                this.$refs.tagName.focus()
            } else {
                this.name = ""
                this.$emit('closed')
            }
        }
    },
    emits: ['closed', 'created']
}
</script>

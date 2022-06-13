<template>
    <div class="buttons">
        <template v-for="el in choices" :key="el.id">
            <button type="button" :class="buttonClass(el)" @click="toggle(el)">{{ el.name }}</button>
        </template>
        <button v-if="extendComponent" type="button" class="button is-circular is-primary is-outlined" @click="openModal">+</button>
    </div>
    <dynamic-modal v-if="extendComponent" ref="modal">
        <component :is="extendComponent" @done="closeModal" @created="created"></component>
    </dynamic-modal>
</template>

<script>
import DynamicModal from '@/components/DynamicModal.vue'
export default {
    name: 'toggle-buttons',
    components : {
        DynamicModal
    },
    props: {
        choices: {
            type: Array
        },
        picked: {
            type: Set
        },
        extendable: {
            type: Boolean
        },
        extendComponent : {
            type: Object
        },
    },
    emits: ['update:picked', 'addChoice'],
    methods: {
        buttonClass(el) {
            return {
                "button": true,
                "is-rounded": true,
                "is-primary": this.picked.has(el.id)
            }
        },
        toggle(el) {
            if (this.picked.has(el.id))
                this.picked.delete(el.id)
            else
                this.picked.add(el.id)
            this.$emit('update:picked', this.picked)
        },
        openModal() {
            this.$refs.modal.open()
        },
        closeModal() {
            this.$refs.modal.close()
        },
        created(new_choice) {
            this.toggle(new_choice)
        }
    },
}
</script>

<style>
.is-circular {
    border-radius: 50%;
}
</style>

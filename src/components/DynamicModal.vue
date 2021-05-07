<template is-clipped>
    <div class="modal has-overflow" :class="{'is-active': isActive}">
        <div class="modal-background" @click="close"></div>
        <div class="modal-content columns is-centered is-mobile">
            <component :is="currentComponent" @done="close"></component>
        </div>
    </div>
</template>

<script>
import NewTag from '@/components/NewTag.vue'
import NewCategory from '@/components/NewCategory.vue'
import NewIngredient from '@/components/NewIngredient.vue'
import NewUnit from '@/components/NewUnit.vue'

export default {
    name: 'dynamic-modal',
    components: {
        NewTag,
        NewCategory,
        NewIngredient,
        NewUnit,
    },
    props: {
        currentComponent: null,
    },
    methods: {
        close() {
            this.$emit('update:currentComponent', null)
        },
    },
    computed: {
        isActive: function() {
            return this.currentComponent != null
        },
    },
    emits: ['update:currentComponent']

}
</script>

<template is-clipped>
    <div class="modal has-overflow" :class="{'is-active': isActive}">
        <div class="modal-background" @click="close"></div>
        <div class="modal-content columns is-centered is-mobile">
            <component :is="currentComponent" :input="input" @done="close"></component>
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
    data: function() {
        return {
            currentComponent: null,
            input: null
        }
    },
    methods: {
        openNewTagForm() {
            this.currentComponent = "NewTag"
        },
        openNewCategoryForm() {
            this.currentComponent = "NewCategory"
        },
        openNewIngredientForm(input) {
            this.input = input
            this.currentComponent = "NewIngredient"
        },
        openNewUnitForm(input) {
            this.input = input
            this.currentComponent = "NewUnit"
        },
        close() {
            this.currentComponent = null
            this.input = null
        },
    },
    computed: {
        isActive: function() {
            return this.currentComponent != null
        },
    },
}
</script>


<style>
.modal.has-overflow {
  position: fixed !important;
  overflow: auto !important;
}
.modal.has-overflow .modal-background {
    position: fixed !important;
}
.modal.has-overflow .modal-content {
    overflow: visible !important;
}
</style>


<template is-clipped>
    <div class="modal has-overflow" :class="{'is-active': active}">
        <div class="modal-background" @click="close"></div>
        <div class="modal-content columns is-centered is-mobile">
            <slot v-if="active"></slot>
        </div>
    </div>
</template>

<script>
import {modal_stack} from '@/utils/utils.js'

export default {
    name: 'dynamic-modal',
    data: function() {
        return {
            active: false,
        }
    },
    methods: {
        open() {
            this.active = true
            modal_stack.push(this)
        },
        close() {
            this.active = false
            modal_stack.pop()
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


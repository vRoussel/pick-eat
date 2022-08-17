<template>
<!--
    <div class="modal has-overflow" :class="{'is-active': active}">
        <div class="modal-background" @click="close"></div>
        <div class="modal-content columns is-centered is-mobile">
            <slot v-if="active"></slot>
        </div>
    </div>
-->
    <input type="checkbox" :id="modal_id" class="modal-toggle" />
    <div class="modal">
        <div class="modal-box">
            <slot></slot>
            <div class="modal-action">
              <label :for="modal_id" class="btn">{{ yes_button_text }}</label>
              <label :for="modal_id" class="btn">{{ yes_button_text }}</label>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: 'dynamic-modal',
    props: {
        modal_id: {
            required: true
        },
        yes_button_text: null,
        no_button_text: null,
    }
}
</script>

<script setup>
import { ref } from 'vue';
import { object, string } from 'yup'

import { useFoodStore } from '@/store/food.js'
import { useNotifStore } from '@/store/notif.js'

import { handle_form_api_errors, handle_form_local_errors } from '@/utils/utils.js'

const foodStore = useFoodStore()
const notifStore = useNotifStore()

const emit = defineEmits(['closed', 'created'])

const fields = ref({
    name: null
})

const errors = ref({
    name: null
})

const validator = object().shape({
    name: string().required('Le nom du tag est obligatoire'),
})

function sendTag() {
    errors.value = {}
    validator
        .validate(fields.value, { abortEarly: false })
        .then(() => {
            let tag = {
                name: fields.value.name,
            }
            foodStore
                .sendNewTag(tag)
                .then((new_tag) => {
                    emit('created', new_tag)
                    close()
                })
                .catch((err) => {
                    handle_form_api_errors(err.response, errors.value, notifStore)
                })
        })
        .catch((err) => {
            handle_form_local_errors(err.inner, errors.value, notifStore)
        })
}

const opened = ref(false)
const tag_name_input_el = ref(null)
function open() {
    opened.value = true
    setTimeout(() => {
        tag_name_input_el.value.focus()
        errors.value = {}
    }, 50)
}

function close() {
    opened.value = false
    fields.value.name = ''
    emit('closed')
}

function validate(field) {
    validator
        .validateAt(field, fields.value)
        .then(() => {
            errors.value[field] = null
        })
        .catch((err) => {
            setTimeout(() => (errors.value[field] = err.message), 200)
        })
}
defineExpose({ open });
</script>

<template>
    <div class="modal cursor-pointer" :class="{ 'modal-open': opened }" tabindex="-1" @click.self="close"
        @keyup.esc.stop="close">
        <div class="modal-box relative overflow-visible cursor-default">
            <form autocomplete="off" @submit.prevent="sendTag">
                <div class="form-control w-full">
                    <label class="label">
                        <span class="label-text">Nom</span>
                    </label>
                    <input id="name" ref="tag_name_input_el" v-model="fields.name" class="input input-bordered w-full"
                        type="text" name="name" :class="errors.name && '!input-error'" @blur="validate('name')" />
                    <label v-if="errors.name" class="label">
                        <span class="label-text-alt text-error">{{ errors.name }}</span>
                    </label>
                </div>
                <div class="modal-action">
                    <button class="btn btn-primary btn-sm btn-wide mx-auto">Ajouter</button>
                </div>
            </form>
        </div>
    </div>
</template>

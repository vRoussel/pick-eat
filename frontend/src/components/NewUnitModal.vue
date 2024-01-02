<script setup>
import { ref, watch } from 'vue';
import { object, string } from 'yup'

import { useFoodStore } from '@/store/food.js'
import { useNotifStore } from '@/store/notif.js'

import { handle_form_api_errors, handle_form_local_errors } from '@/utils/utils.js'

const foodStore = useFoodStore()
const notifStore = useNotifStore()

const emit = defineEmits(['closed', 'created'])

const props = defineProps({
    input: String
});

const fields = ref({
    full_name: props.input,
    short_name: null
})

const errors = ref({
    full_name: null,
    short_name: null
})

const validator = object().shape({
    full_name: string().required("Le nom complet de l'unité est obligatoire"),
})

watch(() => fields.full_name, () => {
    fields.value.full_name = props.input
})

function sendUnit() {
    errors.value = {}
    validator
        .validate(fields.value, { abortEarly: false })
        .then(() => {
            let unit = {
                full_name: fields.value.full_name,
                short_name: fields.value.short_name || fields.value.full_name,
            }
            foodStore
                .sendNewUnit(unit)
                .then((new_unit) => {
                    emit('created', new_unit)
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

const opened = ref(null)
const unit_name_input_el = ref(null)
function open() {
    opened.value = true
    setTimeout(() => {
        unit_name_input_el.value.focus()
        errors.value = {}
    }, 50)
}

function close() {
    opened.value = false
    fields.value.full_name = null
    fields.value.short_name = null
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
defineExpose({ open })
</script>

<template>
    <div class="modal cursor-pointer" :class="{ 'modal-open': opened }" tabindex="-1" @click.self="close"
        @keyup.esc.stop="close">
        <div class="modal-box relative overflow-visible cursor-default">
            <form autocomplete="off" @submit.prevent="sendUnit">
                <div class="form-control w-full">
                    <label class="label">
                        <span class="label-text">Nom complet</span>
                    </label>
                    <input id="full_name" ref="unit_name_input_el" v-model="fields.full_name"
                        class="input input-bordered w-full" type="text" name="full_name"
                        :class="errors.full_name && '!input-error'" @blur="validate('full_name')" />
                    <label v-if="errors.full_name" class="label">
                        <span class="label-text-alt text-error">{{ errors.full_name }}</span>
                    </label>
                </div>
                <div class="form-control w-full">
                    <label class="label">
                        <span class="label-text">Abréviation</span>
                    </label>
                    <input id="short_name" v-model="fields.short_name" class="input input-bordered w-full" type="text"
                        name="short_name" :class="errors.short_name && '!input-error'" />
                    <label v-if="errors.short_name" class="label">
                        <span class="label-text-alt text-error">{{ errors.short_name }}</span>
                    </label>
                </div>
                <div class="modal-action">
                    <button class="btn btn-primary btn-sm btn-wide mx-auto">Ajouter</button>
                </div>
            </form>
        </div>
    </div>
</template>

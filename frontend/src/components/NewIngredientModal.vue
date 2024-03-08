<script setup>
import { ref, watch } from 'vue'
import { object, string } from 'yup'

import { useFoodStore } from '@/store/food.js'
import { useNotifStore } from '@/store/notif.js'
import Multiselect from '@vueform/multiselect'
import NewUnitModal from '@/components/NewUnitModal.vue'

import { handle_form_api_errors, handle_form_local_errors } from '@/utils/utils.js'

const foodStore = useFoodStore()
const notifStore = useNotifStore()

const emit = defineEmits(['closed', 'created'])

const props = defineProps({
    input: String,
})

const fields = ref({
    name: props.input,
    default_unit: null,
})

const errors = ref({
    name: null,
})

const validator = object().shape({
    name: string().required("Le nom de l'ingrédient est obligatoire"),
})

watch(
    () => props.input,
    () => {
        fields.value.name = props.input
    },
)

function sendIngredient() {
    errors.value = {}
    validator
        .validate(fields.value, { abortEarly: false })
        .then(() => {
            let ingredient = {
                name: fields.value.name,
                default_unit_id: fields.value.default_unit,
            }
            foodStore
                .sendNewIngredient(ingredient)
                .then((new_ingr) => {
                    emit('created', new_ingr)
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

const unit_search = ref(null)
const multiselect_el = ref(null)
function save_unit_search() {
    unit_search.value = multiselect_el.value.search
}

function set_unit(unit) {
    fields.value.default_unit = unit.id
}

const opened = ref(false)
const ingr_name_input_el = ref(null)
function open() {
    opened.value = true
    setTimeout(() => {
        ingr_name_input_el.value.focus()
        errors.value = {}
    }, 50)
}

function close() {
    opened.value = false
    fields.value.name = null
    fields.value.default_unit = null
    unit_search.value = null
    emit('closed')
}

const unit_modal_inner_el = ref(null)
function open_unit_modal() {
    unit_modal_inner_el.value.open()
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
    <div
        class="modal cursor-pointer"
        :class="{ 'modal-open': opened }"
        tabindex="-1"
        @click.self="close"
        @keyup.esc.stop="close"
    >
        <div class="modal-box relative overflow-visible cursor-default">
            <form autocomplete="off" @submit.prevent="sendIngredient">
                <div class="form-control w-full">
                    <label class="label">
                        <span class="label-text">Nom</span>
                    </label>
                    <input
                        id="name"
                        ref="ingr_name_input_el"
                        v-model="fields.name"
                        class="input input-bordered w-full"
                        type="text"
                        name="name"
                        :class="errors.name && '!input-error'"
                        @blur="validate('name')"
                    />
                    <label v-if="errors.name" class="label">
                        <span class="label-text-alt text-error">{{ errors.name }}</span>
                    </label>
                </div>
                <div class="form-control w-full">
                    <label for="" class="label">
                        <span class="label-text">Unité par défaut</span>
                        <button
                            class="btn rounded-full btn-primary btn-outline btn-sm modal-button"
                            type="button"
                            tabindex="-1"
                            @mousedown="save_unit_search"
                            @click="open_unit_modal"
                        >
                            Unité manquante ?
                        </button>
                    </label>
                    <multiselect
                        ref="multiselect_el"
                        v-model="fields.default_unit"
                        :options="foodStore.units"
                        :strict="false"
                        label="full_name"
                        searchable
                        track-by="full_name"
                        value-prop="id"
                        @keydown.ctrl.enter.prevent="save_unit_search(), open_unit_modal()"
                    />
                </div>
                <div class="modal-action">
                    <button class="btn btn-primary btn-sm btn-wide mx-auto">Ajouter</button>
                </div>
            </form>
        </div>
    </div>
    <new-unit-modal
        ref="unit_modal_inner_el"
        :input="unit_search"
        @created="set_unit"
        @closed="ingr_name_input_el.focus()"
    />
</template>

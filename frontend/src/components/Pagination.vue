<script setup>
import { useRoute } from 'vue-router'

const route = useRoute()
const current_page = defineModel('current_page', {
    type: Number,
    required: true,
})
const props = defineProps({
    max_page: {
        type: Number,
    },
    min_page: {
        type: Number,
        default: 1,
    },
    page_offset: {
        default: 3,
    },
})

function go_page(page) {
    if (page === null || page === current_page.value) return
    current_page.value = page
}

function page(position) {
    if (current_page.value < props.min_page + props.page_offset) return position
    else if (current_page.value > props.max_page - props.page_offset)
        return props.max_page + position - 5
    else return current_page.value + position - props.page_offset
}

function buttonClass(page) {
    return {
        btn: true,
        'w-9': true,
        'sm:w-10': true,
        'lg:w-16': true,
        'btn-active': page == current_page.value,
        'cursor-default': page == current_page.value,
        'btn-disabled': page == '...',
        'btn-primary': true,
        '!btn-outline': page != current_page.value,
        'border-x-0': true,
        'border-l': page == props.min_page,
        'border-r': page == props.max_page,
    }
}
</script>

<template>
    <nav class="join" role="navigation" aria-label="pagination">
        <template v-if="current_page >= props.min_page + props.page_offset">
            <router-link
                :to="{ query: { ...route.query, page: page(props.min_page), ns: undefined } }"
                :class="buttonClass(props.min_page)"
            >
                {{ props.min_page }}
            </router-link>
            <button :class="buttonClass('...')">...</button>
        </template>

        <template v-for="i in Math.min(props.max_page, 5)">
            <router-link
                :to="{ query: { ...route.query, page: page(i), ns: undefined } }"
                class="join-item"
                :v-if="props.max_page >= i"
                :class="buttonClass(page(i))"
            >
                {{ page(i) }}
            </router-link>
        </template>

        <template v-if="current_page <= props.max_page - props.page_offset">
            <button :class="buttonClass('...')">...</button>
            <router-link
                :class="buttonClass(props.max_page)"
                :to="{ query: { ...route.query, page: page(props.max_page), ns: undefined } }"
            >
                {{ props.max_page }}
            </router-link>
        </template>
    </nav>
</template>

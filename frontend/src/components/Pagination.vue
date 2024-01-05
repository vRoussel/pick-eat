<script setup>

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
    }
});

function go_page(page) {
    if (page === null || page === current_page.value)
        return
    current_page.value = page
}

function page(position) {
    if (current_page.value < props.min_page + props.page_offset)
        return position
    else if (current_page.value > props.max_page - props.page_offset)
        return props.max_page + position - 5
    else return current_page.value + position - props.page_offset
}

function buttonClass(page) {
    return {
        btn: true,
        'join-item': props.max_page > 1,
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
            <button :class="buttonClass(props.min_page)" @click="go_page(props.min_page)">
                {{ props.min_page }}
            </button>
            <button :class="buttonClass('...')">...</button>
        </template>
        <button v-if="props.max_page >= 1" :class="buttonClass(page(1))" @click="go_page(page(1))">
            {{ page(1) }}
        </button>
        <button v-if="props.max_page >= 2" :class="buttonClass(page(2))" @click="go_page(page(2))">
            {{ page(2) }}
        </button>
        <button v-if="props.max_page >= 3" :class="buttonClass(page(3))" @click="go_page(page(3))">
            {{ page(3) }}
        </button>
        <button v-if="props.max_page >= 4" :class="buttonClass(page(4))" @click="go_page(page(4))">
            {{ page(4) }}
        </button>
        <button v-if="props.max_page >= 5" :class="buttonClass(page(5))" @click="go_page(page(5))">
            {{ page(5) }}
        </button>
        <template v-if="current_page <= props.max_page - props.page_offset">
            <button :class="buttonClass('...')">...</button>
            <button :class="buttonClass(props.max_page)" @click="go_page(props.max_page)">
                {{ props.max_page }}
            </button>
        </template>
    </nav>
</template>


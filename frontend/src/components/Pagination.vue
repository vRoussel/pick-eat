<template>
<nav class="btn-group" role="navigation" aria-label="pagination">
    <template v-if="current_page >= min_page + page_offset">
        <button :class="buttonClass(min_page)" @click="go_page(min_page)">{{ min_page }}</button>
        <button :class="buttonClass('...')">...</button>
    </template>
    <button v-if="max_page >= 1" :class="buttonClass(page(1))" @click="go_page(page(1))">{{ page(1) }}</button>
    <button v-if="max_page >= 2" :class="buttonClass(page(2))" @click="go_page(page(2))">{{ page(2) }}</button>
    <button v-if="max_page >= 3" :class="buttonClass(page(3))" @click="go_page(page(3))">{{ page(3) }}</button>
    <button v-if="max_page >= 4" :class="buttonClass(page(4))" @click="go_page(page(4))">{{ page(4) }}</button>
    <button v-if="max_page >= 5" :class="buttonClass(page(5))" @click="go_page(page(5))">{{ page(5) }}</button>
    <template v-if="current_page <= max_page - page_offset">
        <button :class="buttonClass('...')">...</button>
        <button :class="buttonClass(max_page)" @click="go_page(max_page)">{{ max_page }}</button>
    </template>
</nav>
</template>

<script>
export default {
    name: 'pagination',
    props: {
        current_page: {
            type: Number
        },
        max_page: {
            type: Number
        },
        min_page: {
            type: Number,
            default: 1
        },
        page_offset: {
            default: 3
        }

    },
    methods: {
        go_page(page) {
            if (page === null) return;
            this.$router.push({ query: { ...this.$route.query, "page": page}, params: {noscroll: false} });
        },
        page(position) {
            if (this.current_page < this.min_page + this.page_offset)
                return position
            else if (this.current_page > this.max_page - this.page_offset)
                return this.max_page + position - 5
            else
                return this.current_page + position - this.page_offset
        },
        buttonClass(page) {
            return {
                "btn": true,
                "w-9": true,
                "sm:w-10": true,
                "lg:w-16": true,
                "btn-active": page == this.current_page,
                "btn-disabled": page == this.current_page || page == '...',
                "btn-primary": true,
                "!btn-outline": page != this.current_page,
                "border-x-0": true,
                "border-l": page == this.min_page,
                "border-r": page == this.max_page,
            }
        },
    },
}
</script>

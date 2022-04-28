<template>
    <div class="field has-addons">
        <div class="control has-icons-left has-icons-right is-expanded-mobile">
            <input @input="e => query = e.target.value" class="input is-rounded" type="text" placeholder="Trouver une recette" :value="query">
            <span class="icon is-right" v-if="this.query">
               <i class="fas fa-times is-clickable" @click="this.query = null"></i>
            </span>
            <span class="icon is-left">
               <i class="fas fa-search"></i>
            </span>
        </div>
    </div>
</template>

<script>
export default {
    name: 'live-search',
    data: function() {
        return {
            query: this.initial_query,
            timer: null
        }
    },
    props: {
        initial_query: null,
        search_delay: {
            type: Number,
            default: 400
        }
    },
    methods: {
        scheduleSearch(query) {
            if (this.timer) {
                clearTimeout(this.timer)
                this.timer = null
            }
            this.timer = setTimeout(() => {
                if (query == null || query == "") {
                    this.$emit('clear')
                } else {
                    this.$emit('search', query)
                }
            }, this.search_delay)
        }
    },
    watch: {
        query: function(val) {
            this.scheduleSearch(val)
        }
    },
    emits: ['search', 'clear']
}
</script>

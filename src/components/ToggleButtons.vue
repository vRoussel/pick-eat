<template>
    <div class="buttons">
        <template v-for="el in choices" :key="el.id">
            <button type="button" :class="buttonClass(el.id)" @click="clicked(el.id)">{{ el.name }}</button>
        </template>
    </div>
</template>

<script>
export default {
    name: 'toggle-buttons',
    props: {
        choices: {
            type: Array
        },
        picked: {
            type: Set
        }
    },
    emits: ['update:picked'],
    methods: {
        buttonClass(id) {
            return {
                "button": true,
                "is-rounded": true,
                "is-success": this.picked.has(id)
            }
        },
        clicked(id) {
            if (this.picked.has(id))
                this.picked.delete(id)
            else
                this.picked.add(id)
            this.$emit('update:picked', this.picked)
        }
    }
}
</script>

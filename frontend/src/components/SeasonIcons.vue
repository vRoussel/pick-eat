<template>
    <div class="flex">
        <span class="icon">
            <ion-icon :src="require('@/assets/ionicons/flower.svg')" :class="dynClass(1)"></ion-icon>
        </span>
        <span class="icon">
            <ion-icon :src="require('@/assets/ionicons/sunny.svg')" :class="dynClass(2)"></ion-icon>
        </span>
        <span class="icon">
            <ion-icon :src="require('@/assets/ionicons/leaf.svg')" :class="dynClass(3)"></ion-icon>
        </span>
        <span class="icon">
            <ion-icon :src="require('@/assets/ionicons/snow.svg')" :class="dynClass(4)"></ion-icon>
        </span>
    </div>
</template>

<script>
export default {
    name: 'season-icons',
    props: {
        seasons: {
            type: Array
        },
    },
    methods: {
        dynClass(id) {
            return {
                "text-primary" : this.has_season(id),
                "text-accent" : !this.has_season(id),
                "opacity-30" : !this.has_season(id)
            }
        },
        has_season(id) {
            return this.season_ids.includes(id)
        },
    },
    computed: {
        season_ids() {
            return this.seasons.map(r => r.id)
        },
    }
}
</script>

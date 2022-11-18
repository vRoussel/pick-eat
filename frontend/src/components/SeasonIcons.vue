<template>
    <div class="flex">
        <span class="icon">
            <ion-icon :src="flower_svg" :class="dynClass(1)"></ion-icon>
        </span>
        <span class="icon">
            <ion-icon :src="sunny_svg" :class="dynClass(2)"></ion-icon>
        </span>
        <span class="icon">
            <ion-icon :src="leaf_svg" :class="dynClass(3)"></ion-icon>
        </span>
        <span class="icon">
            <ion-icon :src="snow_svg" :class="dynClass(4)"></ion-icon>
        </span>
    </div>
</template>

<script>
import flower_svg from '@/assets/ionicons/flower.svg'
import sunny_svg from '@/assets/ionicons/sunny.svg'
import leaf_svg from '@/assets/ionicons/leaf.svg'
import snow_svg from '@/assets/ionicons/snow.svg'
export default {
    name: 'season-icons',
    props: {
        seasons: {
            type: Array
        },
    },
    data: function() {
        return {
            flower_svg: flower_svg,
            sunny_svg: sunny_svg,
            leaf_svg: leaf_svg,
            snow_svg: snow_svg
        }
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

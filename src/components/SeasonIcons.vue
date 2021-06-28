<template>
    <p >
        <span class="icon" :class="dynClass(1)">
          <i class="fas fa-seedling"></i>
        </span>
        <span class="icon" :class="dynClass(2)">
          <i class="fas fa-sun"></i>
        </span>
        <span class="icon" :class="dynClass(3)">
          <i class="fas fa-leaf"></i>
        </span>
        <span class="icon" :class="dynClass(4)">
          <i class="fas fa-snowflake"></i>
        </span>
    </p>
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
                active : this.has_season(id),
                inactive : !this.has_season(id),
                'mx-1' : screen.width <= 760,
                'mx-3' : screen.width > 760,
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

<style>
    .inactive {
        color: lightgrey
    }
    
    .active {
        color: #BF9C53
    }

</style>

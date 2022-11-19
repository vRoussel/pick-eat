<template>
  <div class="flex">
    <span class="icon">
      <Icon
        :icon="icons.flower"
        :inline="true"
        :class="dynClass(1)"
      />
    </span>
    <span class="icon">
      <Icon
        :icon="icons.sun"
        :inline="true"
        :class="dynClass(2)"
      />
    </span>
    <span class="icon">
      <Icon
        :icon="icons.leaf"
        :inline="true"
        :class="dynClass(3)"
      />
    </span>
    <span class="icon">
      <Icon
        :icon="icons.snowflake"
        :inline="true"
        :class="dynClass(4)"
      />
    </span>
  </div>
</template>

<script>
export default {
    name: 'SeasonIcons',
    inject: ["icons"],
    props: {
        seasons: {
            type: Array
        },
    },
    computed: {
        season_ids() {
            return this.seasons.map(r => r.id)
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
    }
}
</script>

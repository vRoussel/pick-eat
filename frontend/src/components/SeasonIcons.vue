<template>
  <div class="flex">
    <span class="icon">
      <Icon
        :icon="icons.flower"
        :inline="true"
        :class="dynClass('spring')"
      />
    </span>
    <span class="icon">
      <Icon
        :icon="icons.sun"
        :inline="true"
        :class="dynClass('summer')"
      />
    </span>
    <span class="icon">
      <Icon
        :icon="icons.leaf"
        :inline="true"
        :class="dynClass('fall')"
      />
    </span>
    <span class="icon">
      <Icon
        :icon="icons.snowflake"
        :inline="true"
        :class="dynClass('winter')"
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
        season_labels() {
            return this.seasons.map(r => r.label)
        },
    },
    methods: {
        dynClass(label) {
            return {
                "text-primary" : this.has_season(label),
                "text-accent" : !this.has_season(label),
                "opacity-30" : !this.has_season(label)
            }
        },
        has_season(label) {
            return this.season_labels.includes(label)
        },
    }
}
</script>

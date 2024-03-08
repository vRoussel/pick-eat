<script setup>
import { useHead } from '@unhead/vue'

import no_recipe_video from '@/assets/homer_not_found.mp4'

useHead({
    title: '404 not found',
    meta: {
        name: 'robots',
        content: 'noindex',
    },
})
</script>

<template>
    <div>
        <p class="text-xl mb-2">Oups, cette page n'existe pas</p>
        <video :src="no_recipe_video" autoplay loop muted playsinline />
    </div>
</template>

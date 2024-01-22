<script setup>
import { inject, ref, computed } from 'vue'

import { loadScript } from '@/utils/utils.js'

const icons = inject('icons')

const props = defineProps({
    image_url: String,
});

const emit = defineEmits(['update:image_url'])
const image_widget = ref(createImageWidget())
const image_preview = computed(() => {
    if (props.image_url === '')
        return icons.camera
    else
        return props.image_url.replace('/upload', '/upload/c_limit,h_512,w_512')
})

function createImageWidget() {
    loadScript('https://upload-widget.cloudinary.com/global/all.js', () => {
        image_widget.value = window.cloudinary.createUploadWidget(
            {
                cloudName: 'pickeat',
                uploadPreset: 'devel1',
                cropping: true,
                multiple: false,
                showSkipCropButton: false,
                croppingAspectRatio: 1.0,
                maxFileSize: 10000000,
                maxImageWidth: 2000,
                thumbnails: false,
                croppingShowDimensions: true,
            },
            (error, result) => {
                if (result.event == 'success') {
                    emit('update:image_url', result.info.secure_url)
                }
            },
        )
    })
}

function clear() {
    emit('update:image_url', '')
}
</script>

<template>
    <div class="relative w-fit mx-auto">
        <img class="cursor-pointer m-auto rounded-lg w-80" :src="image_preview" @click="image_widget.open()" />
        <span v-if="props.image_url != ''" class="icon text-2xl cursor-pointer text-black absolute right-2 top-2"
            @click="clear">
            <Icon :icon="icons.close" />
        </span>
    </div>
</template>

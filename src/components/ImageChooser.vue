<template>
<figure class="image container is-256x256">
    <img @click="imageWidget.open()" class="is-clickable" :src="image_preview">
</figure>
</template>

<script>
import {PLACEHOLDER_IMG} from '@/utils/utils.js'

export default {
    name: 'image-chooser',
    props: {
        image_url: {
            type: String
        }
    },
    data: function() {
        return {
            imageWidget: this.createImageWidget(),
        }
    },
    emits: ['update:image_url'],
    computed: {
        image_preview() {
            if (this.image_url === "")
                return PLACEHOLDER_IMG
            else
                return this.image_url.replace("/upload", "/upload/c_limit,h_256,w_256");
        },
    },
    methods: {
        createImageWidget() {
            return window.cloudinary.createUploadWidget({
                cloudName: 'pickeat',
                uploadPreset: 'devel1',
                cropping: true,
                multiple: false,
                showSkipCropButton: false,
                croppingAspectRatio: 1.0,
                maxFileSize: 10000000,
                maxImageWidth: 2000,
                thumbnails: false,
                croppingShowDimensions: true},
                (error, result) => {
                    if (result.event == "success") {
                        this.$emit('update:image_url', result.info.secure_url)
                    }
                }
            )
        }

    }
}
</script>

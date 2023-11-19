<template>
  <div class="relative w-fit mx-auto">
    <img
      class="cursor-pointer m-auto rounded-lg w-80"
      :src="image_preview"
      @click="imageWidget.open()"
    >
    <span
      v-if="image_url != ''"
      class="icon text-2xl cursor-pointer text-black absolute right-2 top-2"
      @click="clear"
    >
      <Icon :icon="icons.close" />
    </span>
  </div>
</template>

<script>
import { loadScript } from '@/utils/utils.js'

export default {
    name: 'ImageChooser',
    inject: ["icons"],
    props: {
        image_url: {
            type: String
        }
    },
    emits: ['update:image_url'],
    data: function() {
        return {
            imageWidget: null,
        }
    },
    computed: {
        image_preview() {
            if (this.image_url === "")
                return this.icons.camera
            else
                return this.image_url.replace("/upload", "/upload/c_limit,h_512,w_512");
        },
    },
    mounted() {
        if (window.cloudinary === undefined) {
            loadScript("https://upload-widget.cloudinary.com/global/all.js", () => {
                this.imageWidget = this.createImageWidget()
            })
        }
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
        },
        clear() {
            this.$emit('update:image_url', "")
        }

    }
}
</script>

<template>
    <div class="w-full h-full flex justify-center sm:justify-start my-3">
        <div class="relative">
            <img @click="imageWidget.open()" class="cursor-pointer m-auto rounded-lg w-64" :src="image_preview">
            <button @click="clear" v-if="image_url != ''" class="btn btn-sm btn-circle absolute right-2 top-2">✕</button>
        </div>
    </div>
</template>

<script>

export default {
    name: 'image-chooser',
    inject: ["icons"],
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
                return this.icons.camera
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
        },
        clear() {
            this.$emit('update:image_url', "")
        }

    }
}
</script>

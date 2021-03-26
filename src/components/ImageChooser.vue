<template>
<figure class="image container is-128x128">
    <img @click="imageWidget.open()" class="is-clickable" :src="image_preview">
</figure>
</template>

<script>
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
                return 'https://bulma.io/images/placeholders/128x128.png'
            else
                return this.image_url.replace("/upload", "/upload/c_limit,h_128,w_128");
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
                        //this.image_url = result.info.secure_url
                        this.$emit('update:image_url', result.info.secure_url)
                        console.log(result)
                    }
                }
            )
        }

    }
}
</script>

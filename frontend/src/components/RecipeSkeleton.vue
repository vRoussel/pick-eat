<template>
    <div
        class="flex flex-col w-full max-w-5xl mx-auto p-4 md:p-6 lg:p-8 xl:p-12 gap-y-12 border border-primary rounded-xl"
    >
        <div class="flex flex-wrap sm:flex-nowrap gap-y-12 gap-x-4 md:gap-x-6">
            <div
                class="basis-full sm:basis-2/5 md:basis-1/3 p-2 sm:p-0 w-[512px] aspect-square skeleton"
            ></div>
            <div
                class="flex flex-col basis-full sm:basis-1/2 justify-between items-center mx-auto gap-y-2 sm:gap-y-0"
            >
                <div class="skeleton w-8/12 h-8"></div>
                <div class="skeleton w-4/12 h-8"></div>
                <div class="flex flex-row space-x-4">
                    <div class="skeleton w-40 h-8"></div>
                    <div class="skeleton w-40 h-8"></div>
                </div>
                <div class="skeleton w-40 h-8"></div>
            </div>
        </div>
        <div
            class="flex flex-wrap sm:flex-nowrap gap-x-4 sm:gap-x-6 md:gap-x-8 items-start gap-y-12"
        >
            <div class="skeleton basis-4/12 h-48"></div>
            <div class="skeleton h-48 grow"></div>
        </div>
        <div class="skeleton w-full h-16"></div>
    </div>
</template>

<script setup>
import { ElCard, ElIcon } from 'element-plus';

    const props = defineProps({
        is_loading: {
            type: Boolean,
        }
    })
</script>

<template>
    <div class="loader center" v-show="is_loading">
            <el-icon class="is-loading"><Loading ></Loading></el-icon>
    </div>
</template>

<style scoped>
    .center {
        display: flex;    
        align-items: center;
        justify-content: center;
    }
    .loader {
        z-index: 100;
        position: absolute;
        left:0px;
        top:0px;
        width: 100%;
        height: 100vh;
        background-color: rgba(0, 0, 0, 0.37);
    }    
</style>
<template>
    <div class="rounded-tl-md rounded-tr-md grid grid-cols-2">
        <button v-if="!isView" class="selected switcher rounded-tl-md" v-on:click="setEdit()">
            <p>Edit</p>
        </button>
        <button v-if="isView" class="switcher rounded-tl-md" v-on:click="setEdit()">
            <p>Edit</p>
        </button>
        <button v-if="isView" class="selected switcher rounded-tr-md" v-on:click="setView()">
            <p>View Jsongraph</p>
        </button>
        <button v-if="!isView" class="switcher rounded-tr-md" v-on:click="setView()">
            <p>View Jsongraph</p>
        </button>
    </div>
</template>

<script>
export default {
    name: 'ViewSwitcher',
    data() {
        return {
            isView: false,
        }
    },
    methods: {
        setEdit() {
            this.isView = false;
            this.$emit('setView', false);
        },
        setView() {
            this.isView = true;
            this.$emit('setView', true);
        }
    }
}
</script>

<style scoped>
    .switcher {
        @apply text-center py-2 bg-slate-600 hover:bg-slate-500 text-xl font-semibold;
    }

    .selected {
        @apply bg-slate-700 hover:bg-slate-700;
    }
</style>
<template>
	<div class="checkbox-input">
		<input type="checkbox" :id="" />
		<label></label>
	</div>
</template>

<style lang="scss">
.checkbox-input {
	input {
		margin: 0;
		padding: 0;
		border: none;
		outline: none;
		border-radius: 2px;
		width: 16px;
		height: 16px;
	}
}
</style>
<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
	id: "",
	componentId: 0,
	created() {
		this.componentId += 1;
	},
	props: {},
	components: {},
});
</script>

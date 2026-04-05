<script lang="ts">
  import CustomCaretTextarea from "./components/CustomCaretTextarea.svelte";

  let docText = $state("");
</script>

<div>
  <CustomCaretTextarea bind:value={docText} />
</div>

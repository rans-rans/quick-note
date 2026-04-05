<script lang="ts">
  import { onMount } from "svelte";

  let {
    value = $bindable(""),
    className = ""
  }: {
    value?: string;
    className?: string;
  } = $props();

  let editorEl = $state<HTMLDivElement | null>(null);
  let caretVisible = $state(false);
  let caretTop = $state(0);
  let caretLeft = $state(0);
  let caretHeight = $state(22);

  function syncFromEditor() {
    if (!editorEl) return;
    value = editorEl.textContent ?? "";
  }

  function syncFromValue() {
    if (!editorEl) return;
    const editorText = editorEl.textContent ?? "";

    if (editorText !== value) {
      editorEl.textContent = value;
      queueCaretRefresh();
    }
  }

  function updateCaretPosition() {
    if (!editorEl) return;

    const selection = window.getSelection();
    if (!selection || selection.rangeCount === 0) {
      caretVisible = false;
      return;
    }

    const range = selection.getRangeAt(0);
    if (!range.collapsed || !editorEl.contains(range.startContainer)) {
      caretVisible = false;
      return;
    }

    const probe = range.cloneRange();
    probe.collapse(true);
    let rect = probe.getClientRects()[0] ?? probe.getBoundingClientRect();

    if (!rect || (rect.width === 0 && rect.height === 0)) {
      const marker = document.createElement("span");
      marker.textContent = "\u200b";
      probe.insertNode(marker);
      rect = marker.getBoundingClientRect();
      marker.remove();
      selection.removeAllRanges();
      selection.addRange(range);
    }

    const editorRect = editorEl.getBoundingClientRect();
    const computed = getComputedStyle(editorEl);
    const paddingTop = parseFloat(computed.paddingTop) || 0;
    const lineHeight = parseFloat(computed.lineHeight) || 22;

    caretTop = Math.max(rect.top - editorRect.top, paddingTop);
    caretLeft = Math.max(rect.left - editorRect.left, 0);
    caretHeight = Math.max(rect.height || lineHeight, lineHeight);
    caretVisible = document.activeElement === editorEl;
  }

  function queueCaretRefresh() {
    requestAnimationFrame(updateCaretPosition);
  }

  $effect(() => {
    syncFromValue();
  });

  onMount(() => {
    syncFromValue();
    queueCaretRefresh();
  });
</script>

<div class={`editor-shell ${className}`}>
  <div
    bind:this={editorEl}
    class="input-area"
    contenteditable="true"
    role="textbox"
    aria-multiline="true"
    tabindex="0"
    spellcheck="false"
    oninput={() => {
      syncFromEditor();
      queueCaretRefresh();
    }}
    onfocus={() => {
      caretVisible = true;
      queueCaretRefresh();
    }}
    onblur={() => {
      caretVisible = false;
    }}
    onkeyup={queueCaretRefresh}
    onmouseup={queueCaretRefresh}
  ></div>

  {#if caretVisible}
    <span
      class="fake-caret"
      style={`top: ${caretTop}px; left: ${caretLeft}px; height: ${caretHeight}px;`}
    ></span>
  {/if}
</div>

<style>
  .editor-shell {
    position: relative;
    height: 100vh;
    height: 100dvh;
    overflow: hidden;
  }

  .input-area {
    display: block;
    white-space: pre-wrap;
    overflow-wrap: break-word;
    padding: 0.5rem;
    overflow: auto;
    width: 100%;
    height: 100%;
    border: none;
    font-size: 16px;
    line-height: 1.5;
    caret-color: transparent;
  }

  .input-area:focus {
    outline: none;
    border: none;
  }

  .fake-caret {
    position: absolute;
    width: 3px;
    background: #111;
    pointer-events: none;
    animation: caret-blink 1s steps(1) infinite;
  }

  @keyframes caret-blink {
    0%,
    49% {
      opacity: 1;
    }
    50%,
    100% {
      opacity: 0.3;
    }
  }
</style>

<script>
  import { onMount, afterUpdate } from 'svelte';
  import Message from './Message.svelte';
  
  export let messages = [];
  
  let messageListElement;

  function scrollToBottom() {
    if (messageListElement) {
      messageListElement.scrollTop = messageListElement.scrollHeight;
    }
  }

  onMount(() => {
    scrollToBottom();
  });

  afterUpdate(() => {
    scrollToBottom();
  });
</script>

<div class="message-list" bind:this={messageListElement}>
  {#if messages.length === 0}
    <div class="empty-state">
      <div class="empty-icon">💬</div>
      <p>Start a conversation with your AI assistant</p>
    </div>
  {:else}
    {#each messages as message (message.id)}
      <Message {message} />
    {/each}
  {/if}
</div>

<style>
  .message-list {
    flex: 1;
    overflow-y: auto;
    padding: 20px 16px;
    background: #FFFFFF;
  }

  .message-list::-webkit-scrollbar {
    width: 4px;
  }

  .message-list::-webkit-scrollbar-track {
    background: transparent;
  }

  .message-list::-webkit-scrollbar-thumb {
    background: #C7C7CC;
    border-radius: 2px;
  }

  .message-list::-webkit-scrollbar-thumb:hover {
    background: #AEAEB2;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #8E8E93;
  }

  .empty-state p {
    margin: 0;
    font-size: 15px;
  }
</style>


<script>
  export let message;

  function formatTime(date) {
    return new Date(date).toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: '2-digit'
    });
  }
</script>

<div class="message {message.role}">
  <div class="message-content">
    <p>{message.content}</p>
  </div>
  <div class="message-time">
    {formatTime(message.timestamp)}
  </div>
</div>

<style>
  .message {
    margin-bottom: 20px;
    display: flex;
    flex-direction: column;
    animation: fadeIn 0.3s ease-in;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .message.user {
    align-items: flex-end;
  }

  .message.assistant {
    align-items: flex-start;
  }

  .message-content {
    max-width: 85%;
    padding: 10px 14px;
    border-radius: 16px;
    word-wrap: break-word;
  }

  .message.user .message-content {
    background: #007AFF;
    color: white;
    border-bottom-right-radius: 4px;
  }

  .message.assistant .message-content {
    background: #F2F2F7;
    color: #000000;
    border-bottom-left-radius: 4px;
  }

  .message-content p {
    margin: 0;
    font-size: 15px;
    line-height: 1.4;
  }

  .message-time {
    font-size: 11px;
    color: #8E8E93;
    margin-top: 4px;
    padding: 0 4px;
  }
</style>


<script>
  import MessageList from './MessageList.svelte';
  import MessageInput from './MessageInput.svelte';
  
  let messages = [
    {
      id: 1,
      role: 'user',
      content: 'Hello! Can you help me with my code?',
      timestamp: new Date()
    },
    {
      id: 2,
      role: 'assistant',
      content: 'Of course! I\'d be happy to help you with your code. What do you need assistance with?',
      timestamp: new Date()
    }
  ];

  function handleSendMessage(event) {
    const { message } = event.detail;
    
    // Add user message
    messages = [...messages, {
      id: messages.length + 1,
      role: 'user',
      content: message,
      timestamp: new Date()
    }];

    // Simulate AI response
    setTimeout(() => {
      const responses = [
        'I understand. Let me help you with that.',
        'That\'s a great question! Here\'s what I think...',
        'I can definitely assist you with that.',
        'Let me analyze that for you.'
      ];
      
      messages = [...messages, {
        id: messages.length + 1,
        role: 'assistant',
        content: responses[Math.floor(Math.random() * responses.length)],
        timestamp: new Date()
      }];
    }, 1000);
  }
</script>

<div class="chat-window">
  <div class="chat-header">
    <h1>Agent</h1>
  </div>
  
  <MessageList {messages} />
  
  <MessageInput on:send={handleSendMessage} />
</div>

<style>
  .chat-window {
    width: 675px; /* 1.5x wider to match request */
    height: 800px;
    background: white;
    border-radius: 16px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .chat-header {
    padding: 16px 20px;
    border-bottom: 1px solid #E5E5EA;
    background: #FAFAFA;
  }

  .chat-header h1 {
    margin: 0;
    font-size: 17px;
    font-weight: 600;
    color: #000000;
    letter-spacing: -0.4px;
  }

  @media (max-width: 500px) {
    .chat-window {
      width: 100%;
      height: 100vh;
      border-radius: 0;
    }
  }
</style>


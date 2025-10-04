<script>
  import MessageList from './MessageList.svelte';
  import MessageInput from './MessageInput.svelte';
  import { FolderOpen, Settings } from 'lucide-svelte';
  
  let messages = [
    {
      id: 1,
      role: 'user',
      content: '你好！你能帮我看看这段代码吗？',
      timestamp: new Date()
    },
    {
      id: 2,
      role: 'assistant',
      content: '当然可以！我很乐意帮助您分析代码。请告诉我您需要什么帮助？',
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
        '我明白了，让我来帮助您解决这个问题。',
        '这是一个很好的问题！让我来分析一下...',
        '我可以帮助您处理这个问题。',
        '让我为您分析一下这段代码。'
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
    <h1>代码助手</h1>
    <div class="header-actions">
      <button class="header-button" title="打开目录">
        <FolderOpen size={16} />
      </button>
      <button class="header-button" title="设置">
        <Settings size={16} />
      </button>
    </div>
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
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .chat-header h1 {
    margin: 0;
    font-size: 17px;
    font-weight: 600;
    color: #000000;
    letter-spacing: -0.4px;
  }

  .header-actions {
    display: flex;
    gap: 8px;
  }

  .header-button {
    width: 32px;
    height: 32px;
    border: none;
    background: transparent;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: #666;
    transition: all 0.2s ease;
  }

  .header-button:hover {
    background: rgba(0, 0, 0, 0.05);
    color: #333;
  }

  .header-button:active {
    transform: scale(0.95);
  }

  @media (max-width: 500px) {
    .chat-window {
      width: 100%;
      height: 100vh;
      border-radius: 0;
    }
  }
</style>


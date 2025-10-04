<script>
  import { createEventDispatcher } from 'svelte';
  import { AtSign, Hash, Image, ChevronDown, GitPullRequestCreate, ListChecks, ArrowUp } from 'lucide-svelte';

  const dispatch = createEventDispatcher();

  let message = '';
  let inputElement;

  function adjustTextareaHeight() {
    if (!inputElement) return;
    inputElement.style.height = 'auto';
    const style = getComputedStyle(inputElement);
    const lineHeight = parseFloat(style.lineHeight);
    const paddingTop = parseFloat(style.paddingTop);
    const paddingBottom = parseFloat(style.paddingBottom);
    const maxHeight = lineHeight * 3 + paddingTop + paddingBottom; // up to 3 lines
    const sh = inputElement.scrollHeight;
    if (sh > maxHeight) {
      inputElement.style.height = `${maxHeight}px`;
      inputElement.style.overflowY = 'auto';
    } else {
      inputElement.style.height = `${sh}px`;
      inputElement.style.overflowY = 'hidden';
    }
  }

  function handleSubmit() {
    if (message.trim()) {
      dispatch('send', { message: message.trim() });
      message = '';
      inputElement.focus();
      adjustTextareaHeight(); // Reset height after sending
    }
  }

  function handleKeyDown(event) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleSubmit();
    }
  }

  function handleInput() {
    adjustTextareaHeight();
  }
</script>

<div class="message-input-container">
  <div class="input-wrapper">
    <!-- 输入框区域 - 现在占据上半部分 -->
    <div class="input-container">
      <textarea
        bind:this={inputElement}
        bind:value={message}
        on:keydown={handleKeyDown}
        on:input={handleInput}
        placeholder="请输入..."
        class="input"
        rows="3"
        spellcheck="false"
      ></textarea>
    </div>
    
    <!-- 底部工具栏 - 包含所有图标 -->
    <div class="bottom-actions">
      <div class="actions-left">
        <button class="action-button" title="Add hashtag">
          <Hash size={16} />
        </button>
        <button class="action-button" title="Attach image">
          <Image size={16} />
        </button>
        <button class="action-button" title="Task list">
          <ListChecks size={16} />
        </button>
      </div>
      <div class="actions-right">
        <div class="model-selector">
          <span>Gemini-2.5-Pro</span>
          <ChevronDown size={16} style="transform: translateY(1px);" />
        </div>
        <button class="action-button append-button" title="Append task">
          <GitPullRequestCreate size={16} />
        </button>
        <button
          on:click={handleSubmit}
          class="send-button"
          class:active={message.trim()}
          disabled={!message.trim()}
          title="Send message"
        >
          <ArrowUp size={16} />
        </button>
      </div>
    </div>
  </div>
</div>

<style>
  .message-input-container {
    padding: 16px;
    background: #F0F4F9; /* New background color */
    border-top: 1px solid #D1D5DB;
  }

  .input-wrapper {
    /* 改为垂直布局，上下分布图标，中间是输入框 */
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
    background: #FFFFFF;
    border: 1px solid #D1D5DB;
    border-radius: 12px;
    box-shadow: 0 1px 2px rgba(0,0,0,0.05);
    transition: all 0.2s;
  }

  .input-wrapper:focus-within {
    border-color: #A8B3C5;
    box-shadow: 0 0 0 3px rgba(66, 133, 244, 0.1); /* Google blue shadow */
  }

  .bottom-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .actions-left {
    display: flex;
    align-items: center;
    gap: 4px; /* 减少左侧图标间距，与右侧保持一致 */
  }

  .actions-right {
    display: flex;
    align-items: center;
    gap: 4px; /* 图标之间更紧凑的间距 */
  }

  /* 模型选择器与图标之间的间距 */
  .actions-right .model-selector {
    margin-right: 8px; /* 与后面图标保持稍大距离 */
  }

  .action-button {
    width: 32px; /* 统一按钮尺寸与send-button一致 */
    height: 32px;
    border: none;
    border-radius: 8px; /* 统一圆角与send-button一致 */
    background: transparent;
    color: #5F6368; /* Google grey for icons */
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s;
    padding: 0;
  }

  .action-button:hover {
    background: #F1F3F4; /* Lighter grey on hover */
  }

  .input-container {
    /* 输入框容器现在可以占据完整宽度 */
    width: 100%;
    display: flex;
    align-items: stretch;
    min-height: calc(1.5em * 3 + 12px);
  }

  .input {
    width: 100%;
    box-sizing: border-box;
    padding: 8px 12px; /* 恢复正常的内边距，因为不再被挤压 */
    font-size: 15px;
    line-height: 1.5;
    border: none;
    background: transparent;
    outline: none;
    color: #202124;
    resize: none;
    min-height: calc(1.5em * 3); /* base three lines */
    overflow: auto; /* scroll if exceeds three lines */
    text-align: left;
  }

  .input::placeholder {
    color: #969BA1;
  }

  .model-selector {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    font-size: 12px; /* 减小字体大小，与图标按钮更协调 */
    color: #3C4043;
    background: #F8F9FA;
    padding: 4px 8px;
    border-radius: 8px;
    cursor: pointer;
    height: 32px; /* 明确设置高度与其他按钮一致 */
    box-sizing: border-box;
    line-height: 1; /* 确保文字垂直居中 */
  }

  .model-selector:hover {
    background: #F1F3F4;
  }

  .append-button {
    width: 32px; /* 确保与其他按钮尺寸一致 */
    height: 32px;
    border-radius: 8px; /* 确保圆角一致 */
    background: #F8F9FA; /* 浅灰色背景，表示操作类型 */
    color: #5F6368;
  }

  .append-button:hover {
    background: #F1F3F4;
  }

  .send-button {
    width: 32px;
    height: 32px;
    border: none;
    border-radius: 8px; /* Squarish button */
    background: #E8F0FE;
    color: #1967D2;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s;
    flex-shrink: 0;
    padding: 0;
  }

  .send-button.active {
    background: #1A73E8; /* Google blue */
    color: white;
  }

  .send-button.active:hover {
    background: #185ABC;
  }
</style>


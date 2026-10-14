<script setup lang="ts">
import { ref } from 'vue'

const emit = defineEmits<{
  (e: 'file', file: File): void
}>()

const dragging = ref(false)
const selectedFile = ref<File | null>(null)
const error = ref('')

const ACCEPTED = ['.pdf', '.docx']
const ACCEPT_MIME = ['application/pdf', 'application/vnd.openxmlformats-officedocument.wordprocessingml.document']
const MAX_SIZE = 50 * 1024 * 1024

function validate(file: File): string {
  const ext = '.' + file.name.split('.').pop()?.toLowerCase()
  if (!ACCEPTED.includes(ext)) return `仅支持 PDF 和 docx 格式`
  if (file.size > MAX_SIZE) return `文件大小不能超过 50 MB`
  return ''
}

function pick(file: File) {
  const msg = validate(file)
  if (msg) { error.value = msg; return }
  error.value = ''
  selectedFile.value = file
  emit('file', file)
}

function onDrop(e: DragEvent) {
  dragging.value = false
  const file = e.dataTransfer?.files[0]
  if (file) pick(file)
}

function onInput(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0]
  if (file) pick(file)
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / 1024 / 1024).toFixed(1) + ' MB'
}

function clear() {
  selectedFile.value = null
  error.value = ''
}
</script>

<template>
  <div class="file-upload">
    <label
      class="drop-zone"
      :class="{ dragging, 'has-file': selectedFile }"
      @dragover.prevent="dragging = true"
      @dragleave="dragging = false"
      @drop.prevent="onDrop"
    >
      <input
        type="file"
        :accept="ACCEPTED.join(',')"
        class="hidden-input"
        @change="onInput"
      />

      <template v-if="!selectedFile">
        <div class="upload-icon">
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="17 8 12 3 7 8"/>
            <line x1="12" y1="3" x2="12" y2="15"/>
          </svg>
        </div>
        <p class="upload-hint">拖拽文件到此处，或 <span class="link">点击选择</span></p>
        <p class="upload-types">支持 PDF、docx · 最大 50 MB</p>
      </template>

      <template v-else>
        <div class="file-info">
          <div class="file-icon">
            <span v-if="selectedFile.name.endsWith('.pdf')">📄</span>
            <span v-else>📝</span>
          </div>
          <div class="file-details">
            <span class="file-name">{{ selectedFile.name }}</span>
            <span class="file-size">{{ formatSize(selectedFile.size) }}</span>
          </div>
          <button class="clear-btn" type="button" @click.prevent="clear" title="移除">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </div>
      </template>
    </label>

    <p v-if="error" class="error-msg">{{ error }}</p>
  </div>
</template>

<style scoped>
.file-upload { width: 100%; }

.drop-zone {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 32px 24px;
  border: 2px dashed var(--border-strong);
  border-radius: var(--radius-lg);
  background: var(--overlay);
  cursor: pointer;
  transition: all var(--duration-base) var(--ease-out);
  min-height: 140px;
}

.drop-zone:hover,
.drop-zone.dragging {
  border-color: var(--accent);
  background: var(--accent-muted);
}

.drop-zone.has-file {
  border-style: solid;
  border-color: var(--success);
  background: var(--success-muted);
  padding: 20px 24px;
  min-height: auto;
}

.hidden-input {
  position: absolute;
  width: 1px;
  height: 1px;
  opacity: 0;
  pointer-events: none;
}

.upload-icon {
  color: var(--text-tertiary);
  transition: color var(--duration-fast);
}

.drop-zone:hover .upload-icon,
.drop-zone.dragging .upload-icon {
  color: var(--accent);
}

.upload-hint {
  font-size: 14px;
  color: var(--text-secondary);
}

.link { color: var(--accent); font-weight: 500; }

.upload-types {
  font-size: 12px;
  color: var(--text-tertiary);
}

.file-info {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
}

.file-icon { font-size: 28px; flex-shrink: 0; }

.file-details {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.file-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-size {
  font-size: 12px;
  color: var(--text-secondary);
}

.clear-btn {
  flex-shrink: 0;
  width: 28px;
  height: 28px;
  border-radius: var(--radius-pill);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-tertiary);
  background: var(--overlay);
  border: 1px solid var(--border);
  cursor: pointer;
  transition: all var(--duration-fast);
}

.clear-btn:hover {
  color: var(--danger);
  border-color: var(--danger);
  background: var(--danger-muted);
}

.error-msg {
  margin-top: 8px;
  font-size: 13px;
  color: var(--danger);
  padding: 8px 12px;
  background: var(--danger-muted);
  border-radius: var(--radius-sm);
  border: 1px solid var(--danger);
}

[data-theme="cyberpunk"] .drop-zone {
  border-color: var(--border);
}
[data-theme="cyberpunk"] .drop-zone:hover,
[data-theme="cyberpunk"] .drop-zone.dragging {
  box-shadow: var(--neon-glow);
}
</style>

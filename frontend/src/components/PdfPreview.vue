<script setup lang="ts">
import { t } from '@/i18n'
import { onBeforeUnmount, onMounted, ref } from 'vue'
import { getDocument, GlobalWorkerOptions, type PDFDocumentLoadingTask } from 'pdfjs-dist'
import { EventBus, PDFLinkService, PDFViewer } from 'pdfjs-dist/web/pdf_viewer.mjs'
import workerUrl from 'pdfjs-dist/build/pdf.worker.min.mjs?url'
import 'pdfjs-dist/web/pdf_viewer.css'
import { getMyWriteupFile, getUserWriteupFile } from '@/api/writeup'

const props = withDefaults(defineProps<{
  gameId: number
  writeupId: number
  fileName: string
  owner: string
  source?: 'admin' | 'mine'
}>(), { source: 'admin' })
defineEmits<{ retry: [] }>()
const container = ref<HTMLDivElement>()
const viewerElement = ref<HTMLDivElement>()
const loading = ref(true)
const error = ref('')
const pageNumber = ref(1)
const pageCount = ref(0)
const scale = ref(1)
const controller = new AbortController()
let viewer: PDFViewer | undefined
let loadingTask: PDFDocumentLoadingTask | undefined
let resizeObserver: ResizeObserver | undefined

GlobalWorkerOptions.workerSrc = workerUrl

function goToPage(value: number) {
  if (viewer && Number.isInteger(value) && value >= 1 && value <= pageCount.value) viewer.currentPageNumber = value
}

function changePage(event: Event) {
  const input = event.target as HTMLInputElement
  goToPage(Number(input.value))
  input.value = String(pageNumber.value)
}

function zoom(factor: number) {
  if (viewer) viewer.currentScale = Math.max(0.25, Math.min(4, viewer.currentScale * factor))
}

function fitWidth() {
  if (viewer && pageCount.value) viewer.currentScaleValue = 'page-width'
}

onMounted(async () => {
  const eventBus = new EventBus()
  const linkService = new PDFLinkService({ eventBus, externalLinkTarget: 2, externalLinkRel: 'noopener noreferrer nofollow' })
  viewer = new PDFViewer({
    container: container.value!,
    viewer: viewerElement.value!,
    eventBus,
    linkService,
    imageResourcesPath: `${import.meta.env.BASE_URL}pdfjs/images/`,
  })
  linkService.setViewer(viewer)
  eventBus.on('pagesinit', () => {
    fitWidth()
    loading.value = false
  })
  eventBus.on('pagechanging', ({ pageNumber: page }: { pageNumber: number }) => { pageNumber.value = page })
  eventBus.on('scalechanging', ({ scale: value }: { scale: number }) => { scale.value = value })
  eventBus.on('pagerendered', ({ error: renderError }: { error?: Error }) => {
    if (renderError) error.value = t('pdf.renderFailed')
  })
  resizeObserver = new ResizeObserver(() => {
    if (viewer?.currentScaleValue === 'page-width') fitWidth()
  })
  resizeObserver.observe(container.value!)

  try {
    const data = await (props.source === 'mine'
      ? getMyWriteupFile(props.gameId, controller.signal)
      : getUserWriteupFile(props.gameId, props.writeupId, controller.signal))
    if (controller.signal.aborted) return
    const assets = `${import.meta.env.BASE_URL}pdfjs/`
    loadingTask = getDocument({
      data,
      cMapUrl: `${assets}cmaps/`,
      cMapPacked: true,
      standardFontDataUrl: `${assets}standard_fonts/`,
      wasmUrl: `${assets}wasm/`,
    })
    loadingTask.onPassword = () => {
      error.value = t('pdf.encrypted')
      loading.value = false
      void loadingTask?.destroy()
    }
    const pdf = await loadingTask.promise
    if (controller.signal.aborted) return
    pageCount.value = pdf.numPages
    linkService.setDocument(pdf)
    viewer.setDocument(pdf)
  } catch (e: any) {
    if (controller.signal.aborted) return
    if (!error.value) {
      error.value = e.response?.status === 403 ? t('pdf.forbidden')
        : e.response?.status === 401 ? t('pdf.expired')
        : e.response?.status === 404 ? t('pdf.notFound')
        : t('pdf.loadFailed')
    }
    loading.value = false
  }
})

onBeforeUnmount(() => {
  controller.abort()
  resizeObserver?.disconnect()
  viewer?.setDocument(null)
  void loadingTask?.destroy()
})
</script>

<template>
  <div class="pdf-preview">
    <div class="pdf-toolbar" :aria-label="t('pdf.toolbar')">
      <span class="document-name" :title="fileName">{{ fileName }}</span>
      <div class="toolbar-controls">
        <div class="toolbar-group">
          <button :title="t('common.previous')" :aria-label="t('common.previous')" :disabled="loading || !!error || pageNumber <= 1" @click="goToPage(pageNumber - 1)">‹</button>
          <input :value="pageNumber" type="number" min="1" :max="pageCount || 1" :disabled="loading || !!error" :aria-label="t('pdf.page')" @change="changePage" />
          <span>/ {{ pageCount }}</span>
          <button :title="t('common.next')" :aria-label="t('common.next')" :disabled="loading || !!error || pageNumber >= pageCount" @click="goToPage(pageNumber + 1)">›</button>
        </div>
        <div class="toolbar-group">
          <button :title="t('pdf.zoomOut')" :aria-label="t('pdf.zoomOut')" :disabled="loading || !!error || scale <= 0.25" @click="zoom(1 / 1.2)">−</button>
          <span class="zoom-value">{{ Math.round(scale * 100) }}%</span>
          <button :title="t('pdf.zoomIn')" :aria-label="t('pdf.zoomIn')" :disabled="loading || !!error || scale >= 4" @click="zoom(1.2)">+</button>
          <button class="fit-button" :disabled="loading || !!error" @click="fitWidth">{{ t('pdf.fitWidth') }}</button>
        </div>
      </div>
      <span class="document-owner" :title="owner">{{ owner }}</span>
    </div>
    <div class="pdf-body">
      <div ref="container" class="pdf-container" :aria-busy="loading">
        <div ref="viewerElement" class="pdfViewer"></div>
      </div>
      <div v-if="loading || error" class="pdf-state" :role="error ? 'alert' : 'status'">
        <span v-if="loading && !error" class="spinner" aria-hidden="true"></span>
        <p>{{ error || t('pdf.loading') }}</p>
        <button v-if="error" class="retry-button" @click="$emit('retry')">{{ t('common.retry') }}</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.pdf-preview { display: flex; flex: 1; flex-direction: column; min-height: 0; container-type: inline-size; }
.pdf-toolbar { display: grid; grid-template-columns: minmax(0, 1fr) auto minmax(0, 1fr); align-items: center; gap: 8px 16px; padding: 8px 20px; background: var(--surface); border-bottom: 1px solid var(--border); font-size: 12px; }
.document-name, .document-owner { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 13px; }
.document-name { font-weight: 500; }
.document-owner { text-align: right; color: var(--text-secondary); }
.toolbar-controls { display: flex; align-items: center; justify-content: center; gap: 24px; }
.toolbar-group { display: flex; align-items: center; gap: 6px; }
.pdf-toolbar button { height: 30px; min-width: 30px; padding: 0 8px; border-radius: var(--radius-sm); color: var(--text-primary); font-size: 20px; }
.pdf-toolbar button:hover:not(:disabled) { background: var(--overlay); }
.pdf-toolbar button:disabled { opacity: 0.35; cursor: not-allowed; }
.pdf-toolbar .fit-button { font-size: 12px; }
.pdf-toolbar input { width: 52px; height: 28px; border: 1px solid transparent; border-radius: var(--radius-sm); background: var(--overlay); color: var(--text-primary); text-align: center; font-size: 12px; }
.pdf-toolbar input::-webkit-inner-spin-button { appearance: none; }
.zoom-value { min-width: 40px; text-align: center; }
.pdf-body { position: relative; flex: 1; min-height: 0; }
.pdf-container { position: absolute; inset: 0; overflow: auto; overscroll-behavior: contain; }
.pdfViewer { padding: 16px 0; }
.pdf-state { position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 14px; padding: 24px; background: var(--bg-sunken); color: var(--text-secondary); text-align: center; font-size: 14px; }
.retry-button { padding: 8px 16px; color: var(--text-primary); background: var(--overlay); border-radius: var(--radius-sm); }
.spinner { width: 24px; height: 24px; border: 2px solid var(--border); border-top-color: var(--accent); border-radius: 50%; animation: spin 0.7s linear infinite; }
@container (max-width: 600px) {
  .pdf-toolbar { grid-template-columns: minmax(0, 1fr) auto; padding: 8px 12px; }
  .toolbar-controls { grid-column: 1 / -1; grid-row: 2; gap: 12px; }
  .document-owner { grid-column: 2; grid-row: 1; }
}
@keyframes spin { to { transform: rotate(360deg); } }
</style>

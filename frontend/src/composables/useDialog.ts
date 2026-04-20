import { ref, readonly } from 'vue'

interface AlertState {
  show: boolean
  title: string
  message: string
}

interface InputState {
  show: boolean
  title: string
  message: string
  defaultValue: string
  placeholder: string
}

const alertState = ref<AlertState>({
  show: false,
  title: '',
  message: '',
})

const inputState = ref<InputState>({
  show: false,
  title: '',
  message: '',
  defaultValue: '',
  placeholder: '',
})

let alertResolve: (() => void) | null = null
let inputResolve: ((value: string | null) => void) | null = null

function showAlert(message: string, title?: string): Promise<void> {
  alertState.value = { show: true, title: title ?? '', message }
  return new Promise<void>((resolve) => {
    alertResolve = resolve
  })
}

function showInput(
  message: string,
  defaultValue?: string,
  title?: string,
  placeholder?: string
): Promise<string | null> {
  inputState.value = {
    show: true,
    title: title ?? '',
    message,
    defaultValue: defaultValue ?? '',
    placeholder: placeholder ?? '',
  }
  return new Promise<string | null>((resolve) => {
    inputResolve = resolve
  })
}

function handleAlertOk() {
  alertState.value.show = false
  alertResolve?.()
  alertResolve = null
}

function handleInputConfirm(value: string) {
  inputState.value.show = false
  inputResolve?.(value)
  inputResolve = null
}

function handleInputCancel() {
  inputState.value.show = false
  inputResolve?.(null)
  inputResolve = null
}

export function useDialog() {
  return {
    alert: showAlert,
    prompt: showInput,
    alertState: readonly(alertState),
    inputState: readonly(inputState),
    handleAlertOk,
    handleInputConfirm,
    handleInputCancel,
  }
}

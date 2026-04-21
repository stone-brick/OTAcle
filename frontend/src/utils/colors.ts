export const containerMaxW = 'xl:max-w-6xl xl:mx-auto'

export const colorsTransparent = {
  bg: 'bg-transparent',
  bgHover: 'hover:bg-gray-100 dark:hover:bg-slate-700',
  border: 'border-transparent',
  ring: 'ring-transparent',
}

export const gradientBgBase = 'bg-linear-to-tr'
export const gradientBgPurplePink = `${gradientBgBase} from-purple-400 via-pink-500 to-red-500`
export const gradientBgDark = `${gradientBgBase} from-slate-700 via-slate-900 to-slate-800`
export const gradientBgPinkRed = `${gradientBgBase} from-pink-400 via-red-500 to-yellow-500`

export const colorsBgLight = {
  white: 'bg-white text-black',
  light: 'bg-white text-black dark:bg-slate-900/70 dark:text-white',
  contrast: 'bg-gray-800 text-white dark:bg-white dark:text-black',
  success: 'bg-emerald-300 border-emerald-300 text-white',
  danger: 'bg-red-400 border-red-400 text-white',
  warning: 'bg-yellow-400 border-yellow-400 text-white',
  info: 'bg-blue-400 border-blue-400 text-white',
}

export const colorsText = {
  white: 'text-black dark:text-slate-100',
  light: 'text-gray-700 dark:text-slate-400',
  contrast: 'dark:text-white',
  success: 'text-emerald-400',
  danger: 'text-red-500',
  warning: 'text-yellow-600',
  info: 'text-blue-500',
}

export const colorsOutline = {
  white: [colorsText.white, 'border-gray-100'],
  light: [colorsText.light, 'border-gray-100'],
  contrast: [colorsText.contrast, 'border-gray-900 dark:border-slate-100'],
  success: [colorsText.success, 'border-emerald-400'],
  danger: [colorsText.danger, 'border-red-500'],
  warning: [colorsText.warning, 'border-yellow-600'],
  info: [colorsText.info, 'border-blue-500'],
}

type ButtonColor = 'white' | 'whiteDark' | 'lightDark' | 'contrast' | 'success' | 'danger' | 'warning' | 'info'

export const getButtonColor = (color: ButtonColor, isOutlined: boolean, hasHover: boolean, isActive = false): string[] => {
  const colors = {
    ring: {
      white: 'ring-gray-200 dark:ring-gray-500',
      whiteDark: 'ring-gray-200 dark:ring-gray-500',
      lightDark: 'ring-gray-200 dark:ring-gray-500',
      contrast: 'ring-gray-300 dark:ring-gray-400',
      success: 'ring-emerald-200 dark:ring-emerald-700',
      danger: 'ring-red-200 dark:ring-red-700',
      warning: 'ring-yellow-200 dark:ring-yellow-700',
      info: 'ring-blue-200 dark:ring-blue-700',
    },
    active: {
      white: 'bg-gray-100',
      whiteDark: 'bg-gray-100 dark:bg-slate-800',
      lightDark: 'bg-gray-200 dark:bg-slate-700',
      contrast: 'bg-gray-700 dark:bg-slate-100',
      success: 'bg-emerald-400 dark:bg-emerald-500',
      danger: 'bg-red-500 dark:bg-red-600',
      warning: 'bg-yellow-500 dark:bg-yellow-600',
      info: 'bg-blue-500 dark:bg-blue-600',
    },
    bg: {
      white: 'bg-white text-black',
      whiteDark: 'bg-white text-black dark:bg-slate-900 dark:text-white',
      lightDark: 'bg-gray-100 text-black dark:bg-slate-800 dark:text-white',
      contrast: 'bg-gray-800 text-white dark:bg-white dark:text-black',
      success: 'bg-emerald-400 dark:bg-emerald-500 text-white',
      danger: 'bg-red-500 dark:bg-red-600 text-white',
      warning: 'bg-yellow-500 dark:bg-yellow-600 text-white',
      info: 'bg-blue-500 dark:bg-blue-600 text-white',
    },
    bgHover: {
      white: 'hover:bg-gray-100',
      whiteDark: 'hover:bg-gray-100 dark:hover:bg-slate-800',
      lightDark: 'hover:bg-gray-200 dark:hover:bg-slate-700',
      contrast: 'hover:bg-gray-700 dark:hover:bg-slate-100',
      success:
        'hover:bg-emerald-500 hover:border-emerald-500 dark:hover:bg-emerald-600 dark:hover:border-emerald-600',
      danger:
        'hover:bg-red-600 hover:border-red-600 dark:hover:bg-red-700 dark:hover:border-red-700',
      warning:
        'hover:bg-yellow-600 hover:border-yellow-600 dark:hover:bg-yellow-700 dark:hover:border-yellow-700',
      info: 'hover:bg-blue-600 hover:border-blue-600 dark:hover:bg-blue-700 dark:hover:border-blue-700',
    },
    borders: {
      white: 'border-white',
      whiteDark: 'border-white dark:border-slate-900',
      lightDark: 'border-gray-100 dark:border-slate-800',
      contrast: 'border-gray-800 dark:border-white',
      success: 'border-emerald-400 dark:border-emerald-500',
      danger: 'border-red-500 dark:border-red-600',
      warning: 'border-yellow-500 dark:border-yellow-600',
      info: 'border-blue-500 dark:border-blue-600',
    },
    text: {
      white: 'text-black dark:text-white',
      whiteDark: 'text-black dark:text-white',
      lightDark: 'text-gray-700 dark:text-slate-300',
      contrast: 'dark:text-slate-100',
      success: 'text-emerald-500 dark:text-emerald-400',
      danger: 'text-red-600 dark:text-red-500',
      warning: 'text-yellow-600 dark:text-yellow-500',
      info: 'text-blue-600 dark:text-blue-500',
    },
    outlineHover: {
      white: 'hover:bg-gray-100 dark:hover:bg-slate-700',
      whiteDark: 'hover:bg-gray-100 dark:hover:bg-slate-800',
      lightDark: 'hover:bg-gray-200 dark:hover:bg-slate-700',
      contrast:
        'hover:bg-gray-800 hover:text-gray-100 dark:hover:bg-slate-100 dark:hover:text-black',
      success:
        'hover:bg-emerald-500 hover:text-white dark:hover:text-white dark:hover:border-emerald-500',
      danger:
        'hover:bg-red-600 hover:text-white dark:hover:text-white dark:hover:border-red-600',
      warning:
        'hover:bg-yellow-600 hover:text-white dark:hover:text-white dark:hover:border-yellow-600',
      info: 'hover:bg-blue-600 hover:border-blue-600 dark:hover:bg-blue-700 dark:hover:border-blue-700',
    },
  }

  if (!colors.bg[color]) {
    return [color]
  }

  const isOutlinedProcessed = isOutlined && ['white', 'whiteDark', 'lightDark'].indexOf(color) < 0

  const base: string[] = [colors.borders[color], colors.ring[color]]

  if (isActive) {
    base.push(colors.active[color])
  } else {
    base.push(isOutlinedProcessed ? colors.text[color] : colors.bg[color])
  }

  if (hasHover) {
    base.push(isOutlinedProcessed ? colors.outlineHover[color] : colors.bgHover[color])
  }

  return base
}

export type LogStatusType = 'info' | 'warn' | 'success' | 'error'

const statusTextColors: Record<LogStatusType, string> = {
  success: 'text-emerald-500 dark:text-emerald-400',
  error: 'text-red-500 dark:text-red-400',
  warn: 'text-yellow-500 dark:text-yellow-400',
  info: 'text-gray-500 dark:text-slate-400',
}

export function getStatusTextColor(type: LogStatusType): string {
  return statusTextColors[type] || statusTextColors.info
}

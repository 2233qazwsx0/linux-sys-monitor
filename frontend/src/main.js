import { createApp } from 'vue'
import App from './App.vue'
import './styles/main.css'

const app = createApp(App)

const browserLang = navigator.language.toLowerCase()
const isZh = browserLang.includes('zh')

window.i18n = {
  t: (key) => {
    const translations = {
      'en': {
        'title': 'System Monitor',
        'live': 'Live',
        'disconnected': 'Disconnected',
        'uptime': 'Uptime',
        'cpu': 'CPU',
        'memory': 'Memory',
        'disk': 'Disk I/O',
        'network': 'Network',
        'processes': 'Processes',
        'cores': 'Cores',
        'read': 'Read',
        'write': 'Write',
        'download': 'Download',
        'upload': 'Upload',
        'pid': 'PID',
        'name': 'Name',
        'cpu_percent': 'CPU %',
        'mem_percent': 'Memory %',
        'top_processes': 'Top Processes',
        'avg': 'avg'
      },
      'zh': {
        'title': '系统监控',
        'live': '实时',
        'disconnected': '已断开',
        'uptime': '运行时间',
        'cpu': 'CPU',
        'memory': '内存',
        'disk': '磁盘 I/O',
        'network': '网络',
        'processes': '进程',
        'cores': '核心',
        'read': '读取',
        'write': '写入',
        'download': '下载',
        'upload': '上传',
        'pid': 'PID',
        'name': '名称',
        'cpu_percent': 'CPU %',
        'mem_percent': '内存 %',
        'top_processes': '热门进程',
        'avg': '平均'
      }
    }
    const lang = isZh ? 'zh' : 'en'
    return translations[lang][key] || key
  },
  isZh
}

app.mount('#app')

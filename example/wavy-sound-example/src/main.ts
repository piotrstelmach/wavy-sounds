import init, { parse_audio } from 'wavy-sounds'
import './style.css'

await init();

function drawAudioWave(data: Float32Array | number[], canvas: HTMLCanvasElement) {
    const ctx = canvas.getContext('2d')
    if (!ctx) return;

    const { width, height } = canvas;
    ctx.clearRect(0, 0, width, height)

    const config = {
        barWidth: 20,
        barGap: 5,
        minBarHeight: 2,
        barColor: '#2196F3'
    }

    const dataPoints = Array.from(data)
    const totalBarsWidth = config.barWidth + config.barGap
    const barsCount = Math.min(dataPoints.length, Math.floor(width / totalBarsWidth))

    // Możemy użyć bezpośrednio znormalizowanych wartości
    const heightScale = (height / 2) * 0.9

    // Gradient dla lepszego efektu wizualnego
    const gradient = ctx.createLinearGradient(0, 0, 0, height)
    gradient.addColorStop(0, '#1976D2')
    gradient.addColorStop(0.5, '#2196F3')
    gradient.addColorStop(1, '#64B5F6')
    ctx.fillStyle = gradient

    for (let i = 0; i < barsCount; i++) {
        const value = dataPoints[i]
        if (Number.isFinite(value)) {
            // Używamy znormalizowanej wartości (0-100) do określenia wysokości
            const normalizedHeight = value / 100
            const barHeight = Math.max(normalizedHeight * heightScale, config.minBarHeight)
            
            const x = i * totalBarsWidth + (width - barsCount * totalBarsWidth) / 2
            const y = height / 2

            ctx.fillRect(x, y - barHeight, config.barWidth, barHeight)
            ctx.fillRect(x, y, config.barWidth, barHeight * 0.8)
        }
    }
}

async function handleFile(file: File, canvas: HTMLCanvasElement, groupSize: number, debugInfo: HTMLPreElement) {
    try {
        const buffer = await file.arrayBuffer()
        const audioData = new Uint8Array(buffer)
        
        const waveform = parse_audio(audioData, groupSize)

      console.log('Raw waveform data:', waveform);


      const stats = {
            min: Math.min(...waveform),
            max: Math.max(...waveform),
            avg: waveform.reduce((a, b) => a + b, 0) / waveform.length
        }
        
        const debugText = [
            `File size: ${file.size} bytes`,
            `Waveform samples: ${waveform.length}`,
            `Value range: ${stats.min.toFixed(4)} to ${stats.max.toFixed(4)}`,
            `Average value: ${stats.avg.toFixed(4)}`
        ]
        
        debugInfo.textContent = debugText.join('\n')
        
        if (waveform && waveform.length > 0) {
            drawAudioWave(Array.from(waveform), canvas)
        } else {
            throw new Error('Waveform is empty or invalid')
        }
    } catch (error) {
        console.error('Error processing audio:', error)
        debugInfo.textContent = `Error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
}

function main() {
    const app = document.querySelector<HTMLDivElement>('#app')
    if (!app) return

    const container = document.createElement('div')
    container.className = 'container'

    const controls = document.createElement('div')
    controls.className = 'controls'

    const canvas = document.createElement('canvas')
    canvas.width = 800
    canvas.height = 200
    
    const fileInput = document.createElement('input')
    fileInput.type = 'file'
    fileInput.accept = 'audio/*'

    const groupSizeInput = document.createElement('input')
    groupSizeInput.type = 'number'
    groupSizeInput.value = '8'
    groupSizeInput.min = '1'

    const debugInfo = document.createElement('pre')
    debugInfo.style.textAlign = 'left'
    
    controls.appendChild(fileInput)
    controls.appendChild(groupSizeInput)
    
    container.appendChild(controls)
    container.appendChild(canvas)
    container.appendChild(debugInfo)
    
    app.appendChild(container)

    fileInput.addEventListener('change', async (event) => {
        const file = (event.target as HTMLInputElement).files?.[0]
        if (!file) return
        
        const groupSize = parseInt(groupSizeInput.value) || 8
        await handleFile(file, canvas, groupSize, debugInfo)
    })
}

main()
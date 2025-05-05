import init, {parse_audio} from 'wavy-sounds'
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
    const totalBarsWidth = config.barWidth + config.barGap;
    const barsCount = dataPoints.length;
    canvas.width = barsCount * totalBarsWidth;

    const heightScale = (height / 2) * 0.9

    const gradient = ctx.createLinearGradient(0, 0, 0, height)
    gradient.addColorStop(0, '#0D47A1')  // głęboki niebieski
    gradient.addColorStop(0.5, '#512DA8') // fiolet
    gradient.addColorStop(1, '#00ACC1')  // błękit morski
    ctx.fillStyle = gradient

    for (let i = 0; i < barsCount; i++) {
        const value = dataPoints[i] 
        if (Number.isFinite(value)) {
            // Logarytmiczna transformacja wartości amplitudy dla lepszej dynamiki wizualnej
            const logValue = Math.log10(1 + value * 9);
            const barHeight = Math.max(logValue * heightScale, config.minBarHeight);
            
            const x = i * totalBarsWidth;
            const y = height / 2

            const radius = config.barWidth / 2;

            ctx.beginPath();
            ctx.roundRect?.(
                x,
                y - barHeight,                  // start od góry
                config.barWidth,
                barHeight * 2,                 // pełna wysokość (góra + dół)
                radius
            )
            ctx.fill();

        }
    }
}

async function handleFile(file: File, canvas: HTMLCanvasElement, debugInfo: HTMLPreElement) {
    try {
        const buffer = await file.arrayBuffer()
        const audioData = new Uint8Array(buffer)
        
        const waveform = parse_audio(audioData, 8)

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

    const debugInfo = document.createElement('pre')
    debugInfo.style.textAlign = 'left'
    
    controls.appendChild(fileInput)

    container.appendChild(controls)
    container.appendChild(canvas)
    container.appendChild(debugInfo)
    
    app.appendChild(container)

    fileInput.addEventListener('change', async (event) => {
        const file = (event.target as HTMLInputElement).files?.[0]
        if (!file) return
        
        // const groupSize = parseInt(groupSizeInput.value) || 8
        await handleFile(file, canvas, debugInfo)
    })
}

main()
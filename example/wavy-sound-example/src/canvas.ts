import './style.css';
import { parse_audio } from 'wavy-sounds';
import init from 'wavy-sounds';

function createCanvas(width: number, height: number): [HTMLCanvasElement, CanvasRenderingContext2D] {
  const canvas = document.createElement('canvas');
  canvas.width = width;
  canvas.height = height;
  
  const ctx = canvas.getContext('2d');
  if (!ctx) throw new Error('Cannot get 2d context');
  
  return [canvas, ctx];
}

function drawWaveform(ctx: CanvasRenderingContext2D, data: number[]) {
  const { width, height } = ctx.canvas;
  
  ctx.clearRect(0, 0, width, height);
  ctx.beginPath();
  ctx.strokeStyle = '#2196F3';
  ctx.lineWidth = 2;

  const step = width / data.length;
  const middle = height / 2;

  data.forEach((value, index) => {
    const x = index * step;
    const y = middle + (value * middle);
    
    if (index === 0) {
      ctx.moveTo(x, y);
    } else {
      ctx.lineTo(x, y);
    }
  });

  ctx.stroke();
}

function createControls(): [HTMLInputElement, HTMLInputElement] {
  const fileInput = document.createElement('input');
  fileInput.type = 'file';
  fileInput.accept = 'audio/*';
  
  const groupSizeInput = document.createElement('input');
  groupSizeInput.type = 'number';
  groupSizeInput.value = '1024';
  groupSizeInput.min = '1';
  groupSizeInput.placeholder = 'Group size';

  return [fileInput, groupSizeInput];
}

async function handleFileSelect(
  file: File, 
  groupSize: number, 
  ctx: CanvasRenderingContext2D
): Promise<void> {
  try {
    const arrayBuffer = await file.arrayBuffer();
    const audioData = new Uint8Array(arrayBuffer);
    const waveform = parse_audio(audioData, groupSize);
    drawWaveform(ctx, Array.from(waveform));
  } catch (error) {
    console.error('Error processing audio:', error);
    alert('Error processing audio file');
  }
}

function createContainer(): HTMLDivElement {
  const container = document.createElement('div');
  container.className = 'container';
  return container;
}

function createControlsContainer(): HTMLDivElement {
  const controls = document.createElement('div');
  controls.className = 'controls';
  return controls;
}

async function setupApp() {
  // Initialize WASM module
  await init();

  const app = document.getElementById('app');
  if (!app) throw new Error('No app container found');

  const container = createContainer();
  const [canvas, ctx] = createCanvas(800, 200);
  const [fileInput, groupSizeInput] = createControls();
  const controlsContainer = createControlsContainer();

  fileInput.addEventListener('change', async (event) => {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0];
    
    if (!file) return;

    const groupSize = parseInt(groupSizeInput.value) || 1024;
    await handleFileSelect(file, groupSize, ctx);
  });

  controlsContainer.appendChild(fileInput);
  controlsContainer.appendChild(groupSizeInput);
  container.appendChild(controlsContainer);
  container.appendChild(canvas);
  app.appendChild(container);
}

setupApp().catch(console.error);
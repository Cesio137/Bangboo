import { loadImage, createCanvas } from "@napi-rs/canvas";

function dct2(matrix: number[][]): number[][] {
  const N = matrix.length;
  const result: number[][] = Array.from({ length: N }, () => Array(N).fill(0));

  for (let u = 0; u < N; u++) {
    for (let v = 0; v < N; v++) {
      let sum = 0;
      for (let x = 0; x < N; x++) {
        for (let y = 0; y < N; y++) {
          sum +=
            matrix[x][y] *
            Math.cos(((2 * x + 1) * u * Math.PI) / (2 * N)) *
            Math.cos(((2 * y + 1) * v * Math.PI) / (2 * N));
        }
      }
      const cu = u === 0 ? 1 / Math.sqrt(2) : 1;
      const cv = v === 0 ? 1 / Math.sqrt(2) : 1;
      result[u][v] = (2 / N) * cu * cv * sum;
    }
  }

  return result;
}

export async function getPhashFromImageBuffer(
  buffer: ArrayBuffer,
  hashSize = 8,
  highfreqFactor = 4
): Promise<string> {
  const size = hashSize * highfreqFactor; // 8 × 4 = 32
  const image = await loadImage(buffer);
  const canvas = createCanvas(size, size);
  const ctx = canvas.getContext('2d');

  ctx.drawImage(image, 0, 0, size, size);
  const imageData = ctx.getImageData(0, 0, size, size);
  const pixels = imageData.data;

  const grayMatrix: number[][] = [];
  for (let y = 0; y < size; y++) {
    const row: number[] = [];
    for (let x = 0; x < size; x++) {
      const i = (y * size + x) * 4;
      const r = pixels[i];
      const g = pixels[i + 1];
      const b = pixels[i + 2];
      const gray = 0.299 * r + 0.587 * g + 0.114 * b;
      row.push(gray);
    }
    grayMatrix.push(row);
  }

  const dct = dct2(grayMatrix);
  const dctLowFreq = dct.slice(0, hashSize).map(row => row.slice(0, hashSize));
  const flat = dctLowFreq.flat();
  const avg = flat.reduce((a, b) => a + b, 0) / flat.length;

  const bits = flat.map(val => (val > avg ? '1' : '0')).join('');
  const hex = parseInt(bits, 2).toString(16).padStart(hashSize * hashSize / 4, '0');

  return hex;
}
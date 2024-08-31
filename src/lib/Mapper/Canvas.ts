import type { Point } from './Geometry';

//todo dashboard design!!
export class Canvas {
	public static GetMousePos(e: MouseEvent, canvas: HTMLCanvasElement): Point {
		const rect = canvas.getBoundingClientRect();
		return {
			x: (e.clientX - rect.left) * (canvas.width / rect.width),
			y: (e.clientY - rect.top) * (canvas.height / rect.height)
		};
	}

	public static Clear(ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement) {
		ctx.clearRect(0, 0, canvas.width, canvas.height);
	}

	public static Resize(canvas: HTMLCanvasElement) {
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;
	}

	public static DrawPaths(ctx: CanvasRenderingContext2D, paths: Point[][]) {
		ctx.lineWidth = 3;
		for (const path of paths) {
			ctx.beginPath();
			for (let i = 1; i < path.length; i++) {
				const lastPoint = path[i - 1];
				const point = path[i];
				ctx.moveTo(lastPoint.x, lastPoint.y);
				ctx.lineTo(point.x, point.y);
			}
			ctx.stroke();
		}
	}

	public static DrawCircle(
		ctx: CanvasRenderingContext2D,
		origin: Point,
		radius: number,
		color?: string
	) {
		const prevStyle = ctx.strokeStyle;
		if (color) ctx.strokeStyle = color;
		ctx.lineWidth = 2;

		ctx.beginPath();
		ctx.arc(origin.x, origin.y, radius, 0, 2 * Math.PI);
		ctx.stroke();

		ctx.strokeStyle = prevStyle;
	}

	public static FitBoundingBoxToCanvas(
		ctx: CanvasRenderingContext2D,
		canvas: HTMLCanvasElement,
		w: number,
		h: number
	): void {
		// Calculate the scale factor needed to fit the bounding box within the canvas
		const canvasWidth = canvas.clientWidth;
		const canvasHeight = canvas.clientHeight;
		const scaleX = canvasWidth / w;
		const scaleY = canvasHeight / h;

		// Choose the smaller scale to ensure the entire bounding box fits
		const scale = Math.min(scaleX, scaleY);

		// Calculate the offset to center the bounding box
		const offsetX = (canvasWidth - w * scale) / 2;
		const offsetY = (canvasHeight - h * scale) / 2;

		// Increase the resolution of the canvas to reduce blurriness
		const scaleFactor = window.devicePixelRatio || 1; // Adjust based on device pixel ratio
		canvas.width = canvasWidth * scaleFactor;
		canvas.height = canvasHeight * scaleFactor;
		ctx.scale(scaleFactor, scaleFactor);

		// Disable image smoothing to prevent blurriness
		ctx.imageSmoothingEnabled = false;

		// Clear the canvas
		ctx.clearRect(0, 0, canvasWidth, canvasHeight);

		// Apply the scale and translation to center the bounding box
		ctx.setTransform(scale, 0, 0, scale, offsetX, offsetY);

		// Reset transform after drawing
		ctx.setTransform(1, 0, 0, 1, 0, 0);
	}

	public static TrueToScreen(p: Point, scale: number, offsetX: number, offsetY: number): Point {
		return { x: (p.x + offsetX) * scale, y: (p.y + offsetY) * scale };
	}

	public static ScreenToTrue(
		evt: MouseEvent,
		scale: number,
		offsetX: number,
		offsetY: number
	): Point {
		return { x: evt.pageX / scale - offsetX, y: evt.pageY / scale - offsetY };
	}

	public static toScreenX(xTrue: number, scale: number, offsetX: number): number {
		return (xTrue + offsetX) * scale;
	}

	public static toScreenY(yTrue: number, scale: number, offsetY: number): number {
		return (yTrue + offsetY) * scale;
	}

	public static toTrueX(xScreen: number, scale: number, offsetX: number): number {
		return xScreen / scale - offsetX;
	}

	public static toTrueY(yScreen: number, scale: number, offsetY: number): number {
		return yScreen / scale - offsetY;
	}

	// public static trueHeight(): number {
	// 	return canvas.clientHeight / scale;
	// }

	// function trueWidth(): number {
	// 	return canvas.clientWidth / scale;
	// }
}

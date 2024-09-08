import type { Point } from './Geometry';

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
}

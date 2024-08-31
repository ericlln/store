import type { Point } from '$lib/Mapper/Geometry';
import type { Item, Store } from './Models';
import { invoke } from '@tauri-apps/api';

export class Backend {
	public static async GetDrawing(): Promise<Point[][]> {
		try {
			const coordArray: number[][][] = await invoke('send_drawing');

			const paths: Point[][] = coordArray.map((path) =>
				path.map((point) => ({ x: point[0], y: point[1] }))
			);

			return paths;
		} catch (error) {
			console.error('Failed to fetch shapes: ', error);
			return [];
		}
	}

	public static async SendDrawing(paths: Point[][]): Promise<void> {
		try {
			const shapes = paths.map((path) => path.map((point) => [point.x, point.y]));
			await invoke('receive_drawing', { shapes });
		} catch (error) {
			console.error('Failed to send shapes: ', error);
		}
	}

	public static async CreateStore(name: string, location: Point): Promise<boolean> {
		try {
			return await invoke('create_store', {
				name: name,
				spaceId: 1,
				x: location.x,
				y: location.y
			});
		} catch (err: unknown) {
			console.error('Failed to create store: ', err);
			return false;
		}
	}

	public static async FetchStore(id: number): Promise<Store | null> {
		try {
			return await invoke<Store>('fetch_store', { id });
		} catch (err: unknown) {
			console.error('Failed to fetch store: ', err);
			return null;
		}
	}

	public static async FetchAllStores(spaceId: number): Promise<Store[]> {
		try {
			const resp = await invoke<Store[]>('fetch_all_stores', { spaceId });
			return resp;
		} catch (err: unknown) {
			console.error(`Failed to fetch stores on space ${spaceId}: ${err}`);
			return [];
		}
	}

	public static async AddItem(
		storeId: number,
		name: string,
		quantity: number,
		notes: string
	): Promise<boolean> {
		try {
			return await invoke<boolean>('add_item', { storeId, name, quantity, notes });
		} catch (err: unknown) {
			console.error('Failed to add item: ', err);
			return false;
		}
	}

	public static async FetchItem(id: number): Promise<Item | null> {
		try {
			return await invoke<Item>('fetch_item', { id });
		} catch (err: unknown) {
			console.error('Failed to fetch item: ', err);
			return null;
		}
	}
}

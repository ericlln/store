import type { Point } from '$lib/Mapper/Geometry';
import type { Item, Space, Store } from './Models';
import { invoke } from '@tauri-apps/api';

export class Backend {
	public static async CreateStore(name: string, path: string): Promise<boolean> {
		try {
			await invoke('create_store', { name, path });
			return true;
		} catch (err: unknown) {
			console.error('Failed to create store: ', err);
			return false;
		}
	}

	public static async OpenStore(name: string): Promise<string | null> {
		try {
			return await invoke('open_store', { name });
		} catch (err: unknown) {
			console.error(`Failed to get store: ${err}`);
			return null;
		}
	}

	public static async GetStoreList(): Promise<string[] | null> {
		try {
			return await invoke<string[]>('get_store_list');
		} catch (err: unknown) {
			console.error('Failed to get store list: ', err);
			return null;
		}
	}

	public static async CreateSpace(
		storeName: string,
		name: string,
		drawing: Point[][]
	): Promise<boolean> {
		try {
			await invoke('create_space', { storeName, name, drawing });
			return true;
		} catch (err: unknown) {
			console.error(`Failed to create store list: ${err}`);
			return false;
		}
	}

	public static async GetSpace(storeName: string, spaceId: number): Promise<Space | null> {
		try {
			return await invoke('get_space', { storeName, spaceId });
		} catch (err: unknown) {
			console.error(`Failed to get space with id ${spaceId} in ${storeName}: ${err}`);
			return null;
		}
	}

	public static async GetSpaceList(storeName: string): Promise<Space[] | null> {
		try {
			return await invoke<Space[]>('get_space_list', { storeName });
		} catch (err: unknown) {
			console.error(`Failed to get spaces: ${err}`);
			return null;
		}
	}

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

	public static async SendDrawing(paths: Point[][]): Promise<boolean> {
		try {
			const shapes = paths.map((path) => path.map((point) => [point.x, point.y]));
			await invoke('receive_drawing', { shapes });
			return true;
		} catch (error) {
			console.error('Failed to send shapes: ', error);
			return false;
		}
	}

	// public static async CreateStore(name: string, location: Point): Promise<boolean> {
	// 	try {
	// 		return await invoke('create_store', {
	// 			name: name,
	// 			spaceId: 1,
	// 			x: location.x,
	// 			y: location.y
	// 		});
	// 	} catch (err: unknown) {
	// 		console.error('Failed to create store: ', err);
	// 		return false;
	// 	}
	// }

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

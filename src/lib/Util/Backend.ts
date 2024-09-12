import type { Point } from '$lib/Mapper/Geometry';
import type { Bin, Item, Space, Store } from './Models';
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

	public static async CreateBin(
		storeName: string,
		spaceId: number,
		name: string,
		x: number,
		y: number
	): Promise<boolean> {
		try {
			await invoke('create_bin', {
				storeName,
				spaceId,
				name,
				x,
				y
			});
			return true;
		} catch (err: unknown) {
			console.error('Failed to create bin: ', err);
			return false;
		}
	}

	public static async GetBinList(storeName: string, spaceId: number): Promise<Bin[] | null> {
		try {
			return await invoke<Bin[]>('get_bin_list', { storeName, spaceId });
		} catch (err: unknown) {
			console.error(`Failed to get spaces: ${err}`);
			return null;
		}
	}

	//
	//todo: unused code, here for reference for now
	//
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

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

	public static async GetStoreList(): Promise<Store[] | null> {
		try {
			return await invoke<Store[]>('get_store_list');
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

	public static async CreateItem(
		storeName: string,
		spaceId: number,
		binId: number,
		name: string
	): Promise<boolean> {
		try {
			await invoke('create_item', { storeName, spaceId, binId, name });
			return true;
		} catch (err: unknown) {
			console.error('Failed to add item: ', err);
			return false;
		}
	}

	public static async GetItemList(storeName: string, binId: number): Promise<Item[] | null> {
		try {
			return await invoke<Item[]>('get_item_list', { storeName, binId });
		} catch (err: unknown) {
			console.error(`Failed to get items: ${err}`);
			return null;
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

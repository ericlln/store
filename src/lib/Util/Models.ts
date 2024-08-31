import type { Point } from '$lib/Mapper/Geometry';

export interface Space {
	id: number;
	name: string;
	drawing: Point[][];
}

export interface Store {
	id: number;
	spaceId: number;
	name: string;
	location: Point;
}

export interface Item {
	id: number;
	storeId: number;
	name: string;
	quantity?: number;
	notes: string;
}

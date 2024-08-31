import type { Point } from "$lib/Mapper/Geometry";
import type { PageLoad } from "./$types";

export const load: PageLoad = ({ url }) => {
	const x = url.searchParams.get("x");
	const y = url.searchParams.get("y");

	if (x !== null && y !== null) {
		return {
			coord: {
				x: parseFloat(x),
				y: parseFloat(y)
			} as Point
		};
	}
};

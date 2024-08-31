import type { ComponentType } from 'svelte';

export class Util {
	public static ComponentToDom(componentClass: ComponentType, props: object): HTMLElement {
		const target = document.createElement('div');
		new componentClass({ target, props });
		return target;
	}
}

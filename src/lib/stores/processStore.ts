import { writable } from 'svelte/store';

export type ProcessState = 'idle' | 'running' | 'stopped' | 'success';

export const processState = writable<ProcessState>('idle');
export const progressValue = writable<number>(0); 
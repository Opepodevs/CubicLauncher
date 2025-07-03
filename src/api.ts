import { invoke } from '@tauri-apps/api/core';

import {
	BackendResponse,
	Instance,
} from "./types";

export function closeLauncher() {
	invoke('close_window')
}

export function hideLauncher() {
	invoke('minimize_window')
}
export function maximizeLauncher() {
	invoke('maximize_window')
}

export async function SaveInstance(instance: Instance): Promise<BackendResponse> {
	const response = await invoke("save_instance", { instance });
	return response as BackendResponse;
}

export async function getInstances(): Promise<BackendResponse> {
	const response = await invoke("get_instances");
	console.log(response)
	return response as BackendResponse;
}
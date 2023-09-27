import { writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import type { Log } from "../types/entities";

export const logStore: Writable<Array<Log>> = writable([]);

export const setLogs = async () => {
  const res: string = await invoke("get_logs");
  const logs = JSON.parse(res);
  logStore.set(logs);
}
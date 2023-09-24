import { get, writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import type { Log } from "../types/entities";
import { personStore } from "./personStore";
import { equipmentStore } from "./equipmentStore";

export const logStore: Writable<Array<Log>> = writable([]);

export const addLog = async ( name: string) => {
  const res = await invoke("add_log", { name });
}

export const setLogs = async () => {
  const res: string = await invoke("get_logs");
  const logs = JSON.parse(res);
  logStore.set(logs);
}
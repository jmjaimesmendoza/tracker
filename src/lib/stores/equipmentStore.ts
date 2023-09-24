import { writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import type { Equipment } from "../types/entities";

export const equipmentStore: Writable<Array<Equipment>> = writable([]);

export const addEquipment = async ( name: string, km: number) => {
  const res = await invoke("add_equipment", { name, km });
}

export const setEquipments = async () => {
  const res: string = await invoke("get_equipments");
  const equipments = JSON.parse(res);
  equipmentStore.set(equipments);
}
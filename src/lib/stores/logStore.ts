import { get, writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import type { Equipment, Log, ParsedLog } from "../types/entities";
import { format } from "date-fns";
import { equipmentStore } from "./equipmentStore";
import { personStore } from "./personStore";

export const logStore: Writable<Array<Log>> = writable([]);
export const parsedLogStore: Writable<Array<ParsedLog>> = writable([]);

export const setLogs = async () => {
  const res: string = await invoke("get_logs");
  const logs = JSON.parse(res);
  logStore.set(logs);
  parseLogs();
};

const parseLogs = () => {
  const parsedLogs = get(logStore).map((log) => {
    const person = get(personStore).find(
      (person) => person.id === log.person_id
    );
    const equipment = get(equipmentStore).find(
      (equipment: Equipment) => equipment.id === log.equipment_id
    );
    return {
      id: log.id,
      name: equipment?.name ?? '',
      person: person?.name ?? '',
      km: log.km,
      job: log.job,
      description: log.description,
      date: format(new Date(log.created_at.replace(/-/g, "/")), "dd/MM/yyyy"),
    };
  });
  parsedLogStore.set(parsedLogs);
}
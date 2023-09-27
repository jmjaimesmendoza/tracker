import { writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import type { Person } from "../types/entities";

export const personStore: Writable<Array<Person>> = writable([]);

export const addPerson = async ( name: string) => {
  const res = await invoke("add_person", { name });
  await setPersons();
}

export const setPersons = async () => {
  const res: string = await invoke("get_persons");
  const persons = JSON.parse(res);
  personStore.set(persons);
}
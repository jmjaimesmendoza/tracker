import { get, writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import type { Equipment } from "../types/entities";
import { logStore } from "./logStore";
import { addDays, differenceInDays, format } from "date-fns";
import _ from "lodash";
import { toast, Toaster, useToasterStore } from "svelte-french-toast"

export const equipmentStore: Writable<Array<Equipment>> = writable([]);

export const addEquipment = async ( name: string, km: number, modelId: number, nserial: string, notes: string) => {
  const res = await invoke("add_equipment", { name, km, modelId, nserial, notes });
  await setEquipments();
}

export const setEquipments = async () => {
  const equipments: Equipment[] = JSON.parse(await invoke("get_equipments"));
  // equipmentStore.set(equipments);
  const revisions: any[] = JSON.parse(await invoke("get_revisions"));
  let parsedEquipments = equipments.map((e) => {
    const revision = revisions.findLast((r) => r.equipment_id === e.id);
    const logs = get(logStore).filter((l) => l.equipment_id === e.id);
    
    const lastLog = logs[logs.length - 1];

    if (!revision || !logs.length) return {...e, lastRevision: "Por determinar", expectedRevisionDate: "Por determinar"};

    if (revision?.tipo === "D") {
      const revisionDate = new Date(revision.target.replace(/-/g, '/'))
      return {...e, lastRevision: format(new Date(lastLog.created_at), "dd/MM/yyy"), expectedRevisionDate: format(revisionDate, "dd/MM/yyyy"), eskeler: revisionDate}
    }

    let avgs = logs.map((l,i) => {
      if (i === logs.length - 1) return;
      
      const dateDiff = differenceInDays(new Date(logs[i+1]?.created_at), new Date(l.created_at));
      
      if (!dateDiff) return;
      
      const kmDiff = logs[i+1]?.km - l.km;
      const kmPerInterval = kmDiff / dateDiff;
      return kmPerInterval;
    })

    avgs = avgs.filter((value) => value != undefined);
    
    // avgs.pop()

    if (avgs.length <= 1) {
      return {...e, lastRevision: format(new Date(lastLog.created_at.replace(/-/g, '/')), "dd/MM/yyy"),expectedRevisionDate: `${revision.target} Km/Horas`}
    };
    const revDiff = parseInt(revision?.target) - lastLog.km;
    const avgkm = _.mean(avgs);
    const estimatedDays = revDiff/avgkm;
    const estimatedRevisionDate = addDays(new Date(lastLog.created_at.replace(/-/g, '/')), estimatedDays);
    return {...e, lastRevision: format(new Date(lastLog.created_at.replace(/-/g, '/')), "dd/MM/yyy"),expectedRevisionDate: `${format(estimatedRevisionDate, "dd/MM/yyyy")} (${revision.target} Km/Horas)`, eskeler: estimatedRevisionDate}
    })
    equipmentStore.set(parsedEquipments);
  }
  
equipmentStore.subscribe((equipments) => {
  toast.remove();
  for (const equipment of equipments) {
    if (equipment.eskeler) {
      const msg = `El equipo ${equipment.name} tiene una revisión programada para el ${format(equipment.eskeler, 'dd/MM/yyyy')}`
      toast(msg, {icon:'⚠️',duration: 99999999, position: "bottom-right"})
    }
  }
})
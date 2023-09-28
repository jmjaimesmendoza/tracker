export type Equipment = {
  id: number;
  name: string;
  km: number;
  brand: string;
  model: string;
  nserial: number;
  notes: string;
  eskeler: Date | null;
};

export type Person = {
  id: number;
  name: string;
}

export type Log = {
  id: number;
  equipment_id: number;
  person_id: number;
  description: string;
  job: string;
  km: number;
  created_at: string;
};
export type Equipment = {
  id: number;
  name: string;
  km: number;
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
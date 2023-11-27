export type Equipment = {
  id: number;
  name: string;
  km: number;
  brand: string;
  model_id: number;
  nserial: string;
  notes: string;
  file_path: string;
  eskeler: Date | null;
};

export type Manual = {
  id: number;
  name: string;
  equipment_id: number;
  file_path: string;
};

export type Person = {
  id: number;
  name: string;
};

export type Log = {
  id: number;
  equipment_id: number;
  person_id: number;
  description: string;
  job: string;
  km: number;
  created_at: string;
};

export type ParsedLog = {
  id: number;
  name: string;
  person: string;
  job: string;
  km: number;
  description: string;
  date: string;
};

export type Model = {
  id: number;
  brand_id: number;
  name: string;
};

export type ParsedModel = {
  id: number;
  brand: string;
  name: string;
};

export type Brand = {
  id: number;
  name: string;
};


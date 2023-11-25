import { invoke } from "@tauri-apps/api";
import { writable, type Writable } from "svelte/store";
import type { Brand, Model, ParsedModel } from "../types/entities";

export const modelStore: Writable<Array<Model>> = writable([]);
export const parsedModelStore: Writable<Array<ParsedModel>> = writable([]);

export const getModels = async () => {
  const models = JSON.parse(await invoke("get_models"));
  const brands = JSON.parse(await invoke("get_brands"));
  const parsedModels = parseModels(models, brands);
  modelStore.set(models);
  parseModels(models, brands);
};

const parseModels = (models: Model[], brands: Brand[]) => {
  const parsedData = models.map((model) => {
    return {
      id: model.id,
      name: model.name,
      brand: brands.find((brand) => brand.id === model.brand_id)?.name ?? 'NO BRAND',
    };
  });
  parsedModelStore.set(parsedData);
};

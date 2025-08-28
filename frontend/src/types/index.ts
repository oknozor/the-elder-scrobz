export * from "./album";
export * from "./api-param";
export * from "./artist";
export * from "./stats";
export * from "./track";
export * from "./user";

export interface Item {
    type: string;
    id: string;
    name: string;
    thumbnail_url: string;
    subsonic_url?: string;
    listens?: number;
}

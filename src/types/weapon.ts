export interface IWeapon {
    name: string,
    level: number,
    ascend: boolean,
    params: any,
    refine: number,
    configUnlinked: any
}

export type WeaponType = "Bow" | "Claymore" | "Sword" | "Catalyst" | "Polearm"

export type WeaponName = string

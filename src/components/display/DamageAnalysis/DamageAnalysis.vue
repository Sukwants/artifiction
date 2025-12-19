<template>
    <div style="margin-bottom: 16px;" class="flex-row" v-if="this.damageType != 'None'">
        <el-radio-group v-model="damageType" style="margin-right: 24px;">
            <el-radio-button v-if="hasDamageNormal" label="DamageNormal">{{ get_name_from_element(this.DamageNormal.element) }}</el-radio-button>
            <el-radio-button v-if="hasDamageMelt" label="DamageMelt">融化</el-radio-button>
            <el-radio-button v-if="hasDamageVaporize" label="DamageVaporize">蒸发</el-radio-button>
            <el-radio-button v-if="hasDamageSpread" label="DamageSpread">蔓激化</el-radio-button>
            <el-radio-button v-if="hasDamageAggravate" label="DamageAggravate">超激化</el-radio-button>
            <el-radio-button v-if="hasTransformativeDamage" label="TransformativeDamage">{{ get_name_from_transformative_type(this.TransformativeDamage.transformative_type) }}</el-radio-button>
            <el-radio-button v-if="hasMoonglareDamage" label="MoonglareDamage">{{ get_name_from_lunar_type(this.MoonglareDamage.lunar_type) }}</el-radio-button>
            <el-radio-button v-if="hasHeal" label="Heal">治疗</el-radio-button>
            <el-radio-button v-if="hasShield" label="Heal">护盾</el-radio-button>
        </el-radio-group>

        <span class="damage-display">{{ calc_result() }}</span>
    </div>

    <div v-if="this.damageType != 'None'" class="header-row" style="overflow: auto; margin-bottom: 16px; min-height: 200px;">

        <div>
            <div class="big-title base-damage-region">基础伤害</div>
            <div class="header-row">
                <damage-analysis-util
                    v-if="result.atk_ratio?.length > 0"
                    :arr="result.atk"
                    title="攻击力"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="result.atk_ratio?.length > 0"
                    :arr="result.atk_ratio"
                    title="攻击力倍率"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="result.def_ratio?.length > 0"
                    :arr="result.def"
                    title="防御力"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="result.def_ratio?.length > 0"
                    :arr="result.def_ratio"
                    title="防御力倍率"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="result.hp_ratio?.length > 0"
                    :arr="result.hp"
                    title="生命值"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="result.hp_ratio?.length > 0"
                    :arr="result.hp_ratio"
                    title="生命值倍率"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="result.em_ratio?.length > 0"
                    :arr="result.em"
                    title="元素精通"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="result.em_ratio?.length > 0"
                    :arr="result.em_ratio"
                    title="元素精通倍率"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="result.base_damage?.length > 0"
                    :arr="result.base_damage"
                    title="其他"
                ></damage-analysis-util>
                <div v-if="result.reaction_base > 0" style="min-width: 100px">
                    <div class="big-title" style="background: rgb(236, 245, 255)">反应基础伤害</div>
                    <div class="header-row" style="height: 100%; display: flex; align-items: center; justify-content: center; min-height: 125px;">
                        <span>{{ result.reaction_base }}</span>
                    </div>
                </div>
                <damage-analysis-util
                    v-if="result.reaction_base > 0"
                    :arr="result.reaction_enhance"
                    title="反应伤害提升"
                ></damage-analysis-util>
            </div>
        </div>

        <div>
            <div class="big-title bonus-region">加成</div>
            <div class="header-row">
                <damage-analysis-util
                    v-if="this.damageType == 'DamageNormal' || this.damageType == 'DamageMelt' || this.damageType == 'DamageVaporize' || this.damageType == 'DamageSpread' || this.damageType == 'DamageAggravate'"
                    :arr="result.bonus"
                    title="伤害加成"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="this.damageType == 'MoonglareDamage'"
                    :arr="result.moonglare_base"
                    title="基础提升"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="this.damageType == 'Heal'"
                    :arr="result.healing_bonus"
                    title="治疗加成"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="this.damageType == 'Heal'"
                    :arr="result.incoming_healing_bonus"
                    title="受治疗加成"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="this.damageType == 'Shield'"
                    :arr="result.shield_strength"
                    title="护盾强效"
                ></damage-analysis-util>
            </div>
        </div>

        <div v-if="this.damageType === 'DamageMelt' || this.damageType === 'DamageVaporize' || this.damageType === 'TransformativeDamage' || this.damageType === 'MoonglareDamage'" >
            <div class="big-title reaction-region">反应区</div>
            <div class="header-row">
                <div style="min-width: 100px">
                    <div class="big-title" style="background: rgb(236, 245, 255)">反应系数</div>
                    <div class="header-row" style="height: 100%; display: flex; align-items: center; justify-content: center; min-height: 125px;">
                        <span>{{ result.reaction_coefficient }}</span>
                    </div>
                </div>
                <damage-analysis-util
                    :arr="result.reaction_enhance"
                    title="反应伤害提升"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="result.reaction_extra?.length > 0"
                    :arr="result.reaction_extra"
                    title="反应额外伤害"
                ></damage-analysis-util>
            </div>
        </div>
    </div>

    <div v-if="this.damageType != 'None'" class="header-row" style="overflow: auto; margin-bottom: 16px; min-height: 200px;">
        <div v-if="this.damageType != 'Shield'">
            <div class="big-title critical-region">暴击区</div>
            <div class="header-row">
                <damage-analysis-util
                    :arr="result.critical_rate"
                    title="暴击率"
                ></damage-analysis-util>
                <damage-analysis-util
                    :arr="result.critical_damage"
                    title="暴击伤害"
                ></damage-analysis-util>
            </div>
        </div>
        <div v-if="this.damageType != 'Heal' && this.damageType != 'Shield'">
            <div class="big-title res-minus">抗性区</div>
            <div class="header-row">
                <damage-analysis-util
                    :arr="result.res_minus"
                    title="减抗"
                ></damage-analysis-util>
            </div>
        </div>
        <div v-if="this.damageType == 'DamageNormal' || this.damageType == 'DamageMelt' || this.damageType == 'DamageVaporize' || this.damageType == 'DamageSpread' || this.damageType == 'DamageAggravate'">
            <div class="big-title def-minus">防御区</div>
            <div class="header-row">
                <damage-analysis-util
                    :arr="result.def_minus"
                    title="减防"
                ></damage-analysis-util>
                <damage-analysis-util
                    :arr="result.def_penetration"
                    title="穿防"
                ></damage-analysis-util>
            </div>
        </div>
    </div>

    <div v-if="this.damageType == 'None'" style="font-size: 16px; color: gray; text-align: center; margin-top: 100px; min-height: 200px;">
        无数据
        <br>
        该技能对应伤害、治疗或护盾在当前配置下不会被触发
    </div>
</template>

<script>
import { get } from "lodash"
import DamageAnalysisUtil from "./DamageAnalysisUtil"
import { LEVEL_MULTIPLIER } from "@/constants/levelMultiplier"

function init_value(result) {
    let temp = {}
    for (let key in result) {
        if (key === "result") {
            continue
        }
        if (typeof result[key] === "number") {
            temp[key] = Math.round(result[key] * 1000) / 1000
            continue
        }
        if (typeof result[key] === "string") {
            temp[key] = result[key]
            continue
        }
        temp[key] = []
        for (let i in result[key]) {
            temp[key].push({
                name: i,
                checked: true,
                value: Math.round(result[key][i] * 1000) / 1000
            })
        }
    }
    return temp
}

function sum(arr) {
    if (!arr) {
        return 0
    }
    let s = 0
    for (let item of arr) {
        if (item.checked) {
            s += parseFloat(item.value)
        }
    }
    return s
}

export default {
    name: "DamageAnalysis",
    components: {
        DamageAnalysisUtil
    },
    props: ["enemyConfig", "characterLevel"],
    data() {
        return {
            damageType: "None",
            DamageNormal: null,
            DamageMelt: null,
            DamageVaporize: null,
            DamageSpread: null,
            DamageAggravate: null,
            TransformativeDamage: null,
            MoonglareDamage: null,
            Heal: null,
            Shield: null,
        }
    },
    methods: {
        setValue(analysis) {
            console.log(analysis)
            
            this.damageType = "None"

            console.log(analysis)

            if (analysis.Shield) {
                this.Shield = init_value(analysis.Shield)
                this.damageType = "Shield"
            } else this.Shield = null
            if (analysis.Heal) {
                this.Heal = init_value(analysis.Heal)
                this.damageType = "Heal"
            } else this.Heal = null
            if (analysis.MoonglareDamage) {
                this.MoonglareDamage = init_value(analysis.MoonglareDamage)
                this.damageType = "MoonglareDamage"
            } else this.MoonglareDamage = null
            if (analysis.TransformativeDamage) {
                this.TransformativeDamage = init_value(analysis.TransformativeDamage)
                this.damageType = "TransformativeDamage"
            } else this.TransformativeDamage = null
            if (analysis.Damage) {
                if (analysis.Damage.aggravate) {
                    this.DamageAggravate = init_value(analysis.Damage.aggravate)
                    this.damageType = "DamageAggravate"
                } else this.DamageAggravate = null
                if (analysis.Damage.spread) {
                    this.DamageSpread = init_value(analysis.Damage.spread)
                    this.damageType = "DamageSpread"
                } else this.DamageSpread = null
                if (analysis.Damage.vaporize) {
                    this.DamageVaporize = init_value(analysis.Damage.vaporize)
                    this.damageType = "DamageVaporize"
                } else this.DamageVaporize = null
                if (analysis.Damage.melt) {
                    this.DamageMelt = init_value(analysis.Damage.melt)
                    this.damageType = "DamageMelt"
                } else this.DamageMelt = null
                if (analysis.Damage.normal) {
                    this.DamageNormal = init_value(analysis.Damage.normal)
                    this.damageType = "DamageNormal"
                } else this.DamageNormal = null
            } else {
                this.DamageNormal = null
                this.DamageMelt = null
                this.DamageVaporize = null
                this.DamageSpread = null
                this.DamageAggravate = null
            }
        },

        resRatio(result) {
            if (result.element == undefined) {
                return 1.0
            }
            const originalRes = this.enemyConfig[result.element.toLowerCase() + "_res"]
            const res = originalRes - sum(result.res_minus)
            let res_ratio
            if (res > 0.75) {
                res_ratio = 1 / (1 + res * 4)
            } else if (res > 0) {
                res_ratio = 1 - res
            } else {
                res_ratio = 1 - res / 2
            }
            return res_ratio
        },

        defMultiplier(result) {
            const enemyLevel = this.enemyConfig.level
            const characterLevel = this.characterLevel
            const c = 100 + characterLevel
            return c / ((1 - sum(result.def_penetration)) * (1 - sum(result.def_minus)) * (100 + enemyLevel) + c)
        },

        calc_Damage(result) {
            let base_damage = sum(result.atk) * sum(result.atk_ratio)
                + sum(result.hp) * sum(result.hp_ratio)
                + sum(result.def) * sum(result.def_ratio)
                + sum(result.em) * sum(result.em_ratio)
                + sum(result.base_damage)
            
            let damage = base_damage
                * (1 + sum(result.bonus))
                * result.reaction_coefficient * (1 + sum(result.reaction_enhance))
                * (1 + sum(result.critical_rate) * sum(result.critical_damage))
                * this.resRatio(result)
                * this.defMultiplier(result)

            return damage
        },

        calc_TransformativeDamage(result) {
            let base_damage = result.reaction_base
            
            let damage = base_damage
                * reaction_coefficient * (1 + sum(result.reaction_enhance))
                * (1 + sum(result.critical_rate) * sum(result.critical_damage))
                * this.resRatio(result)

            return damage
        },

        calc_MoonglareDamage(result) {
            let base_damage = sum(result.atk) * sum(result.atk_ratio)
                + sum(result.hp) * sum(result.hp_ratio)
                + sum(result.def) * sum(result.def_ratio)
                + sum(result.em) * sum(result.em_ratio)
                + sum(result.base)
                + result.reaction_base;

            let damage = base_damage
                * (1 + sum(result.moonglare_base))
                * (1 + sum(result.moonglare_elevate))
                * result.reaction_coefficient * (1 + sum(result.reaction_enhance))
                * (1 + sum(result.critical_rate) * sum(result.critical_damage))
                * this.resRatio(result)

            return damage
        },

        calc_Heal(result) {
            let heal = sum(result.atk) * sum(result.atk_ratio)
                + sum(result.hp) * sum(result.hp_ratio)
                + sum(result.def) * sum(result.def_ratio)
                + sum(result.em) * sum(result.em_ratio)
                + sum(result.base)

            heal = heal
                * (1 + sum(result.heal_bonus))
                * (1 + sum(result.incoming_healing_bonus))
                * (1 + sum(result.critical_rate) * sum(result.critical_damage))

            return heal
        },

        calc_Shield(result) {
            let shield = sum(result.atk) * sum(result.atk_ratio)
                + sum(result.hp) * sum(result.hp_ratio)
                + sum(result.def) * sum(result.def_ratio)
                + sum(result.em) * sum(result.em_ratio)
                + sum(result.base)

            shield = shield
                * (1 + sum(result.shield_strength))

            return shield
        },

        calc_result() {
            return Math.round((() => {
                if (this.damageType == "DamageNormal") {
                    return this.calc_Damage(this.DamageNormal)
                } else if (this.damageType == "DamageMelt") {
                    return this.calc_Damage(this.DamageMelt)
                } else if (this.damageType == "DamageVaporize") {
                    return this.calc_Damage(this.DamageVaporize)
                } else if (this.damageType == "DamageSpread") {
                    return this.calc_Damage(this.DamageSpread)
                } else if (this.damageType == "DamageAggravate") {
                    return this.calc_Damage(this.DamageAggravate)
                } else if (this.damageType == "TransformativeDamage") {
                    return this.calc_TransformativeDamage(this.TransformativeDamage)
                } else if (this.damageType == "MoonglareDamage") {
                    return this.calc_MoonglareDamage(this.MoonglareDamage)
                } else if (this.damageType == "Heal") {
                    return this.calc_Heal(this.Heal)
                } else if (this.damageType == "Shield") {
                    return this.calc_Shield(this.Shield)
                }
            })())
        },
        
        get_name_from_element(element) {
            const map = {
                "Pyro": "火元素伤害",
                "Electro": "雷元素伤害",
                "Hydro": "水元素伤害",
                "Anemo": "风元素伤害",
                "Geo": "岩元素伤害",
                "Dendro": "草元素伤害",
                "Cryo": "冰元素伤害",
                "Physical": "物理伤害"
            }
            return map[element]
        },

        get_name_from_transformative_type(transformative_type) {
            const map = {
                SwirlCryo: "扩散（冰）",
                SwirlPyro: "扩散（火）",
                SwirlHydro: "扩散（水）",
                SwirlElectro: "扩散（雷）",
                Overload: "超载",
                ElectroCharged: "感电",
                Shatter: "碎冰",
                Superconduct: "超导",
                Bloom: "绽放",
                Hyperbloom: "超绽放",
                Burgeon: "烈绽放",
                Burning: "燃烧",
                Crystallize: "结晶盾",
            }
            return map[transformative_type]
        },

        get_name_from_lunar_type(lunar_type) {
            const map = {
                "LunarChargedReaction": "月感电",
                "LunarCharged": "月感电伤害",
                "LunarBloom": "月绽放伤害",
            }
            return map[lunar_type]
        },
    },
    computed: {
        hasDamageNormal() {
            return this.DamageNormal != null
        },
        hasDamageMelt() {
            return this.DamageMelt != null
        },
        hasDamageVaporize() {
            return this.DamageVaporize != null
        },
        hasDamageSpread() {
            return this.DamageSpread != null
        },
        hasDamageAggravate() {
            return this.DamageAggravate != null
        },
        hasTransformativeDamage() {
            return this.TransformativeDamage != null
        },
        hasMoonglareDamage() {
            return this.MoonglareDamage != null
        },
        hasHeal() {
            return this.Heal != null
        },
        hasShield() {
            return this.Shield != null
        },

        result() {
            const map = {
                DamageNormal: this.$data.DamageNormal,
                DamageMelt: this.$data.DamageMelt,
                DamageVaporize: this.$data.DamageVaporize,
                DamageSpread: this.$data.DamageSpread,
                DamageAggravate: this.$data.DamageAggravate,
                TransformativeDamage: this.$data.TransformativeDamage,
                MoonglareDamage: this.$data.MoonglareDamage,
                Heal: this.$data.Heal,
                Shield: this.$data.Shield,
                None: {},
            }

            return map[this.damageType]
        },
    }
}
</script>

<style scoped lang="scss">
.header-row {
    display: flex;
    // align-items: center;
}

.big-title {
    height: 32px;
    display: flex;
    justify-content: center;
    align-items: center;
    min-width: 100px;

    &.base-damage-region {
        background-color: rgb(217, 236, 255);
    }

    &.bonus-region {
        background-color: rgb(179, 216, 255);
    }

    &.lunar-increase-region {
        background-color: rgb(179, 216, 255);
    }

    &.reaction-region {
        background-color: rgb(217, 236, 255);
    }

    &.lunar-extra-region {
        background-color: rgb(179, 216, 255);
    }

    &.critical-region {
        background-color: rgb(179, 216, 255);
    }

    &.res-minus {
        background-color: rgb(217, 236, 255);
    }

    &.def-minus {
        background-color: rgb(179, 216, 255);
    }
}
</style>
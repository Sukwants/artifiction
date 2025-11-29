<template>
    <div style="margin-bottom: 16px;" class="flex-row" v-if="!isNone">
        <el-radio-group v-model="damageType" style="margin-right: 24px;">
            <el-radio-button label="normal">{{ normalDamageName }}</el-radio-button>
            <el-radio-button v-if="showMeltOption" label="melt">融化</el-radio-button>
            <el-radio-button v-if="showVaporizeOption" label="vaporize">蒸发</el-radio-button>
            <el-radio-button v-if="showSpreadOption" label="spread">蔓激化</el-radio-button>
            <el-radio-button v-if="showAggravateOption" label="aggravate">超激化</el-radio-button>
        </el-radio-group>

        <span class="damage-display" v-if="damageType === 'normal'">{{ Math.round(damageNormal) }}</span>
        <span class="damage-display" v-if="damageType === 'melt'">{{ Math.round(damageMelt) }}</span>
        <span class="damage-display" v-if="damageType === 'vaporize'">{{ Math.round(damageVaporize) }}</span>
        <span class="damage-display" v-if="damageType === 'spread'">{{ Math.round(damageSpread) }}</span>
        <span class="damage-display" v-if="damageType === 'aggravate'">{{ Math.round(damageAggravate) }}</span>
    </div>

    <div class="header-row" style="overflow: auto; margin-bottom: 16px; min-height: 200px;" v-if="!isNone">
        <div v-if="lunarType !== 'LunarChargedReaction'">
            <div class="big-title base-damage-region" :title="Math.round(baseDamageSpread*1000)/1000" v-if="damageType === 'spread'">{{ baseRegionName }}</div>
            <div class="big-title base-damage-region" :title="Math.round(baseDamageAggravate*1000)/1000" v-else-if="damageType === 'aggravate'">{{ baseRegionName }}</div>
            <div class="big-title base-damage-region" :title="Math.round(baseDamage*1000)/1000" v-else>{{ baseRegionName }}</div>
            <div class="header-row">
                <damage-analysis-util
                    v-if="atkRatioState.length > 0"
                    :arr="atkState"
                    title="攻击力"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="atkRatioState.length > 0"
                    :arr="atkRatioState"
                    title="攻击力倍率"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="defRatioState.length > 0"
                    :arr="defState"
                    title="防御力"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="defRatioState.length > 0"
                    :arr="defRatioState"
                    title="防御力倍率"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="hpRatioState.length > 0"
                    :arr="hpState"
                    title="生命值"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="hpRatioState.length > 0"
                    :arr="hpRatioState"
                    title="生命值倍率"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="emRatioState.length > 0"
                    :arr="emState"
                    title="元素精通"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="emRatioState.length > 0"
                    :arr="emRatioState"
                    title="元素精通倍率"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="extraDamageState.length > 0"
                    :arr="extraDamageState"
                    title="其他"
                ></damage-analysis-util>
                <div v-if="damageType === 'spread'" style="min-width: 100px">
                    <div class="big-title" style="background: rgb(236, 245, 255)">蔓激化基础伤害</div>
                    <div class="header-row" style="height: 100%; display: flex; align-items: center; justify-content: center">
                        <span>{{ Math.round(baseDamageQuicken * 1000) / 1000 }}</span>
                    </div>
                </div>
                <div v-if="damageType === 'aggravate'" style="min-width: 100px">
                    <div class="big-title" style="background: rgb(236, 245, 255)">超激化基础伤害</div>
                    <div class="header-row" style="height: 100%; display: flex; align-items: center; justify-content: center">
                        <span>{{ Math.round(baseDamageQuicken * 1000) / 1000 }}</span>
                    </div>
                </div>
                <damage-analysis-util
                    v-if="damageType === 'spread'"
                    :arr="spreadState"
                    title="蔓激化伤害提升"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="damageType === 'aggravate'"
                    :arr="aggravateState"
                    title="超激化伤害提升"
                ></damage-analysis-util>
            </div>
        </div>
        <div v-if="lunarType == 'LunarChargedReaction'">
            <div class="big-title base-damage-region">基础伤害</div>
            <div class="header-row" style="height: 80%; display: flex; align-items: center; justify-content: center">
                <span>{{ Math.round(baseDamageReaction * 1000) / 1000 }}</span>
            </div>
        </div>
        <div v-if="!isLunar && !isShield">
            <div class="big-title bonus-region">加成</div>
            <div class="header-row">
                <damage-analysis-util
                    v-if="damageType === 'melt'"
                    :arr="meltBonusState"
                    :title="bonusRegionName"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="damageType === 'vaporize'"
                    :arr="vaporizeBonusState"
                    :title="bonusRegionName"
                ></damage-analysis-util>
                <damage-analysis-util
                    v-if="damageType !== 'melt' && damageType !== 'vaporize'"
                    :arr="bonusRegionState"
                    :title="bonusRegionName"
                ></damage-analysis-util>
            </div>
        </div>
        <div v-if="isLunar">
            <div class="big-title lunar-increase-region">基础提升</div>
            <div v-if="this.lunarType == 'LunarChargedReaction' || this.lunarType == 'LunarCharged'" class="header-row">
                <damage-analysis-util
                    :arr="lunarChargedIncreaseState"
                    title="月感电基础提升"
                ></damage-analysis-util>
            </div>
            <div v-if="this.lunarType == 'LunarBloom'" class="header-row">
                <damage-analysis-util
                    :arr="lunarBloomIncreaseState"
                    title="月绽放基础提升"
                ></damage-analysis-util>
            </div>
        </div>
        <div v-if="damageType === 'melt' || damageType === 'vaporize' || isLunar">
            <div class="big-title reaction-region">反应区</div>
            <div class="header-row">
                <div class="header-row">
                    <div style="min-width: 100px;">
                        <div class="header" style="display: flex; height: 32px; justify-content: center; align-items: center; background: rgb(236, 245, 255);">反应倍率</div>
                        <div style="height: 100%; display: flex; align-items: center; justify-content: center">
                            <span>{{ reactionRatio }}</span>
                        </div>
                    </div>
                </div>
                <div v-if="damageType === 'melt'" class="header-row">
                    <damage-analysis-util
                        :arr="meltEnhanceState"
                        title="融化伤害加成"
                    ></damage-analysis-util>
                </div>
                <div v-if="damageType === 'vaporize'" class="header-row">
                    <damage-analysis-util
                        :arr="vaporizeEnhanceState"
                        title="蒸发伤害加成"
                    ></damage-analysis-util>
                </div>
                <div v-if="this.lunarType == 'LunarChargedReaction' || this.lunarType == 'LunarCharged'" class="header-row">
                    <damage-analysis-util
                        :arr="lunarChargedEnhanceState"
                        title="月感电伤害加成"
                    ></damage-analysis-util>
                </div>
                <div v-if="this.lunarType == 'LunarBloom'" class="header-row">
                    <damage-analysis-util
                        :arr="lunarBloomEnhanceState"
                        title="月绽放伤害加成"
                    ></damage-analysis-util>
                </div>
            </div>
        </div>
        <div v-if="this.lunarType == 'LunarBloom'">
            <div class="big-title lunar-extra-region">额外提升</div>
            <div class="header-row">
                <div v-if="this.lunarType == 'LunarBloom'" class="header-row">
                    <damage-analysis-util
                        :arr="lunarBloomExtraIncreaseState"
                        title="月绽放额外提升"
                    ></damage-analysis-util>
                </div>
            </div>
        </div>
    </div>

    <div v-if="isDamage" class="header-row" style="overflow: auto; margin-bottom: 16px; min-height: 200px;">
        <div>
            <div class="big-title critical-region" :title="Math.round(this.critical * this.criticalDamage * 1000)/1000">暴击区</div>
            <div class="header-row">
                <damage-analysis-util
                    :arr="criticalState"
                    title="暴击率"
                ></damage-analysis-util>
                <damage-analysis-util
                    :arr="criticalDamageState"
                    title="暴击伤害"
                ></damage-analysis-util>
            </div>
        </div>
        <div>
            <div class="big-title res-minus">抗性区</div>
            <div class="header-row">
                <damage-analysis-util
                    :arr="resMinusState"
                    title="减抗"
                ></damage-analysis-util>
            </div>
        </div>
        <div v-if="!isLunar">
            <div class="big-title def-minus">防御区</div>
            <div class="header-row">
                <damage-analysis-util
                    :arr="defMinusState"
                    title="减防"
                ></damage-analysis-util>
                <damage-analysis-util
                    :arr="defPenetrationState"
                    title="穿防"
                ></damage-analysis-util>
            </div>
        </div>
    </div>

    <div v-if="isHeal" class="header-row" style="overflow: auto; margin-bottom: 16px; min-height: 200px;">
        <div>
            <div class="big-title critical-region" :title="Math.round(this.critical * this.criticalDamage * 1000)/1000">暴击区</div>
            <div class="header-row">
                <damage-analysis-util
                    :arr="criticalState"
                    title="暴击率"
                ></damage-analysis-util>
                <damage-analysis-util
                    :arr="criticalDamageState"
                    title="暴击伤害"
                ></damage-analysis-util>
            </div>
        </div>
    </div>

    <div v-if="isNone" style="font-size: 16px; color: gray; text-align: center; margin-top: 100px; min-height: 200px;">
        无数据
        <br>
        该技能对应伤害、治疗或护盾在当前配置下不会被触发
    </div>
</template>

<script>
import DamageAnalysisUtil from "./DamageAnalysisUtil"
import { LEVEL_MULTIPLIER } from "@/constants/levelMultiplier"

function sum(arr) {
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
            damageType: "normal",
            element: "Pyro",
            isHeal: false,
            isShield: false,
            isDamage: true,
            isLunar: false,
            lunarType: "None",

            atkState: [{ name: "test", value: 1000, checked: true }],
            atkRatioState: [{ name: "test", value: 1000, checked: true }],
            defState: [],
            defRatioState: [],
            hpState: [],
            hpRatioState: [],
            emState: [],
            emRatioState: [],
            extraDamageState: [],
            spreadState: [],
            aggravateState: [],
            criticalState: [],
            criticalDamageState: [],
            meltEnhanceState: [],
            vaporizeEnhanceState: [],
            lunarChargedEnhanceState: [],
            lunarBloomEnhanceState: [],
            lunarChargedIncreaseState: [],
            lunarBloomIncreaseState: [],
            lunarChargedExtraIncreaseState: [],
            lunarBloomExtraIncreaseState: [],
            defMinusState: [],
            defPenetrationState: [],
            resMinusState: [],
            bonusState: [],
            meltBonusState: [],
            vaporizeBonusState: [],
            healingBonusState: []
        }
    },
    methods: {
        setValue(analysis) {
            console.log(analysis)
            let map = {
                "atkState": "atk",
                "atkRatioState": "atk_ratio",
                "defState": "def",
                "defRatioState": "def_ratio",
                "hpState": "hp",
                "hpRatioState": "hp_ratio",
                "emState": "em",
                "emRatioState": "em_ratio",
                "extraDamageState": "extra_damage",
                "criticalState": "critical",
                "criticalDamageState": "critical_damage",
                "meltEnhanceState": "melt_enhance",
                "vaporizeEnhanceState": "vaporize_enhance",
                "lunarChargedEnhanceState": "lunar_charged_enhance",
                "lunarBloomEnhanceState": "lunar_bloom_enhance",
                "lunarChargedIncreaseState": "lunar_charged_increase",
                "lunarBloomIncreaseState": "lunar_bloom_increase",
                "lunarChargedExtraIncreaseState": "lunar_charged_extra_increase",
                "lunarBloomExtraIncreaseState": "lunar_bloom_extra_increase",
                "bonusState": "bonus",
                "meltBonusState": "melt_bonus",
                "vaporizeBonusState": "vaporize_bonus",
                "defMinusState": "def_minus",
                "defPenetrationState": "def_penetration",
                "resMinusState": "res_minus",
                "healingBonusState": "healing_bonus",
                "aggravateState": "aggravate_compose",
                "spreadState": "spread_compose",
            }
            this.element = analysis.element
            this.isHeal = analysis.is_heal
            this.isShield = analysis.is_shield
            this.isNone = analysis.is_none
            this.isDamage = !this.isHeal && !this.isShield && !this.isNone
            this.isLunar = analysis.lunar_type !== "None"
            this.lunarType = analysis.lunar_type
            this.damageType = "normal"
            for (let key in map) {
                let fromKey = map[key]
                let temp = []
                for (let i in analysis[fromKey]) {
                    temp.push({
                        name: i,
                        checked: true,
                        value: Math.round(analysis[fromKey][i] * 1000) / 1000
                    })
                }
                this[key] = temp
            }
        }
    },
    computed: {
        normalDamageName() {
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
            const lunarMap = {
                "LunarChargedReaction": "月感电",
                "LunarCharged": "月感电伤害",
                "LunarBloom": "月绽放伤害",
            }
            if (this.isLunar) {
                return lunarMap[this.lunarType]
            } else if (this.isHeal) {
                return "治疗量"
            } else if (this.isShield) {
                return "护盾量"
            } else {
                return map[this.element]
            }
        },

        showMeltOption() {
            return (this.element === "Cryo" || this.element === "Pyro") && this.isDamage && !this.isLunar
        },

        showVaporizeOption() {
            return (this.element === "Pyro" || this.element === "Hydro") && this.isDamage && !this.isLunar
        },

        showSpreadOption() {
            return this.element === "Dendro" && this.isDamage && !this.isLunar
        },

        showAggravateOption() {
            return this.element === "Electro" && this.isDamage && !this.isLunar
        },
        
        baseRegionName() {
            if (this.isHeal) {
                return "基础治疗"
            } else if (this.isShield) {
                return "基础护盾"
            } else {
                return "基础伤害"
            }
        },

        bonusRegionState() {
            if (this.isHeal) {
                return this.healingBonusState
            } else {
                return this.bonusState
            }
        },

        bonusRegionName() {
            if (this.isHeal) {
                return "治疗加成"
            } else {
                return "伤害加成"
            }
        },

        reactionRatio() {
            if(this.lunarType !== "None") {
                let map = {
                    "LunarChargedReaction": 1.8,
                    "LunarCharged": 3.0,
                    "LunarBloom": 1.0,
                }

                return map[this.lunarType]
            } else {
                let map = {
                    "Cryomelt": 1.5,
                    "Pyromelt": 2,
                    "Pyrovaporize": 1.5,
                    "Hydrovaporize": 2,
                }
    
                return map[this.element + this.damageType]
            }
        },

        atk() {
            return sum(this.atkState)
        },

        atkRatio() {
            return sum(this.atkRatioState)
        },

        def() {
            return sum(this.defState)
        },

        defRatio() {
            return sum(this.defRatioState)
        },

        hp() {
            return sum(this.hpState)
        },

        hpRatio() {
            return sum(this.hpRatioState)
        },

        em() {
            return sum(this.emState)
        },

        emRatio() {
            return sum(this.emRatioState)
        },

        extraDamage() {
            return sum(this.extraDamageState)
        },

        bonus() {
            return sum(this.bonusState)
        },
        meltBonus() {
            return sum(this.meltBonusState)
        },
        vaporizeBonus() {
            return sum(this.vaporizeBonusState)
        },

        healingBonus() {
            return sum(this.healingBonusState)
        },

        critical() {
            return Math.min(sum(this.criticalState), 1)
        },

        criticalDamage() {
            return sum(this.criticalDamageState)
        },

        meltEnhance() {
            return sum(this.meltEnhanceState)
        },

        vaporizeEnhance() {
            return sum(this.vaporizeEnhanceState)
        },

        defMinus() {
            return sum(this.defMinusState)
        },

        defPenetration() {
            return sum(this.defPenetrationState)
        },

        resMinus() {
            return sum(this.resMinusState)
        },

        baseDamage() {
            return this.atk * this.atkRatio + this.def * this.defRatio + this.hp * this.hpRatio + this.em * this.emRatio + this.extraDamage;
        },

        spreadEnhance() {
            return sum(this.spreadState)
        },

        aggravateEnhance() {
            return sum(this.aggravateState)
        },

        lunarChargedEnhance() {
            return sum(this.lunarChargedEnhanceState)
        },
        lunarChargedIncrease() {
            return sum(this.lunarChargedIncreaseState)
        },

        lunarBloomEnhance() {
            return sum(this.lunarBloomEnhanceState)
        },
        lunarBloomIncrease() {
            return sum(this.lunarBloomIncreaseState)
        },
        lunarBloomExtraIncrease() {
            return sum(this.lunarBloomExtraIncreaseState)
        },

        baseDamageSpread() {
            return this.baseDamage + LEVEL_MULTIPLIER[this.characterLevel - 1] * 1.25 * (1 + this.spreadEnhance)
        },

        baseDamageAggravate() {
            return this.baseDamage + LEVEL_MULTIPLIER[this.characterLevel - 1] * 1.15 * (1 + this.aggravateEnhance)
        },

        baseDamageReaction() {
            return LEVEL_MULTIPLIER[this.characterLevel - 1];
        },

        baseDamageQuicken() {
            return LEVEL_MULTIPLIER[this.characterLevel - 1] * (this.damageType === "spread" ? 1.25 : 1.15)
        },

        resRatio() {
            // default res to 0.1
            // console.log(this.enemyConfig)
            const originalRes = this.enemyConfig[this.element.toLowerCase() + "_res"]
            const res = originalRes - this.resMinus
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

        defMultiplier() {
            const enemyLevel = this.enemyConfig.level
            const characterLevel = this.characterLevel
            const c = 100 + characterLevel
            return c / ((1 - this.defPenetration) * (1 - this.defMinus) * (100 + enemyLevel) + c)
        },

        damageSpread() {
            return this.baseDamageSpread * (1 + this.critical * this.criticalDamage) * (1 + this.bonus) * this.resRatio * this.defMultiplier
        },

        damageAggravate() {
            return this.baseDamageAggravate * (1 + this.critical * this.criticalDamage) * (1 + this.bonus) * this.resRatio * this.defMultiplier
        },

        damageNormal() {
            let d
            if (this.isHeal) {
                d = this.baseDamage * (1 + this.healingBonus) * (1 + this.critical * this.criticalDamage)
            } else if (this.isShield){
                d  = this.baseDamage
            } else if (this.isLunar) {
                if (this.lunarType == "LunarChargedReaction") {
                    d = this.baseDamageReaction * (1 + this.lunarChargedIncrease) * 1.8 * (1 + this.lunarChargedEnhance) * (1 + this.critical * this.criticalDamage) * this.resRatio
                } else if (this.lunarType == "LunarCharged") {
                    d = this.baseDamage * (1 + this.lunarChargedIncrease) * 3.0 * (1 + this.lunarChargedEnhance) * (1 + this.critical * this.criticalDamage) * this.resRatio
                } else if (this.lunarType == "LunarBloom") {
                    d = (this.baseDamage * (1 + this.lunarBloomIncrease) * (1 + this.lunarBloomEnhance) + this.lunarBloomExtraIncrease) * (1 + this.critical * this.criticalDamage) * this.resRatio
                }
                else d = NaN
            } else {
                d = this.baseDamage * (1 + this.critical * this.criticalDamage) * (1 + this.bonus) * this.resRatio * this.defMultiplier
            }
            return d
        },

        damageMelt() {
            const d = this.baseDamage * (1 + this.critical * this.criticalDamage) * (1 + this.meltBonus) * this.resRatio * this.defMultiplier * this.reactionRatio * (1 + this.meltEnhance)
            return d
        },

        damageVaporize() {
            const d = this.baseDamage * (1 + this.critical * this.criticalDamage) * (1 + this.vaporizeBonus) * this.resRatio * this.defMultiplier * this.reactionRatio * (1 + this.vaporizeEnhance)
            return d
        }
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
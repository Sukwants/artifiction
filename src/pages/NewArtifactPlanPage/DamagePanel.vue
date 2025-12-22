<template>
    <div>
        <el-table
            :data="tableData"
        >
            <el-table-column
                prop="name"
                :label="t('misc.type1')"
            >
            </el-table-column>
            <el-table-column
                prop="expectation"
                :label="t('dmg.expect')"
            ></el-table-column>
            <el-table-column
                prop="critical"
                :label="t('dmg.crit')"
            ></el-table-column>
            <el-table-column
                prop="nonCritical"
                :label="t('dmg.nonCrit')"
            ></el-table-column>
        </el-table>
    </div>
</template>

<script>
import {useI18n} from "@/i18n/i18n";

function toSnake(str) {
    return str.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
}

export default {
    name: "DamageList",
    props: {
        analysisFromWasm: {}
    },
    computed: {
        element() {
            return this.analysisFromWasm.element
        },

        tableData() {
            if (this.analysisFromWasm.is_none) {
                return []
            }
            let temp = []
            const NO_DATA = "无数据"

            const r = (x) => Math.round(x)

            const push = (result, title) => {
                temp.push({
                    expectation: r(result?.expectation) ?? NO_DATA,
                    critical: r(result?.critical) ?? NO_DATA,
                    nonCritical: r(result?.non_critical) ?? NO_DATA,
                    name: title,
                })
            }

            if (this.analysisFromWasm.Heal != undefined) {
                push(this.analysisFromWasm.Heal.result, this.t("dmg.heal"))
            } else if (this.analysisFromWasm.Shield != undefined) {
                push(this.analysisFromWasm.Shield.result, this.t("dmg.shield"))
            } else if (this.analysisFromWasm.TransformativeDamage != undefined) {
                push(this.analysisFromWasm.TransformativeDamage.result, this.t(`dmg.${toSnake(this.analysisFromWasm.TransformativeDamage.transformative_type)}`))
            } else if (this.analysisFromWasm.MoonglareDamage != undefined) {
                push(this.analysisFromWasm.MoonglareDamage.result, this.t(`dmg.${toSnake(this.analysisFromWasm.MoonglareDamage.lunar_type)}`))
            } else {

                push(this.analysisFromWasm.Damage.normal.result, this.t(`dmg.${this.analysisFromWasm.Damage.normal.element}`))

                if (this.analysisFromWasm.Damage.melt) {
                    push(this.analysisFromWasm.Damage.melt.result, this.t("dmg.melt"))
                }
                if (this.analysisFromWasm.Damage.vaporize) {
                    push(this.analysisFromWasm.Damage.vaporize.result, this.t("dmg.vaporize"))
                }
                if (this.analysisFromWasm.Damage.spread) {
                    push(this.analysisFromWasm.Damage.spread.result, this.t("dmg.spread"))
                }
                if (this.analysisFromWasm.Damage.aggravate) {
                    push(this.analysisFromWasm.Damage.aggravate.result, this.t("dmg.aggravate"))
                }
            }

            return temp
        }
    },
    setup() {
        const { t } = useI18n()

        return {
            t
        }
    }
}
</script>

<style scoped lang="scss">
.item {
    height: 32px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 14px;

    &:hover {
        background-color: rgb(241, 241, 241);
    }

    .name {
        
    }

    .numbers {
        display: flex;
        gap: 4px;
    }

    .number {
        padding: 4px;
        border-radius: 3px;
    }

    .melt {
        color: rgb(63, 63, 63);
        // background-color: rgb(155, 218, 255);
        background-image: url("@image/misc/cryo");
        // background-size: 48px;
        background-position-x: -20px;
        background-position-y: -30px;
        background-repeat: no-repeat;
    }

    .pyro {
        color: rgb(255, 95, 95);
        background-color: rgb(255, 224, 224);
    }

    .physical {
        color: rgb(71, 71, 71);
        background-color: rgb(218, 218, 218);
    }
}
</style>
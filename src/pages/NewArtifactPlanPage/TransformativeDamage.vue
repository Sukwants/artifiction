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

export default {
    name: "TransformativeDamage",
    props: {
        data: {}
    },
    computed: {
        tableData() {
            let temp = []

            const r = (x) => Math.round(x)
            
            const push = (name) => {
                temp.push({
                    expectation: r(this.data[name]?.TransformativeDamage?.result?.expectation) ?? this.t('misc.noData'),
                    critical: r(this.data[name]?.TransformativeDamage?.result?.critical) ?? this.t('misc.noData'),
                    nonCritical: r(this.data[name]?.TransformativeDamage?.result?.non_critical) ?? this.t('misc.noData'),
                    name: this.t(`dmg.${name}`),
                })
            }

            for(let name in this.data) {
                push(name)
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
import assert from "node:assert/strict"
import {defineProjectTestSuite} from "../../framework"

export default defineProjectTestSuite({
    id: "project-smoke",
    name: "项目基础冒烟测试集",
    description: "验证项目页面可用并能进入计算器。",
    cases: [
        {
            id: "api-ready",
            name: "API 已就绪",
            async run({api}) {
                assert.equal(api.version, "1.1.0")
                const capabilities = await api.capabilities()
                assert.equal(capabilities.calculator, true)
            }
        },
        {
            id: "calculator-navigation",
            name: "进入计算器页面",
            async run({navigate}) {
                const route = await navigate("calculator")
                assert.equal(route.page, "calculator")
            }
        }
    ]
})

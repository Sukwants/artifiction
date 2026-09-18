import {nextTick} from "vue"
import router from "@/router/router"
import {MonaApiError} from "./errors"
import {emitMonaApiEvent} from "./events"
import {waitForCalculatorActive} from "./calculatorRuntime"

const pagePaths = Object.freeze({
    accounts: "/account",
    artifacts: "/artifacts",
    artifactLoadouts: "/artifacts-kumi",
    presets: "/presets",
    calculator: "/calculate",
})

export type MonaApiPage = keyof typeof pagePaths

async function waitForPageRendered(path: string, timeout = 5000) {
    const deadline = Date.now() + timeout
    while (Date.now() < deadline) {
        await nextTick()
        const element = Array.from(document.querySelectorAll<HTMLElement>("[data-mona-route]"))
            .find(item => item.dataset.monaRoute === path)
        if (element && !element.classList.contains("el-fade-in-linear-enter-active")) return
        await new Promise(resolve => setTimeout(resolve, 16))
    }
    throw new MonaApiError("PAGE_NOT_READY", `Page ${path} did not finish rendering in time`)
}

export const navigationApi = Object.freeze({
    async enter(page: MonaApiPage) {
        if (!Object.prototype.hasOwnProperty.call(pagePaths, page)) {
            throw new MonaApiError("UNSUPPORTED_PAGE", `Page ${String(page)} does not have an automation API`)
        }

        await router.isReady()
        await router.push(pagePaths[page])
        await waitForPageRendered(pagePaths[page])

        if (page === "calculator") await waitForCalculatorActive()

        const route = router.currentRoute.value
        const result = {
            page,
            path: route.path,
            routeName: typeof route.name === "string" ? route.name : null,
        }
        emitMonaApiEvent("navigation.entered", result)
        return result
    },
})

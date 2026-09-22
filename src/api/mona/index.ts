import {accountsApi, artifactLoadoutsApi, artifactsApi, presetsApi, waitForMonaData} from "./domains"
import {calculatorApi} from "./calculator"
import {monaApiEvents} from "./events"
import {metaApi} from "./meta"
import {navigationApi} from "./navigation"

export const monaApi = Object.freeze({
    version: "1.1.0",
    async ready() { await waitForMonaData() },
    async capabilities() {
        return {
            accounts: true,
            artifacts: true,
            artifactLoadouts: true,
            presets: true,
            calculator: true,
            navigation: true,
            damageAnalysis: "dynamic-tree",
        }
    },
    accounts: accountsApi,
    artifacts: artifactsApi,
    artifactLoadouts: artifactLoadoutsApi,
    presets: presetsApi,
    navigation: navigationApi,
    calculator: calculatorApi,
    meta: metaApi,
    events: monaApiEvents,
})

export type MonaApi = typeof monaApi

export function installMonaApi() {
    window.monaApi = monaApi
}

declare global {
    interface Window {
        monaApi: MonaApi
    }
}

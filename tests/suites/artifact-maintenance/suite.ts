import assert from "node:assert/strict"
import {defineProjectTestSuite} from "../../framework"

export default defineProjectTestSuite({
    id: "artifact-maintenance",
    name: "圣遗物仓库清空",
    description: "验证页面清空同时删除套装、保留收藏夹，并隔离其他账号。",
    cases: [{
        id: "clear-artifacts-and-loadouts",
        name: "清空确认、套装清理与账号持久化",
        async run({api, page, navigate, reload, expectError}) {
            const account = await api.accounts.getCurrent()
            const other = await api.accounts.create({name: "keep-other-account"})
            const artifactInput = {
                setName: "gladiatorFinale", position: "flower", star: 5, level: 20,
                mainTag: {name: "lifeStatic", value: 4780},
                normalTags: [{name: "criticalDamage", value: 0.14}],
            }
            await api.accounts.switch(other.id)
            const keptArtifact = await api.artifacts.create(artifactInput)
            const keptLoadout = await api.artifactLoadouts.create({
                folderId: 0, name: "keep-me", artifactIds: [keptArtifact.id, null, null, null, null],
            })

            await api.accounts.switch(account.id)
            const artifact = await api.artifacts.create(artifactInput)
            const folder = await api.artifactLoadouts.folders.create({name: "keep-folder-name"})
            const folders = await api.artifactLoadouts.folders.list()
            const loadoutIds: number[] = []
            for (const folderId of [0, 1, folder.id]) {
                const loadout = await api.artifactLoadouts.create({
                    folderId, name: `remove-${folderId}`, artifactIds: [artifact.id, null, null, null, null],
                })
                loadoutIds.push(loadout.id)
            }

            await page.setViewport({width: 1440, height: 900})
            await navigate("artifacts")
            await page.waitForSelector('[data-testid="clear-artifacts"]', {visible: true})
            await page.click('[data-testid="clear-artifacts"]')
            await page.waitForSelector('.el-popconfirm__action .el-button--primary', {visible: true})
            // Dismissal must leave both artifacts and saved sets intact.
            await page.click('.el-popconfirm__action .el-button:not(.el-button--primary)')
            await page.waitForSelector('.el-popconfirm', {hidden: true})
            assert.equal((await api.artifacts.list()).length, 1)
            assert.equal((await api.artifactLoadouts.list()).length, 3)

            await page.click('[data-testid="clear-artifacts"]')
            await page.waitForSelector('.el-popconfirm__action .el-button--primary', {visible: true})
            await page.click('.el-popconfirm__action .el-button--primary')
            await page.waitForFunction(async () => (await (window as any).monaApi.artifacts.list()).length === 0)
            assert.deepEqual(await api.artifactLoadouts.list(), [])
            for (const id of loadoutIds) await expectError("artifactLoadouts.get", "NOT_FOUND", id)
            const clearedFolders = folders.map((item: any) => ({...item, children: []}))
            assert.deepEqual(await api.artifactLoadouts.folders.list(), clearedFolders)
            for (const item of clearedFolders) {
                assert.deepEqual(await api.artifactLoadouts.list({folderId: item.id}), [])
            }

            // Switching waits for pending writes, then verifies account isolation and reload persistence.
            await api.accounts.switch(other.id)
            assert.deepEqual(await api.artifacts.get(keptArtifact.id), keptArtifact)
            assert.deepEqual(await api.artifactLoadouts.get(keptLoadout.id), keptLoadout)
            await api.accounts.switch(account.id)
            await api.ready()
            await reload()
            assert.deepEqual(await api.artifacts.list(), [])
            assert.deepEqual(await api.artifactLoadouts.list(), [])
            assert.deepEqual(await api.artifactLoadouts.folders.list(), clearedFolders)
        },
    }],
})

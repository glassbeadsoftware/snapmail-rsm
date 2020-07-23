// See README.md for prerequisites for this to run

const { Orchestrator, Config } = require('../../tryorama-rsm/src')

const testDna = Config.dna("snapmail.dna.gz")

const config = Config.gen({
    tester: testDna,
})

const orchestrator = new Orchestrator()

// orchestrator.registerScenario('list dnas', async (s, t) => {
//     const { alex } = await s.players({ alex: config })
//     await alex.spawn()
//
//     console.log('\n\n LIST DNAS \n\n')
//     const dnas = await alex.admin().listDnas()
//     console.log('dnas', dnas)
//
//     t.equal(dnas.length, 1)
// })

// orchestrator.registerScenario('call zome', async (s, t) => {
//     const { alex } = await s.players({ alex: config })
//     await alex.spawn()
//
//     const result = await alex.call('tester', 'foo', 'foo', { anything: 'goes' })
//     console.log('result', result)
//
//     t.equal(result, 'foo')
// })
//
orchestrator.registerScenario('state dump', async (s, t) => {
    const { alex } = await s.players({ alex: config })
    await alex.spawn()

    const dump = await alex.stateDump('tester')
    console.log('dump', JSON.stringify(dump))
    t.equal(dump.length, 3)
    t.ok(typeof dump[0].element === 'object')
    t.ok(typeof dump[1].element === 'object')
    t.ok(typeof dump[2].element === 'object')
})

orchestrator.run()
// See README.md for prerequisites for this to run

const { Orchestrator, Config } = require('../../tryorama-rsm/src')

const testDna = Config.dna("../snapmail.dna.gz")

const config = Config.gen({
    tester: testDna,
})

const orchestrator = new Orchestrator()

orchestrator.registerScenario('call zome', async (s, t) => {
    const { alex } = await s.players({ alex: config })
    await alex.spawn()

    //const result = await alex.call('tester', 'snapmail', 'foo', { anything: 'goes' })
    //const result = await alex.call('tester', 'snapmail', 'whoami', undefined)
    //console.log('agent_pubkey:', result.agent_pubkey.hash.toString())
    const data_string = "0123465789".repeat(1 * 1024 * 1024 / 10)
    const chunk = {
        data_hash: "Qm324rdx",
        chunk_index: 0,
        chunk: data_string,
    };
    //const data_string = "toto";
    const result = await alex.call('tester', 'snapmail', 'write_chunk', chunk)
    console.log('result1:', result)
    let entry_hash = [...result.hash];
    console.log('result1 hash:', entry_hash)
    //t.equal(result, 'foo')

    // Get Entry
    // =========
    let arg = {
        hash: {"type": "Buffer", "data": [71,198,64,23,140,236,238,52,45,24,23,49,174,76,245,96,159,177,79,237,236,216,152,112,146,158,213,243,212,178,164,145,204,155,174,205]},// result,
        hash_type: {'1': null},
    }
    //console.log('arg:', arg)
    const result2 = await alex.call('tester', 'snapmail', 'get_chunk', entry_hash)
    console.log('result2:', result2)
    t.equal(result2, data_string)
})

orchestrator.run()
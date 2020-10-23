// See README.md for prerequisites for this to run

const { Orchestrator, Config } = require('../../tryorama-rsm/src')
//const { Orchestrator, Config } = require('../../tryorama/src')

const ALEX_NICK = 'alice'
const BILLY_NICK = 'billy'

//const testDna = Config.dna("../snapmail.dna.gz")

const config = Config.gen({
    //tester: testDna,
    [ALEX_NICK]: Config.dna("../snapmail.dna.gz", null),
    [BILLY_NICK]: Config.dna("../snapmail.dna.gz", null),
})

const orchestrator = new Orchestrator()

/**
 *
 */
/*
orchestrator.registerScenario('write/get chunk', async (s, t) => {
    const { alex } = await s.players({ alex: config })
    await alex.spawn()
    const [_dnaHash, agentAddress] = alex.cellId(ALEX_NICK)
    console.log({agentAddress})
    // const dump = await alex.stateDump(ALEX_NICK)
    // console.log(dump)

    const result = await alex.call(ALEX_NICK, 'snapmail', 'whoami', undefined)
    //console.log({result})
    console.log('agent_latest_pubkey: ', result.agent_latest_pubkey.hash)

    //console.log('agent_pubkey:', result.agent_pubkey.hash.toString())
    const chunk_size = 1 * 1024 * 1024;
    const data_string = "0123465789".repeat( chunk_size / 10)
    //const data_string = "toto"
    const chunk = {
        data_hash: "Qm324rdx90ABC",
        chunk_index: 0,
        chunk: data_string,
    };

    const start = Date.now();
    let loop_avg = [];

    for (let i = 0 ; i < 2; ++i) {
        const loop_start = Date.now();
        let result = await alex.call(ALEX_NICK, 'snapmail', 'write_chunk', chunk)
        const write_end = Date.now();
        console.log( '['+(write_end - start) +'] (' + (write_end - loop_start)  + ') result0:' + JSON.stringify(result))
        result = await alex.call(ALEX_NICK, 'snapmail', 'get_chunk_hash', chunk)
        console.log('result1:', result)
        //let entry_hash = [...result.hash];
        //console.log('result1 hash:', entry_hash)
        //t.equal(result, 'foo')
        const result2 = await alex.call(ALEX_NICK, 'snapmail', 'get_chunk', result)
        const loop_end = Date.now();
        loop_avg.push(loop_end - loop_start);
        console.log('['+ (loop_end - loop_start) +'] (' + (loop_end - write_end)  + ') result2:', result2.length)
        t.equal(result2, data_string)
    }
    const end = Date.now();
    const avg = loop_avg.reduce(function(a, b) {
        return a + b;
    }) / loop_avg.length;
    const speed = avg / (chunk_size / 1024 / 1024);
    console.log('['+(end - start) +'] Chunk average: ' + avg + ' ms ; speed = ' + speed + ' ms / MiB');
})
*/

/**
 *
 */
orchestrator.registerScenario('send chunk', async (s, t) => {
    // const { alex, billy } = await s.players({ alex: config, billy: config })
    // await alex.spawn()
    // await billy.spawn()

    const { conductor } = await s.players({ conductor: config })
    await conductor.spawn()

    const [_dnaHash, alexAgentAddress] = conductor.cellId(ALEX_NICK)
    const [_dnaHash2, billyAgentAddress] = conductor.cellId(BILLY_NICK)

    //console.log({billy})
    //console.log("billy: " + JSON.stringify(billy));
    //console.log("public_address: " + billy._conductor.agents[0].public_address);

    const billyInfo = await conductor.call(BILLY_NICK, 'snapmail', 'whoami', undefined)
    console.log('Billy agent_pubkey: ', billyInfo.agent_latest_pubkey.hash)

    const accessResult = await conductor.call(ALEX_NICK, 'snapmail', 'set_access', undefined)
    console.log({accessResult})
    const accessResult2 = await conductor.call(BILLY_NICK, 'snapmail', 'set_access', undefined)
    console.log({accessResult2})

    // const result = await conductor.call(BILLY_NICK, 'snapmail', 'whoarethey', alexAgentAddress)
    // console.log({result})


    //const data_string = "0123465789".repeat(100 * 1024 * 1024 / 10)
    const data_string = "toto";
    const chunk = {
        data_hash: "Qm324rdx",
        chunk_index: 0,
        chunk: data_string,
    };
    const sendChunk = {
        agent_pubkey: billyInfo.agent_latest_pubkey,
        //agent_pubkey: billyAgentAddress,
        file_chunk: chunk,
    }
    console.log('sendChunk: ', sendChunk)
    const result = await conductor.call(ALEX_NICK, 'snapmail', 'send_chunk', sendChunk)
    console.log('result1: ', result)
    let entry_hash = [...result.hash];
    console.log('result1 hash:', entry_hash)
    //t.equal(result, 'foo')

    // await s.consistency()

    // Get Entry
    // =========
    const hashResult = await conductor.call(BILLY_NICK, 'snapmail', 'get_chunk_hash', chunk)
    console.log('hashResult:', hashResult)
    //console.log('arg:', arg)
    const result2 = await conductor.call(BILLY_NICK, 'snapmail', 'get_chunk', hashResult)
    console.log('result2:', result2)
    t.equal(result2, data_string)
})

orchestrator.run()
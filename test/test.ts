// See README.md for prerequisites for this to run

const { Orchestrator, Config } = require('../../tryorama-rsm/src')

const testDna = Config.dna("../snapmail.dna.gz")

const config = Config.gen({
    tester: testDna,
})

const orchestrator = new Orchestrator()

orchestrator.registerScenario('write/get chunk', async (s, t) => {
    const { alex } = await s.players({ alex: config })
    await alex.spawn()

    //const result = await alex.call('tester', 'snapmail', 'foo', { anything: 'goes' })
    //const result = await alex.call('tester', 'snapmail', 'whoami', undefined)
    //console.log('agent_pubkey:', result.agent_pubkey.hash.toString())
    const chunk_size = 1 * 1024 * 1024;
    const data_string = "0123465789".repeat( chunk_size / 10)
    //const data_string = "toto"
    const chunk = {
        data_hash: "Qm324rdx",
        chunk_index: 0,
        chunk: data_string,
    };
    //const data_string = "toto";
    const start = Date.now();
    let loop_avg = [];

    for (let i = 0 ; i < 10; ++i) {
        const loop_start = Date.now();
        let result = await alex.call('tester', 'snapmail', 'write_chunk', chunk)
        const write_end = Date.now();
        console.log( '['+(write_end - start) +'] (' + (write_end - loop_start)  + ') result0:' + JSON.stringify(result))
        result = await alex.call('tester', 'snapmail', 'get_chunk_hash', chunk)
        //console.log('result1:', result)
        let entry_hash = [...result.hash];
        //console.log('result1 hash:', entry_hash)
        //t.equal(result, 'foo')

        // Get Entry
        // =========
        let arg = {
            hash: {
                "type": "Buffer",
                "data": [71, 198, 64, 23, 140, 236, 238, 52, 45, 24, 23, 49, 174, 76, 245, 96, 159, 177, 79, 237, 236, 216, 152, 112, 146, 158, 213, 243, 212, 178, 164, 145, 204, 155, 174, 205]
            },// result,
            hash_type: {'1': null},
        }
        //console.log('arg:', arg)
        const result2 = await alex.call('tester', 'snapmail', 'get_chunk', entry_hash)
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

// orchestrator.registerScenario('send chunk', async (s, t) => {
//     const { alex, billy } = await s.players({ alex: config, billy: config })
//     await alex.spawn()
//     await billy.spawn()
//
//     //console.log({billy})
//     //console.log("billy: " + JSON.stringify(billy));
//     //console.log("public_address: " + billy._conductor.agents[0].public_address);
//
//     const billyInfo = await billy.call('tester', 'snapmail', 'whoami', undefined)
//     console.log('agent_pubkey: ', billyInfo.agent_pubkey.hash)
//
//     //const data_string = "0123465789".repeat(100 * 1024 * 1024 / 10)
//     const data_string = "toto";
//     const chunk = {
//         data_hash: "Qm324rdx",
//         chunk_index: 0,
//         chunk: data_string,
//     };
//     const sendChunk = {
//         agent_pubkey: billyInfo.agent_pubkey,
//         file_chunk: chunk,
//     }
//     console.log('sendChunk: ', sendChunk)
//     const result = await alex.call('tester', 'snapmail', 'send_chunk', sendChunk)
//     console.log('result1: ', result)
//     let entry_hash = [...result.hash];
//     console.log('result1 hash:', entry_hash)
//     //t.equal(result, 'foo')
//
//     // await s.consistency()
//     //
//     // // Get Entry
//     // // =========
//     // //console.log('arg:', arg)
//     // const result2 = await billy.call('tester', 'snapmail', 'get_chunk', entry_hash)
//     // console.log('result2:', result2)
//     // t.equal(result2, data_string)
// })

orchestrator.run()
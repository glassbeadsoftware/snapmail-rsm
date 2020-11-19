const { config, ALEX_NICK, BILLY_MICK } = require('../config')

// -- Export scenarios -- //

module.exports = scenario => {
    scenario("write/get chunk", test_writeget_chunk)
    //scenario("send chunk", test_send_chunk)
}

// -- Scenarios -- //


/**
 *
 */
const test_writeget_chunk = async (s, t) => {

    const { conductor } = await s.players({ conductor: config })
    //console.log({conductor})

    await conductor.spawn()

    const [_dnaHash, agentAddress] = conductor.cellId(ALEX_NICK)
    console.log({agentAddress})
    // const dump = await alex.stateDump(ALEX_NICK)
    // console.log(dump)

    const result = await conductor.call(ALEX_NICK, 'snapmail', 'whoami', undefined)
    console.log({result})
    console.log('agent_latest_pubkey: ', result.agent_latest_pubkey.hash)

    //console.log('agent_pubkey:', result.agent_pubkey.hash.toString())
    const chunk_size = 1 * 1024 /** 1024*/;
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
        let result = await conductor.call(ALEX_NICK, 'snapmail', 'write_chunk', chunk)
        const write_end = Date.now();
        console.log( '['+(write_end - start) +'] (' + (write_end - loop_start)  + ') result0:' + JSON.stringify(result))
        result = await conductor.call(ALEX_NICK, 'snapmail', 'get_chunk_hash', chunk)
        console.log('result1:', result)
        //let entry_hash = [...result.hash];
        //console.log('result1 hash:', entry_hash)
        //t.equal(result, 'foo')
        const result2 = await conductor.call(ALEX_NICK, 'snapmail', 'get_chunk', result)
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
};

/**
 *
const test_send_chunk = async (s, t) => {
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
};
*/
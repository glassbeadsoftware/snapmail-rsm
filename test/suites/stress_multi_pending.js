const { conductorConfig } = require('../config')
const { sleep, split_file } = require('../utils')


// -- Export scenarios -- //

module.exports = scenario => {
    scenario("test stress pending 10 agents", test_stress_pending_10_agents)
    //scenario("test stress pending 30 agents", test_stress_pending_30_agents)

    // CRASH TESTS
    //scenario("test stress pending 100 agents", test_stress_pending_100_agents)
}

const canBomb = true;
const allSendRounds = 1;
const canAllAttach = false;


// -- Scenarios -- //

const test_stress_pending_100_agents = async (s, t) => {
    await test_stress_pending_multi(s, t, 100)
}

const test_stress_pending_30_agents = async (s, t) => {
    await test_stress_pending_multi(s, t, 30)
}

const test_stress_pending_10_agents = async (s, t) => {
    await test_stress_pending_multi(s, t, 10)
}


// -- Utils -- //

async function killFirstHalf(count, allPlayers) {
    for (let i = 0; i < count / 2; i++) {
        const playaName = 'player' + i;
        const playa = allPlayers[playaName];
        await playa.kill();
    }
}

async function wakeFirstHalf(count, allPlayers, playerMap) {
    for (let i = 0; i < count / 2; i++) {
        const playaName = 'player' + i;
        const playa = allPlayers[playaName];
        await playa.spawn();
    }

    // Ping
    const playerLast = allPlayers['player' + (count - 1)];
    console.log('Pinging player0...')
    const player0Address = playerMap.get('player0');
    let hasResponsed = false;
    for (let i = 0; !hasResponsed && i < 5; i++) {
        const result4 = await playerLast.call("snapmail", "ping_agent", player0Address)
        hasResponsed = result4;
        await sleep(200)
    }
    //assert(hasResponsed)
}

/**
 *
 */
const test_stress_pending_multi = async (s, t, count) => {

    let test_start = Date.now()

    // -- Spawn players -- //

    // Generate list of player names
    let configObj = {}
    for (let i = 0; i < count; i++) {
        let name = 'player' + i
        configObj[name] = conductorConfig
    }

    // Spawn all players
    let spawn_start = Date.now()
    let allPlayers = await s.players(configObj, true)
    let spawn_end = Date.now();
    let spawn_duration = (spawn_end - spawn_start) / 1000

    // Create Map of AgentAddress -> PlayerName
    let playerMap = new Map()
    for (let playerName in allPlayers) {
        if (!Object.prototype.hasOwnProperty.call(allPlayers, playerName)) {
            continue;
        }
        const playa = allPlayers[playerName];
        //console.log({playa})
        const info = playa.instance('app')
        //console.log({info})
        playerMap.set(playerName, info.agentAddress)
    }
    console.log({playerMap})
    const player0 = allPlayers['player0'];
    const player2 = allPlayers['player' + count / 2];
    const player3 = allPlayers['player' + count / 3];
    const playerLast = allPlayers['player' + (count - 1)];
    const allAddresses =[ ...playerMap.values() ];

    // -- Set Handles -- //

    let handles_start = Date.now()

    let handlePromiseArray = new Array(count)
    for (const [playerName, agentAddress] of playerMap) {
        const playa = allPlayers[playerName];
        console.log('** set_handle(): ' + playerName)
        let handle_promise = await playa.call("snapmail", "set_handle", playerName)
        handlePromiseArray.push(handle_promise)
        //console.log('handle_address: ' + JSON.stringify(handle_address))
        //t.match(handle_address.Ok, RegExp('Qm*'))
    }
    //await s.consistency()

    // Make sure handles are set (try 10 times)
    let handle_count = 0

    for (let i = 0; handle_count != count && i < 10; i++) {
        result = await player0.call("snapmail", "get_all_handles", undefined)
        //console.log('handle_list: ' + JSON.stringify(result))
        handle_count = result.Ok.length
    }
    t.deepEqual(handle_count, count)

    // Done
    let handles_end = Date.now();
    let handles_duration = (handles_end - handles_start) / 1000


    // -- Send BOMB: one Mail to All -- //

    let bomb_start = Date.now();

    if (canBomb) {

        // -- Kill first half
        await killFirstHalf(count, allPlayers)
        //await s.consistency();
        await sleep(1000)

        // -- Send Bomb

        const send_bomb_params = {
            subject: "MsgBomb",
            payload: "BOOM BOOM BOOM",
            to: allAddresses,
            cc: [],
            bcc: [],
            manifest_address_list: []
        }

        console.log('** CALLING: send_mail() - BOMB')
        const send_result = await playerLast.call("snapmail", "send_mail", send_bomb_params)
        console.log('send_result: ' + JSON.stringify(send_result))
        // Should have no pendings
        t.deepEqual(send_result.cc_pendings, {})

        //await s.consistency()

        // -- Wake first half
        await wakeFirstHalf(count, allPlayers, playerMap)
        //await s.consistency();
        await sleep(1000)

        // -- Check reception

        let mail_count = 0
        let check_result;
        for (let i = 0; mail_count != 1 && i < 10; i++) {
            //await s.consistency()
            check_result = await player0.call("snapmail", "check_incoming_mail", undefined)
            console.log('' + i + '. check_result2: ' + JSON.stringify(check_result))
            mail_count = check_result.length
            await sleep(1000)
        }
        t.deepEqual(mail_count, 1)
        t.match(check_result[0], RegExp('Qm*'))
        const mail_adr = check_result[0]

        // const arrived_result = await player0.call("app", "snapmail", "get_all_arrived_mail", {})
        // console.log('arrived_result : ' + JSON.stringify(arrived_result))
        // t.deepEqual(arrived_result.Ok.length, 1)
        // const mail_adr = arrived_result.Ok[0]
        // t.match(mail_adr, RegExp('Qm*'))

        const mail_result = await player0.call("snapmail", "get_mail", mail_adr)
        console.log('mail_result : ' + JSON.stringify(mail_result.Ok))
        const result_obj = mail_result.Ok.mail
        console.log('result_obj : ' + JSON.stringify(result_obj))
        t.deepEqual(send_bomb_params.payload, result_obj.payload)
    }

    // Done
    let bomb_end = Date.now();
    let bomb_duration = (bomb_end - bomb_start) / 1000


    // -- All sends one message -- //

    let all_send_start = Date.now();

    for(let round = 1; round < allSendRounds + 1; round++) {

        await sleep(1000)

        console.log('\n\n *** ALL SEND ROUND: ' + round + "\n")

        // -- Kill first half
        await killFirstHalf(count, allPlayers)
        await sleep(1000)

        for (let i = count / 2; i < count; i++) {
            const recvIndex = i - count / 2;
            const recvAgent = allAddresses[recvIndex];
            const recvName = 'player' + recvIndex;
            const playerName = 'player' + i;
            const playa = allPlayers[playerName];

            const send_params = {
                subject: "" + round + ". msg from " + playerName,
                payload: "hello to " + recvName + " ; round: " + round,
                to: [recvAgent],
                cc: [],
                bcc: [],
                manifest_address_list: []
            }

            console.log('** CALLING: send_mail() - ' + playerName)
            const send_result2 = await playa.call("snapmail", "send_mail", send_params)
            //console.log('send_result: ' + JSON.stringify(send_result2))
            // Should have no pendings
            t.deepEqual(send_result2.cc_pendings, {})
        }

        //await s.consistency()

        // -- Wake first half
        await wakeFirstHalf(count, allPlayers, playerMap)
        //await s.consistency();
        await sleep(1000)

        // -- Check reception
        const player21 = allPlayers['player' + (count / 2 - 1)];

        let mail_count = 0
        let check_result;
        for (let i = 0; mail_count != round && i < 5; i++) {
            //await s.consistency()
            check_result = await player21.call("snapmail", "check_incoming_mail", undefined)
            console.log('' + i + '. check incoming: ' + JSON.stringify(check_result))
            mail_count = check_result.length
            await sleep(200)
        }
        t.deepEqual(mail_count, round)
        t.match(check_result.Ok[0], RegExp('Qm*'))
        const mail_adr2 = check_result.Ok[0]

        // const arrived_result2 = await player0.call("app", "snapmail", "get_all_arrived_mail", {})
        // console.log('arrived_result2 : ' + JSON.stringify(arrived_result2.Ok[0]))
        // t.deepEqual(arrived_result2.Ok.length, 2)
        // const mail_adr2 = arrived_result2.Ok[0]
        // t.match(mail_adr2, RegExp('Qm*'))

        const mail_result2 = await player21.call("snapmail", "get_mail", mail_adr2)
        console.log('mail_result2 : ' + JSON.stringify(mail_result2.Ok))
        const result_obj2 = mail_result2.Ok.mail
        console.log('result_obj2 : ' + JSON.stringify(result_obj2))
        t.deepEqual(result_obj2.payload, 'hello to player' + (count / 2 - 1) + ' ; round: ' + round)
    }
    // Done
    let all_send_end = Date.now();
    let all_send_duration = (all_send_end - all_send_start) / 1000


    // -- All sends one attachment -- //

    let all_attach_start = Date.now();

    if (canAllAttach) {

        // -- Kill first half
        await killFirstHalf(count, allPlayers)
        //await s.consistency();
        await sleep(1000)

        // prevAgent = allAddresses[count - 1];
        // prevName = 'player' + (count - 1)
        // for (const [playerName, agentAddress] of playerMap) {
        //     const playa = allPlayers[playerName];

        for (let i = count / 2; i < count; i++) {
            const recvIndex = i - count / 2;
            const recvAgent = allAddresses[recvIndex];
            const recvName = 'player' + recvIndex;
            const playerName = 'player' + i;
            const playa = allPlayers[playerName];

            // Create file
            const data_string = playerName.repeat(250 * 1024 / 10)
            const fileChunks = split_file(data_string)

            // Commit chunks
            let chunk_list = [];
            for (let i = 0; i < fileChunks.numChunks; ++i) {
                const chunk_params = {
                    data_hash: fileChunks.dataHash,
                    chunk_index: i,
                    chunk: fileChunks.chunks[i],
                }
                const chunk_address = await playa.call("snapmail", "write_chunk", chunk_params)
                //console.log('chunk_address' + i + ': ' + JSON.stringify(chunk_address))
                t.match(chunk_address.Ok, RegExp('Qm*'))
                chunk_list.push(chunk_address.Ok)
            }
            chunk_list = chunk_list.reverse();

            // Commit manifest
            const manifest_params = {
                data_hash: fileChunks.dataHash,
                filename: "" + playerName + ".str",
                filetype: "str",
                orig_filesize: data_string.length,
                chunks: chunk_list,
            }
            let manifest_address = await playa.call("snapmail", "write_manifest", manifest_params)
            //console.log('manifest_address: ' + JSON.stringify(manifest_address))
            t.match(manifest_address.Ok, RegExp('Qm*'))

            // -- Send Mail
            const send_params = {
                subject: "parcel from " + playerName,
                payload: "payload to " + recvName,
                to: [recvAgent],
                cc: [],
                bcc: [],
                manifest_address_list: [manifest_address.Ok]
            }

            console.log('** CALLING: send_mail() - ' + playerName)
            const send_result2 = await playa.call("snapmail", "send_mail", send_params)
            //console.log('send_result: ' + JSON.stringify(send_result2))
            // Should have no pendings
            t.deepEqual(send_result2.Ok.cc_pendings, {})
        }

        //await s.consistency()

        // -- Wake first half
        await wakeFirstHalf(count, allPlayers, playerMap)
        //await s.consistency();
        await sleep(1000)

        // // --
        // const arrived_result3 = await player2.call("app", "snapmail", "get_all_arrived_mail", {})
        // console.log('arrived_result3 : ' + JSON.stringify(arrived_result3.Ok[0]))
        // t.deepEqual(arrived_result3.Ok.length, 3)
        // const mail_adr3 = arrived_result3.Ok[0]
        // t.match(mail_adr3, RegExp('Qm*'))

        // -- Check reception
        const player21 = allPlayers['player' + (count / 2 - 1)];

        let mail_count = 0
        let check_result;
        for (let i = 0; mail_count != 1 && i < 5; i++) {
            //await s.consistency()
            check_result = await player21.call("snapmail", "check_incoming_mail", {})
            console.log('' + i + '. check incoming: ' + JSON.stringify(check_result))
            mail_count = check_result.Ok.length
            await sleep(200)
        }
        t.deepEqual(mail_count, 1)
        t.match(check_result.Ok[0], RegExp('Qm*'))
        const mail_adr3 = check_result.Ok[0]

        // --
        const mail_result3 = await player21.call("snapmail", "get_mail", mail_adr3)
        console.log('mail_result3 : ' + JSON.stringify(mail_result3.Ok))
        const mail = mail_result3.Ok.mail
        console.log('mail : ' + JSON.stringify(mail))
        t.deepEqual(mail.payload, 'payload to player' + (count / 2 - 1))
        // check for equality of the actual and expected results
        t.true(mail.attachments[0].orig_filesize > 100 * 1024)

        // -- Get Attachment
        manifest_address = mail.attachments[0].manifest_address
        // Get chunk list via manifest
        const get_manifest_params = {manifest_address}
        const resultGet = await player21.call("snapmail", "get_manifest", get_manifest_params)
        console.log('get_manifest_result: ' + JSON.stringify(resultGet))
        t.deepEqual(resultGet.Ok.orig_filesize, mail.attachments[0].orig_filesize)
    }

    // Done
    let all_attach_end = Date.now();
    let all_attach_duration = (all_attach_end - all_attach_start) / 1000


    // -- Stats -- //

    let test_end = Date.now();
    let test_duration = (test_end - test_start) / 1000

    console.log("\n\n");
    console.log("== Stress multi pending ============ " + count);
    console.log("====================================");
    console.log("Spawn duration      : " + spawn_duration + ' sec')
    console.log("Handles duration    : " + handles_duration + ' sec')
    console.log("Bomb duration       : " + bomb_duration + ' sec')
    console.log("All send duration   : " + all_send_duration + ' sec ; rounds: ' + allSendRounds)
    console.log("All attach duration : " + all_attach_duration + ' sec')
    console.log("------------------------------------");
    console.log("Test duration       : " + test_duration + ' sec')
    console.log("==================================== " + count);
    console.log("\n");
}

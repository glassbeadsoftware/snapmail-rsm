// const { conductorConfig } = require('../config')
// const { sleep, split_file } = require('../utils')

import {monoAgentInstall, setup_3_conductors, setup_conductor_3p, expConfig, quicConductorConfig} from "../config";

const { sleep, filterMailList, delay, logDump, htos, cellIdToStr, split_file } = require('../utils')


// -- Export scenarios -- //

module.exports = scenario => {
    scenario("test stress 2 agents", test_stress_2_agents)
    //scenario("test stress 10 agents", test_stress_10_agents)
    //scenario("test stress 30 agents", test_stress_30_agents)

    // CRASH TESTS
    //scenario("test stress 100 agents", test_stress_100_agents)
}

const canBomb = true;
const canAllSend = false;
const canAllAttach = false;


// -- Scenarios -- //

const test_stress_100_agents = async (s, t) => {
    await test_stress_multi(s, t, 100)
}

const test_stress_30_agents = async (s, t) => {
    await test_stress_multi(s, t, 30)
}

const test_stress_10_agents = async (s, t) => {
    await test_stress_multi(s, t, 10)
}

const test_stress_2_agents = async (s, t) => {
    await test_stress_multi(s, t, 2)
}

/**
 *
 */
const test_stress_multi = async (s, t, count) => {

    let test_start = Date.now()

    // -- Spawn players -- //

    // Generate list of player names
    let configArray = new Array()
    for (let i = 0; i < count; i++) {
        //let name = 'player' + i
        configArray.push(quicConductorConfig)
    }

    // Spawn all players
    let spawn_start = Date.now()
    let allConductors = await s.players(configArray, true)
    let spawn_end = Date.now();
    let spawn_duration = (spawn_end - spawn_start) / 1000

    const r = await s.shareAllNodes(allConductors)
    await delay(4000) // allow 2 second for gossiping

    // Install Happ for each player
    let happArray = new Array()
    for (let i = 0; i < count; i++) {
        const [[happ]] = await allConductors[i].installAgentsHapps(monoAgentInstall);
        happArray.push(happ)
    }

    // Create Map of AgentAddress -> PlayerName
    let playerMap = new Map()
    for (let i = 0; i < count; i++) {
        let name = 'player' + i
        // if (!Object.prototype.hasOwnProperty.call(allPlayers, playerName)) {
        //     continue;
        // }
        playerMap.set(name, happArray[i].agent)
    }
    //console.log({playerMap})
    const playerCell0 = happArray[0].cells[0];
    const playerCell2 = happArray[count / 2].cells[0];
    //const playerCell3 = allConductors['player' + count / 3];
    const allAddresses = [ ...playerMap.values() ]
    console.log({allAddresses})

    // -- Set Handles for each Player -- //

    let handles_start = Date.now()

    //let handlePromiseArray = new Array(count)
    //for (const [playerName, agentAddress] of playerMap) {
    for (let i = 0; i < count; i++) {
        let name = 'player' + i
        const playerCell = happArray[i].cells[0];
        console.log('** set_handle(): ' + name)
        let handle_hh = await playerCell.call("snapmail", "set_handle", name)
        //handlePromiseArray.push(handle_promise)
        console.log('handle_hh: ' + htos(handle_hh))
        //t.match(handle_address.Ok, RegExp('Qm*'))
    }
    await delay(100)

    // Make sure handles are set (try 10 times)
    let handleCount = 0
    for (let i = 0; handleCount != count && i < 10; i++) {
        let allHandles = await playerCell0.call("snapmail", "get_all_handles", undefined)
        console.log('allHandles: ' + JSON.stringify(allHandles))
        handleCount = allHandles.length
    }
    t.deepEqual(handleCount, count)

    // Done
    let handles_end = Date.now();
    let handles_duration = (handles_end - handles_start) / 1000


    // -- Send BOMB: one Mail to All -- //

    let bomb_start = Date.now();

    if (canBomb) {
        const send_bomb_params = {
            subject: "MsgBomb",
            payload: "BOOM BOOM BOOM",
            to: allAddresses,
            cc: [],
            bcc: [],
            //manifest_address_list: []
        }

        console.log('** CALLING: send_mail() - BOMB')
        const send_result = await playerCell0.call("snapmail", "send_mail", send_bomb_params)
        console.log('send_result: ' + JSON.stringify(send_result))
        // Should have no pendings
        t.deepEqual(send_result.Ok.cc_pendings, {})

        await delay(100)

        const arrived_result = await playerCell2.call("snapmail", "get_all_arrived_mail", undefined)
        console.log('arrived_result : ' + JSON.stringify(arrived_result[0]))
        t.deepEqual(arrived_result.length, 1)
        const mail_adr = arrived_result[0]
        //t.match(mail_adr, RegExp('Qm*'))

        const mail_result = await playerCell2.call("app", "snapmail", "get_mail", mail_adr)
        //console.log('mail_result : ' + JSON.stringify(mail_result.Ok))
        const result_obj = mail_result.Ok.mail
        //console.log('result_obj : ' + JSON.stringify(result_obj))
        t.deepEqual(send_bomb_params.payload, result_obj.payload)
    }
    // Done
    let bomb_end = Date.now();
    let bomb_duration = (bomb_end - bomb_start) / 1000


    // -- All sends one message -- //

    let all_send_start = Date.now();

    if (canAllSend) {
        let prevAgent = allAddresses[count - 1];
        let prevName = 'player' + (count - 1)
        for (let i = 0; i < count; i++) {
            let playerName = 'player' + i
            const agentAddress = playerMap[playerName]
            const playa = happArray[i].cells[0];
            const send_params = {
                subject: "msg from " + playerName,
                payload: "hello to " + prevName,
                to: [prevAgent],
                cc: [],
                bcc: [],
                //manifest_address_list: []
            }

            console.log('** CALLING: send_mail() - ' + playerName)
            const send_result2 = await playa.call("snapmail", "send_mail", send_params)
            console.log('send_result: ' + JSON.stringify(send_result2))
            // Should have no pendings
            t.deepEqual(send_result2.cc_pendings, {})
            prevAgent = agentAddress
            prevName = playerName
        }

        await delay(100)

        const arrived_result2 = await playerCell2.call("snapmail", "get_all_arrived_mail", undefined)
        console.log('arrived_result2 : ' + JSON.stringify(arrived_result2[0]))
        t.deepEqual(arrived_result2.length, 2)
        const mail_adr2 = arrived_result2[0]
        //t.match(mail_adr2, RegExp('Qm*'))

        const mail_result2 = await playerCell2.call("snapmail", "get_mail", mail_adr2)
        console.log('mail_result2 : ' + JSON.stringify(mail_result2.Ok))
        const result_obj2 = mail_result2.Ok.mail
        console.log('result_obj2 : ' + JSON.stringify(result_obj2))
        t.deepEqual(result_obj2.payload, 'hello to player' + (count / 2))
    }
    // Done
    let all_send_end = Date.now();
    let all_send_duration = (all_send_end - all_send_start) / 1000


    // -- All sends one attachment -- //

    let all_attach_start = Date.now();

    if (canAllAttach) {
        let prevAgent = allAddresses[count - 1];
        let prevName = 'player' + (count - 1)
        for (let i = 0; i < count; i++) {
            let playerName = 'player' + i
            const agentAddress = playerMap[playerName]
            const playa = happArray[i].cells[0];

            // Create file
            const data_string = playerName.repeat(250 * 1024 / 10)
            const fileChunks = split_file(data_string)

            // Commit chunks
            let chunk_list = new Array();
            for (let i = 0; i < fileChunks.numChunks; ++i) {
                const chunk_params = {
                    data_hash: fileChunks.dataHash,
                    chunk_index: i,
                    chunk: fileChunks.chunks[i],
                }
                const chunk_address = await playa.call("snapmail", "write_chunk", chunk_params)
                //console.log('chunk_address' + i + ': ' + JSON.stringify(chunk_address))
                //t.match(chunk_address.Ok, RegExp('Qm*'))
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
            //t.match(manifest_address.Ok, RegExp('Qm*'))

            // -- Send Mail
            const send_params = {
                subject: "parcel from " + playerName,
                payload: "payload to " + prevName,
                to: [prevAgent],
                cc: [],
                bcc: [],
                manifest_address_list: [manifest_address.Ok]
            }

            console.log('** CALLING: send_mail() - ' + playerName)
            const send_result2 = await playa.call("snapmail", "send_mail", send_params)
            //console.log('send_result: ' + JSON.stringify(send_result2))
            // Should have no pendings
            t.deepEqual(send_result2.Ok.cc_pendings, {})
            prevAgent = agentAddress
            prevName = playerName
        }

        await delay(100)

        const arrived_result3 = await playerCell2.call("snapmail", "get_all_arrived_mail", undefined)
        console.log('arrived_result3 : ' + JSON.stringify(arrived_result3.Ok[0]))
        t.deepEqual(arrived_result3.length, 3)
        const mail_adr3 = arrived_result3[0]
        //t.match(mail_adr3, RegExp('Qm*'))

        const mail_result3 = await playerCell2.call("snapmail", "get_mail", mail_adr3)
        console.log('mail_result3 : ' + JSON.stringify(mail_result3.Ok))
        const mail = mail_result3.Ok.mail
        console.log('mail : ' + JSON.stringify(mail))
        t.deepEqual(mail.payload, 'payload to player' + (count / 2))
        // check for equality of the actual and expected results
        t.true(mail.attachments[0].orig_filesize > 200 * 1024)

        // -- Get Attachment
        let manifestAddress = mail.attachments[0].manifest_address
        // Get chunk list via manifest
        const get_manifest_params = {manifestAddress}
        const resultGet = await playerCell2.call("snapmail", "get_manifest", get_manifest_params)
        console.log('get_manifest_result: ' + JSON.stringify(resultGet))
        t.deepEqual(resultGet.orig_filesize, mail.attachments[0].orig_filesize)
    }

    // Done
    let all_attach_end = Date.now();
    let all_attach_duration = (all_attach_end - all_attach_start) / 1000


    // -- Stats -- //

    let test_end = Date.now();
    let test_duration = (test_end - test_start) / 1000

    console.log("\n\n");
    console.log("== Stress multi ============ " + count);
    console.log("==================================== " + count);
    console.log("Spawn duration      : " + spawn_duration + ' sec')
    console.log("Handles duration    : " + handles_duration + ' sec')
    console.log("Bomb duration       : " + bomb_duration + ' sec')
    console.log("All send duration   : " + all_send_duration + ' sec')
    console.log("All attach duration : " + all_attach_duration + ' sec')
    console.log("------------------------------------");
    console.log("Test duration       : " + test_duration + ' sec')
    console.log("====================================" + count);
    console.log("\n");
}

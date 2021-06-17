import {setup_3_conductors} from "../config";

const { conductorConfig } = require('../config')
const { sleep, split_file } = require('../utils')


// -- Export scenarios -- //

module.exports = scenario => {
    scenario("test send file async tiny", test_send_file_async_tiny)
    //scenario("test send file async big", test_send_file_async_big)

    // LONG TESTS
    //process.env['TRYORAMA_ZOME_CALL_TIMEOUT_MS'] = '90000'
    //scenario("test send file async three", test_send_file_async_three)
}

// -- Scenarios -- //

const test_send_file_async_tiny = async (s, t) => {
    await test_send_file_async(s, t, 1 * 1024)
}

const test_send_file_async_big = async (s, t) => {
    await test_send_file_async(s, t, 1 * 1024 * 1024)
}

const test_send_file_async = async (s, t, size) => {

    // - Create fake file
    const data_string = "0123465789".repeat(size / 10)
    //const data_string = "0123465789"

    let { alex, billy, camille, alexHapp, billyHapp, camilleHapp, alexCell, billyCell, camilleCell } = await setup_3_conductors(s, t)
    //const {alex, billy} = await s.players({alex: conductorConfig, billy: conductorConfig}, true)

    //await sleep(1000)

    console.log('billyId: ' + billyHapp.agent)

    // Make sure Billy has a handle entry
    let name = "billy"
    let handle_address = await billyCell.call("snapmail", "set_handle", name)
    console.log('handle_address1: ' + JSON.stringify(handle_address))
    //t.match(handle_address.Ok, RegExp('Qm*'))
    // Wait for all network activity to settle
    //await s.consistency()

    // Make sure Alex has a handle entry
    name = "alex"
    handle_address = await alexCell.call("snapmail", "set_handle", name)
    console.log('handle_address2: ' + JSON.stringify(handle_address))
    //t.match(handle_address.Ok, RegExp('Qm*'))

    //await sleep(1000)

    // -- Make sure handles are set -- //

    let handle_count = 0
    let result;
    for (let i = 0; handle_count != 2 && i < 10; i++) {
        await sleep(1000)
        result = await billyCell.call("snapmail", "get_all_handles", undefined)
        console.log('handle_listB: ' + JSON.stringify(result))
        handle_count = result.length
    }
    t.deepEqual(handle_count, 2)
    handle_count = 0
    for (let i = 0; handle_count != 2 && i < 10; i++) {
        await sleep(1000)
        result = await alexCell.call("snapmail", "get_all_handles", undefined)
        console.log('handle_listA: ' + JSON.stringify(result))
        handle_count = result.length
    }
    t.deepEqual(handle_count, 2)

    // split file
    const fileChunks = split_file(data_string)
    // Write chunks
    var chunk_list = new Array();
    for (var i = 0; i < fileChunks.numChunks; ++i) {
        const chunk_params = {
            data_hash: fileChunks.dataHash,
            chunk_index: i,
            chunk: fileChunks.chunks[i],
        }
        const chunk_address = await alexCell.call("snapmail", "write_chunk", chunk_params)
        //console.log('chunk_address' + i + ': ' + JSON.stringify(chunk_address))
        //t.match(chunk_address.Ok, RegExp('Qm*'))
        chunk_list.push(chunk_address)
    }
    chunk_list = chunk_list.reverse();

    // Write manifest
    const manifest_params = {
        data_hash: fileChunks.dataHash,
        filename: "fake.str",
        filetype: "str",
        orig_filesize: data_string.length,
        chunks: chunk_list,
    }
    console.log('manifest_params: ' + JSON.stringify(manifest_params))

    let manifest_address = await alexCell.call("snapmail", "write_manifest", manifest_params)
    console.log('manifest_address: ' + JSON.stringify(manifest_address))
    //t.match(manifest_address.Ok, RegExp('Qm*'))

    // -- Billy goes offline

    await billy.shutdown();
    await sleep(1000)

    // -- Send Mail to Billy
    const send_params = {
        subject: "test-attachment",
        payload: "blablabla",
        to: [billyHapp.agent],
        cc: [],
        bcc: [],
        manifest_address_list: [manifest_address],
    }

    const send_result = await alexCell.call("snapmail", "send_mail", send_params)
    console.log('send_result: ' + JSON.stringify(send_result))
    // Should receive via DM, so no pendings
    t.deepEqual(send_result.cc_pendings, {})

    // Wait for all network activity to settle
    //await sleep(1000)

    // -- Billy goes Online
    await billy.startup();
    await sleep(1000) // allow 1 second for gossiping

    // -- Ping -- //
    const result4 = await billyCell.call("snapmail", "ping_agent", alexHapp.agent)
    t.deepEqual(result4, true)

    let mail_count = 0
    let check_result;
    for (let i = 0; mail_count != 1 && i < 3; i++) {
        await sleep(1000) // allow 1 second for gossiping
        check_result = await billyCell.call("snapmail", "check_incoming_mail", undefined)
        console.log('' + i + '. check_result2: ' + JSON.stringify(check_result))
        mail_count = check_result.length
    }
    t.deepEqual(mail_count, 1)
    //t.match(check_result.Ok[0], RegExp('Qm*'))
    const mail_adr = check_result[0]

    // -- Get Mail
    const mail_result = await billyCell.call("snapmail", "get_mail", mail_adr)
    console.log('mail_result: ' + JSON.stringify(mail_result))
    const mail = mail_result.Ok.mail
    // check for equality of the actual and expected results
    t.deepEqual(send_params.payload, mail.payload)
    t.deepEqual(data_string.length, mail.attachments[0].orig_filesize)

    // -- Get Attachment
    manifest_address = mail.attachments[0].manifest_eh;

    // Get chunk list via manifest
    const resultGet = await billyCell.call("snapmail", "get_manifest", manifest_address)
    console.log('get_manifest_result: ' + JSON.stringify(resultGet))
    t.deepEqual(resultGet.orig_filesize, data_string.length)
    chunk_list = resultGet.chunks;

    // Get chunks
    let result_string = ''
    for (var i = chunk_list.length - 1; i >= 0; --i) {
        // await s.consistency()
        // sleep(10000)
        const params2 = chunk_list[i]
        const result = await billyCell.call("snapmail", "get_chunk", params2)
        console.log('get_result' + i + ': ' + JSON.stringify(result))
        result_string += result
    }
    console.log('result_string.length: ' + result_string.length)
    t.deepEqual(data_string.length, result_string.length)
    t.deepEqual(data_string, result_string)
};


/**
 *
 */
const test_send_file_async_three = async (s, t) => {
    // - Create fake file
    //const data_string = "0123465789".repeat(500 * 1024 / 10)
    const data_string = "0123465789"

    let { alex, billy, camille, alexHapp, billyHapp, camilleHapp, alexCell, billyCell, camilleCell } = await setup_3_conductors(s, t)
    // const {alex, billy, camille} = await s.players({alex: conductorConfig, billy: conductorConfig, camille: conductorConfig}, true)

    console.log('alexId: ' + alexHapp.agent)
    console.log('billyId: ' + billyHapp.agent)
    console.log('camilleId: ' + camilleHapp.agent)

    // -- Set Handles for all -- //

    let name = "billy"
    let handle_address = await billyCell.call("snapmail", "set_handle", name)
    console.log('handle_address1: ' + JSON.stringify(handle_address))

    name = "alex"
    handle_address = await alexCell.call("snapmail", "set_handle", name)
    console.log('handle_address2: ' + JSON.stringify(handle_address))

    name = "camille"
    handle_address = await camilleCell.call("snapmail", "set_handle", name)
    console.log('handle_address3: ' + JSON.stringify(handle_address))

    // -- Make sure handles are set -- //

    let handle_count = 0
    let result;
    for (let i = 0; handle_count != 3 && i < 10; i++) {
        await sleep(1000)
        result = await billyCell.call("snapmail", "get_all_handles", undefined)
        console.log('handle_listB: ' + JSON.stringify(result))
        handle_count = result.length
    }
    t.deepEqual(handle_count, 3)

    handle_count = 0
    for (let i = 0; handle_count != 3 && i < 10; i++) {
        await sleep(1000)
        result = await alexCell.call("snapmail", "get_all_handles", undefined)
        console.log('handle_listA: ' + JSON.stringify(result))
        handle_count = result.length
    }
    t.deepEqual(handle_count, 3)

    handle_count = 0
    for (let i = 0; handle_count != 3 && i < 10; i++) {
        await sleep(1000)
        result = await camilleCell.call("snapmail", "get_all_handles", undefined)
        console.log('handle_listC: ' + JSON.stringify(result))
        handle_count = result.length
    }
    t.deepEqual(handle_count, 3)

    // split file
    const fileChunks = split_file(data_string)
    // Write chunks
    var chunk_list = new Array();
    for (var i = 0; i < fileChunks.numChunks; ++i) {
        const chunk_params = {
            data_hash: fileChunks.dataHash,
            chunk_index: i,
            chunk: fileChunks.chunks[i],
        }
        const chunk_address = await alexCell.call("snapmail", "write_chunk", chunk_params)
        //console.log('chunk_address' + i + ': ' + JSON.stringify(chunk_address))
        //t.match(chunk_address.Ok, RegExp('Qm*'))
        chunk_list.push(chunk_address)
    }
    chunk_list = chunk_list.reverse();

    // Write manifest
    const manifest_params = {
        data_hash: fileChunks.dataHash,
        filename: "fake.str",
        filetype: "str",
        orig_filesize: data_string.length,
        chunks: chunk_list,
    }
    let manifest_address = await alexCell.call("snapmail", "write_manifest", manifest_params)
    console.log('manifest_eh: ' + JSON.stringify(manifest_address))
    //t.match(manifest_address.Ok, RegExp('Qm*'))

    // -- Send Mail to Billy offline
    await billy.shutdown();


    const send_params = {
        subject: "test-attachment",
        payload: "blablabla",
        to: [billyHapp.agent],
        cc: [],
        bcc: [],
        manifest_address_list: [manifest_address],
    }

    const send_result = await alexCell.call("snapmail", "send_mail", send_params)
    console.log('send_result: ' + JSON.stringify(send_result))
    // Should receive via DM, so no pendings
    t.deepEqual(send_result.cc_pendings, {})

    await sleep(2000) // allow time for gossiping

    // Kill Alex :(
    await alex.shutdown();

    await sleep(2000) // allow time for gossiping

    // Spawn back billy
    await billy.startup();

    let mail_count = 0
    let check_result;
    for (let i = 0; mail_count != 1 && i < 10; i++) {
        await sleep(1000) // allow 1 second for gossiping
        check_result = await billyCell.call("snapmail", "check_incoming_mail", undefined)
        console.log('' + i + '. check_result2: ' + JSON.stringify(check_result))
        mail_count = check_result.length
    }
    t.deepEqual(mail_count, 1)
    //t.match(check_result.Ok[0], RegExp('Qm*'))
    const mail_adr = check_result[0]


    // -- Get Mail
    const mail_result = await billyCell.call("snapmail", "get_mail", mail_adr)
    console.log('mail_result: ' + JSON.stringify(mail_result.Ok))
    const mail = mail_result.Ok.mail
    // check for equality of the actual and expected results
    t.deepEqual(send_params.payload, mail.payload)
    t.deepEqual(data_string.length, mail.attachments[0].orig_filesize)

    // -- Get Attachment
    manifest_address = mail.attachments[0].manifest_address;

    // Get chunk list via manifest
    let resultGet = await billyCell.call("snapmail", "get_manifest", manifest_address)
    console.log('get_manifest_result: ' + JSON.stringify(resultGet))
    t.deepEqual(resultGet.Err.Internal, "No entry found at given address")

    // Get missing attachment
    const get_missing_attachment = {
        from: mail_result.Ok.from,
        inmail_address: mail_adr
    }
    let result_missing = await billyCell.call("snapmail", "get_missing_attachments", get_missing_attachment)
    console.log('result_missing1: ' + JSON.stringify(result_missing))
    t.deepEqual(result_missing, 1)

    // Spawn back Alex
    await alex.startup();
    await sleep(1000) // allow 1 second for gossiping

    // Ping
    const result4 = await billyCell.call("snapmail", "ping_agent", alexHapp.agent)
    t.deepEqual(result4, true)

    // Get missing attachment
    result_missing = await billyCell.call("snapmail", "get_missing_attachments", get_missing_attachment)
    console.log('result_missing2: ' + JSON.stringify(result_missing))
    t.deepEqual(result_missing.Ok, 0)

    // Get chunk list via manifest
    resultGet = await billyCell.call("snapmail", "get_manifest", manifest_address)
    console.log('get_manifest_result: ' + JSON.stringify(resultGet))
    t.deepEqual(resultGet.orig_filesize, data_string.length)
    chunk_list = resultGet.chunks;

    // Get chunks
    let result_string = ''
    for (var i = chunk_list.length - 1; i >= 0; --i) {
        // await s.consistency()
        const params2 = {chunk_address: chunk_list[i]}
        const result = await billyCell.call("snapmail", "get_chunk", params2)
        // console.log('get_result' + i + ': ' + JSON.stringify(result))
        result_string += result
    }
    console.log('result_string.length: ' + result_string.length)
    t.deepEqual(data_string.length, result_string.length)
    t.deepEqual(data_string, result_string)
};

const { setup_conductor, setup_alex_only, ALEX_NICK, BILLY_NICK, CAMILLE_NICK } = require('../config')

const { sleep, filterMailList, delay } = require('../utils')

// -- Export scenarios -- //

module.exports = scenario => {
    //scenario("send pending test", send_pending_test)
    //scenario("send via DM test", send_dm_test)
    scenario("get all mails test", test_get_all_mails)

    /// DEBUG
    //scenario("outack test", debug_test)
}

// -- Scenarios -- //

/**
 *
 */
async function setup_handles(s, t, conductor) {
    // Make sure Billy has a handle entry
    let name = "billy"
    let handle_address = await conductor.call(BILLY_NICK, "snapmail", "set_handle", name)
    console.log('handle_address1: ' + JSON.stringify(handle_address))
    //t.match(handle_address.Ok, RegExp('Qm*'))

    await delay(10);

    // Make sure Alex has a handle entry
    name = "alex"
    handle_address = await conductor.call(ALEX_NICK, "snapmail", "set_handle", name)
    console.log('handle_address2: ' + JSON.stringify(handle_address))
    //t.match(handle_address.Ok, RegExp('Qm*'))

    await delay(10);

    // -- Make sure handles are set -- //

    let handle_count = 0
    for (let i = 0; handle_count != 2 && i < 10; i++) {
        const result = await conductor.call(BILLY_NICK, "snapmail", "get_all_handles", undefined)
        console.log('handle_list: ' + JSON.stringify(result))
        handle_count = result.length
    }
    t.deepEqual(handle_count, 2)

    console.log('\n**** HANDLES HAVE BEEN SET **** \n\n')
}


/**
 * Send mail and acknowledgement while other party is offline
 */
const send_pending_test = async (s, t) => {
    // -- Setup -- //
    const { conductor, alexAddress, billyAddress } = await setup_conductor(s, t)

    await setup_handles(s, t, conductor)

    // -- Billy goes offline -- //

    await conductor.kill(BILLY_NICK)
    await delay(1000);

    // -- Alex sends mail to Billy -- //

    const send_params = {
        subject: "test-outmail",
        payload: "blablabla",
        to: [billyAddress],
        cc: [],
        bcc: [],
        //manifest_address_list: []
    }

    console.log('** CALLING: send_mail()')
    const send_result = await conductor.call(ALEX_NICK, "snapmail", "send_mail", send_params)
    console.log('send_result: ' + JSON.stringify(send_result))
    // Should have no pendings
    t.deepEqual(send_result.Ok.cc_pendings, {})

    // -- Billy goes online -- //

    await conductor.spawn(BILLY_NICK)

    // handle_address = await billy.call("app", "snapmail", "set_handle", params)
    // console.log('handle_address2: ' + JSON.stringify(handle_address))
    // t.match(handle_address.Ok, RegExp('Qm*'))

    await delay(10);

    // -- Billy checks inbox -- //

    const check_result = await conductor.call(BILLY_NICK, "snapmail", "check_incoming_mail", {})
    console.log('check_result2      : ' + JSON.stringify(check_result))
    t.deepEqual(check_result.Ok.length, 1)
    t.match(check_result.Ok[0], RegExp('Qm*'))

    const arrived_result = await conductor.call(BILLY_NICK, "snapmail", "get_all_arrived_mail", {})
    console.log('arrived_result : ' + JSON.stringify(arrived_result.Ok[0]))
    t.deepEqual(arrived_result.Ok.length, 1)
    const mail_adr = arrived_result.Ok[0]
    t.match(mail_adr, RegExp('Qm*'))

    const mail_result = await conductor.call(BILLY_NICK, "snapmail", "get_mail", mail_adr)
    console.log('mail_result : ' + JSON.stringify(mail_result.Ok))
    const result_obj = mail_result.Ok.mail
    console.log('result_obj : ' + JSON.stringify(result_obj))

    // check for equality of the actual and expected results
    t.deepEqual(send_params.payload, result_obj.payload)

    // -- Alex should see that mail has been received -- //

    // Make sure Alex has a handle entry
    // name = "alex"
    // const params2 = { name }
    // let handle_address2 = await alex.call("app", "snapmail", "set_handle", params2)
    // console.log('handle_address3: ' + JSON.stringify(handle_address2))
    // t.match(handle_address2.Ok, RegExp('Qm*'))

    await delay(10);

    const received_result = await conductor.call(ALEX_NICK, "snapmail", "has_mail_been_received", {"outmail_address": send_result.Ok.outmail})
    console.log('received_result1 : ' + JSON.stringify(received_result.Ok))
    t.deepEqual(received_result.Ok.Err.length, 1)
    t.deepEqual(received_result.Ok.Err[0], billyAddress)

    // -- Alex goes offline -- //

    await conductor.kill(ALEX_NICK)
    //await s.consistency()
    await delay(2000);

    // -- Billy sends Acknowledgment -- //

    const ack_result = await conductor.call(BILLY_NICK, "snapmail", "acknowledge_mail", {"inmail_address": mail_adr})
    console.log('ack_result1 : ' + ack_result.Ok)
    const ack_adr = ack_result.Ok

    // -- Alex goes online -- //

    await conductor.spawn(ALEX_NICK)
    await s.consistency()
    await delay(2000);

    // -- Alex checks for acknowledgement -- //

    const check_result2 = await conductor.call(ALEX_NICK, "snapmail", "check_incoming_ack", {})
    console.log('check_result2      : ' + JSON.stringify(check_result2))
    t.deepEqual(check_result2.Ok.length, 1)
    t.match(check_result2.Ok[0], RegExp('Qm*'))

    const received_result2 = await conductor.call(ALEX_NICK, "snapmail", "has_mail_been_received", {"outmail_address": send_result.Ok.outmail})
    console.log('received_result2 : ' + JSON.stringify(received_result2.Ok))
    t.deepEqual(received_result2.Ok.Ok, null)

    // -- Billy checks if acknowledgement has been received -- //

    // TODO: Fails because Tryorama's alex.spawn() breaks something
    // const ack_result2 = await billy.call("app", "snapmail", "has_ack_been_received", {"inmail_address": mail_adr})
    // console.log('ack_result2 : ' + JSON.stringify(ack_result2))
    // t.deepEqual(ack_result2.Ok, true)
};


/**
 *
 */
const debug_test = async (s, t) => {
    const { conductor } = await setup_alex_only(s, t)

    console.log('sending...')
    //const create_result = await conductor.call(ALEX_NICK, "snapmail", "create_outack", undefined)

    // Validation should fail
    const create_result = await conductor.call(ALEX_NICK, "snapmail", "create_empty_handle", undefined)

    console.log('create_result: ' + JSON.stringify(create_result))
}


/**
 *
 */
const send_dm_test = async (s, t) => {

    const { conductor, alexAddress, billyAddress } = await setup_conductor(s, t)

    await setup_handles(s, t, conductor)

    // Make a call to a Zome function
    // Indicating the function, and passing it an input
    const send_params = {
        subject: "test-outmail",
        payload: "blablabla",
        to: [alexAddress],
        cc: [],
        bcc: [],
        //manifest_address_list: []
    }
    console.log('sending...')
    const send_result = await conductor.call(BILLY_NICK, "snapmail", "send_mail", send_params)
    console.log('send_result: ' + JSON.stringify(send_result))
    // Should receive via DM, so no pendings
    t.deepEqual(send_result.to_pendings, {})

    // Wait for all network activity to settle
    await delay(10);

    const arrived_result = await conductor.call(ALEX_NICK, "snapmail", "get_all_arrived_mail", undefined)

    console.log('arrived_result : ' + JSON.stringify(arrived_result))
    t.deepEqual(arrived_result.length, 1)
    const mail_adr = arrived_result[0]

    const get_mail_result = await conductor.call(ALEX_NICK, "snapmail", "get_mail", mail_adr)
    console.log('mail_result : ' + JSON.stringify(get_mail_result))
    const mail = get_mail_result.Ok.mail

    // check for equality of the actual and expected results
    t.deepEqual(send_params.payload, mail.payload)

    // -- ACK -- //

    //await delay(1000);

    const received_result = await conductor.call(BILLY_NICK, "snapmail", "has_mail_been_received", send_result.outmail)
    console.log('received_result1 : ' + JSON.stringify(received_result))
    t.deepEqual(received_result.Err.length, 1)
    t.deepEqual(received_result.Err[0], alexAddress)

    const ack_result = await conductor.call(ALEX_NICK, "snapmail", "acknowledge_mail", mail_adr)
    console.log('ack_result1 : ' + JSON.stringify(ack_result))

    // await delay(10);
    //
    // const received_result2 = await conductor.call(BILLY_NICK, "snapmail", "has_mail_been_received", send_result.outmail)
    // console.log('received_result2 : ' + JSON.stringify(received_result2))
    // t.deepEqual(received_result2, null)
    //
    // const ack_result2 = await conductor.call(ALEX_NICK, "snapmail", "has_ack_been_received", mail_adr)
    // console.log('ack_result2 : ' + JSON.stringify(ack_result2))
    // t.deepEqual(ack_result2, true)
};


/**
 *
 */
const test_get_all_mails = async (s, t) => {
    // -- SETUP
    const { conductor, alexAddress, billyAddress } = await setup_conductor(s, t)

    await setup_handles(s, t, conductor)

    console.log('test_get_all_mails START')

    // Send mail DM
    let send_params = {
        subject: "inmail 1",
        payload: "aaaaaaaa",
        to: [alexAddress],
        cc: [],
        bcc: [],
        //manifest_address_list: []
    }
    const inMail1Payload = send_params.payload;

    let send_result = await conductor.call(BILLY_NICK, "snapmail", "send_mail", send_params)
    console.log('send_result1: ' + JSON.stringify(send_result))
    t.deepEqual(send_result.to_pendings, {})
    await delay(10);

    // Send mail DM
    send_params = {
        subject: "inmail 2",
        payload: "bbbb",
        to: [alexAddress],
        cc: [],
        bcc: [],
        //manifest_address_list: []
    }
    send_result = await conductor.call(BILLY_NICK, "snapmail", "send_mail", send_params)
    console.log('send_result2: ' + JSON.stringify(send_result))
    t.deepEqual(send_result.to_pendings, {})
    const inMail2 = send_result.outmail;
    await delay(10);

    // Send mail DM
    send_params = {
        subject: "outmail 3",
        payload: "ccccccc",
        to: [billyAddress],
        cc: [],
        bcc: [],
        //manifest_address_list: []
    }
    send_result = await conductor.call(ALEX_NICK, "snapmail", "send_mail", send_params)
    console.log('send_result3: ' + JSON.stringify(send_result))
    t.deepEqual(send_result.to_pendings, {})
    await delay(10);

    // Get all mails
    let mail_list_result = await conductor.call(ALEX_NICK, "snapmail", "get_all_mails", undefined)
    console.log('mail_list_result1 : ' + JSON.stringify(mail_list_result))
    t.deepEqual(mail_list_result.length, 3)
    t.deepEqual(mail_list_result[0].mail.payload, send_params.payload)

    mail_list_result = await conductor.call(BILLY_NICK, "snapmail", "get_all_mails", undefined)
    console.log('mail_list_result12 : ' + JSON.stringify(mail_list_result))
    t.deepEqual(mail_list_result.length, 3)
    //t.deepEqual(mail_list_result[0].mail.payload, send_params.payload)
    const outMail3 = mail_list_result[0].address;
    console.log('outMail3 : ' + JSON.stringify(outMail3))

    // -- delete outmail --//

    send_result = await conductor.call(BILLY_NICK, "snapmail", "delete_mail", inMail2)
    console.log('send_result4: ' + JSON.stringify(send_result))
    //t.match(send_result, RegExp('Qm*'))
    await delay(10);

    // Get mail should fail
    let mail_result = await conductor.call(BILLY_NICK, "snapmail", "get_mail", inMail2)
    console.log('mail_result1 : ' + JSON.stringify(mail_result))
    t.deepEqual(mail_result, null)

    // Get all mails
    mail_list_result = await conductor.call(BILLY_NICK, "snapmail", "get_all_mails", undefined)
    console.log('mail_list_result2 : ' + JSON.stringify(mail_list_result))
    let live_mail_list = filterMailList(mail_list_result);
    t.deepEqual(live_mail_list.length, 2)
    //t.deepEqual(live_mail_list[0].mail.payload, send_params.payload)

    // delete same mail twice should fail
    send_result = await conductor.call(BILLY_NICK, "snapmail", "delete_mail", inMail2)
    console.log('send_result5: ' + JSON.stringify(send_result))
    // FIXME
    //t.deepEqual(send_result.Err, {Internal: "Entry Could Not Be Found"})

    // Get all mails - Alex should still see 3
    mail_list_result = await conductor.call(ALEX_NICK, "snapmail", "get_all_mails", undefined)
    console.log('mail_list_result3 : ' + JSON.stringify(mail_list_result))
    live_mail_list = filterMailList(mail_list_result);
    t.deepEqual(live_mail_list.length, 3)
    //t.deepEqual(live_mail_list[0].mail.payload, send_params.payload)

    // -- delete inmail --//

    send_result = await conductor.call(BILLY_NICK, "snapmail", "delete_mail", outMail3)
    console.log('send_result6: ' + JSON.stringify(send_result))
    //t.match(send_result, RegExp('Qm*'))
    await delay(10);

    // Get mail should fail
    mail_result = await conductor.call(BILLY_NICK, "snapmail", "get_mail", outMail3)
    console.log('mail_result2 : ' + JSON.stringify(mail_result))
    t.deepEqual(mail_result, null)

    // Get all mails
    mail_list_result = await conductor.call(BILLY_NICK, "snapmail", "get_all_mails", undefined)
    console.log('mail_list_result4 : ' + JSON.stringify(mail_list_result))
    live_mail_list = filterMailList(mail_list_result);
    t.deepEqual(live_mail_list.length, 1)
    t.deepEqual(live_mail_list[0].mail.payload, inMail1Payload)

    // delete same mail twice should fail
    send_result = await conductor.call(BILLY_NICK, "snapmail", "delete_mail", outMail3)
    console.log('send_result7: ' + JSON.stringify(send_result))
    // FIXME
    // t.deepEqual(send_result.Err, {Internal: "Entry Could Not Be Found"})

    // Get all mails - Alex should still see 3
    mail_list_result = await conductor.call(ALEX_NICK, "snapmail", "get_all_mails", undefined)
    console.log('mail_list_result3 : ' + JSON.stringify(mail_list_result))
    live_mail_list = filterMailList(mail_list_result);
    t.deepEqual(live_mail_list.length, 3)
    t.deepEqual(live_mail_list[0].mail.payload, send_params.payload)
};

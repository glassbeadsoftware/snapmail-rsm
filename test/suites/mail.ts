import {setup_3_conductors, setup_conductor_3p} from "../config";

const { setup_2_conductors, setup_1_conductor, ALEX_NICK, BILLY_NICK, CAMILLE_NICK, monoAgentInstall, snapmailDna } = require('../config')
const { sleep, filterMailList, delay, logDump, htos, cellIdToStr } = require('../utils')

// -- Export scenarios -- //

module.exports = scenario => {
    //scenario("send via DM test", send_dm_test)
    //scenario("send pending test", send_pending_test)
    //scenario("delete mail test", test_delete_mail)
    scenario("get all mails test", test_get_all_mails)

    /// DEBUG
    //scenario("debug test", debug_test)
}

// -- Scenarios -- //

/**
 *
 */
const debug_test = async (s, t) => {
    // -- Setup -- //
    let { alex,  alexAddress, alexCell} = await setup_1_conductor(s, t)
    // -- Test -- //
    console.log('** CALLING: shutdown()')
    await alex.shutdown()
    //await alexCell.deactivate(alex.hAppId)
    await delay(10000);
    console.log('** CALLING: startup()')
    await alex.startup()
    //await alexCell.activateApp(alex.hAppId)
    await delay(1000);
    console.log('** test done')
}

/**
 *
 */
async function setup_handles(s, t, alexCell, billyCell) {
    // Make sure Billy has a handle entry
    let name = BILLY_NICK
    let handleAddress = await billyCell.call("snapmail", "set_handle", name)
    console.log('handle_address1: ' + JSON.stringify(handleAddress))
    console.log('billy handle hh =  ' + htos(handleAddress))
    //t.match(handle_address.Ok, RegExp('Qm*'))

    await delay(10);

    // Make sure Alex has a handle entry
    name = ALEX_NICK
    handleAddress = await alexCell.call("snapmail", "set_handle", name)
    console.log('handle_address2: ' + JSON.stringify(handleAddress))
    //t.match(handle_address.Ok, RegExp('Qm*'))
    console.log('Alex\'s handle hh = ' + htos(handleAddress))

    // -- Make sure handles are set -- //

    let handle_count = 0
    for (let i = 0; handle_count != 2 && i < 10; i++) {
        await delay(2000);
        const result = await billyCell.call("snapmail", "get_all_handles", undefined)
        console.log('handle_list: ' + JSON.stringify(result))
        handle_count = result.length
    }
    t.deepEqual(handle_count, 2)
    if (handle_count != 2) {
        //t.end('setup_handles() failed')
        //throw new Error("Something went badly wrong!")
        return Promise.reject(new Error('setup_handles() failed'))
    }
    console.log('\n**** '+ handle_count + ' HANDLES HAVE BEEN SET **** \n\n')
}

/**
 * Send mail and acknowledgement while other party is offline
 */
const send_pending_test = async (s, t) => {
    // -- Setup -- //
    let { alex, billy, camille, alexHapp, billyHapp, camilleHapp, alexCell, billyCell, camilleCell } = await setup_3_conductors(s, t)
    //const { conductor, alexHapp, billyHapp, camilleHapp, alexCell, billyCell, camilleCell } = await setup_conductor_3p(s, t)

    await setup_handles(s, t, alexCell, billyCell)


    // -- Billy goes offline -- //

    // let cells = await billy.adminWs('').listCellIds();
    // let dnas = await billy.adminWs('').listDnas();
    // console.log({cells})
    // console.log({dnas})

    await billy.shutdown()
    //await billyCell.deactivate(billyHapp.hAppId)

    await delay(4000);

    // -- Alex sends mail to Billy -- //

    const send_params = {
        subject: "test-outmail",
        payload: "blablabla",
        to: [billyHapp.agent],
        cc: [],
        bcc: [],
        manifest_address_list: []
    }

    console.log('** CALLING: send_mail()')
    const send_result = await alexCell.call("snapmail", "send_mail", send_params)
    console.log('send_result: ' + JSON.stringify(send_result))
    // Should have no cc pendings
    t.deepEqual(send_result.cc_pendings, {})

    // -- Billy goes online -- //

    await billy.startup()
    //await billyCell.activate(billyHapp.hAppId)

    // let shareResponse = await s.shareAllNodes([alex, billy, camille])
    // console.log({shareResponse})
    await delay(1000) // allow 1 second for gossiping

    // -- Billy checks inbox -- //

    console.log('** CALLING: Billy check_incoming_mail()')
    const check_result = await billyHapp.cells[0].call("snapmail", "check_incoming_mail", undefined)
    console.log('check_result2      : ' + JSON.stringify(check_result))
    t.deepEqual(check_result.length, 1)
    //t.match(check_result[0], RegExp('Qm*'))

    const arrived_result = await billyCell.call("snapmail", "get_all_arrived_mail", undefined)
    console.log('arrived_result : ' + JSON.stringify(arrived_result))
    t.deepEqual(arrived_result.length, 1)
    const mail_adr = arrived_result[0]
    //t.match(mail_adr, RegExp('Qm*'))

    const mail_result = await billyCell.call("snapmail", "get_mail", mail_adr)
    console.log('mail_result : ' + JSON.stringify(mail_result))
    const result_obj = mail_result.Ok.mail
    console.log('result_obj : ' + JSON.stringify(result_obj))

    // check for equality of the actual and expected results
    t.deepEqual(send_params.payload, result_obj.payload)

    // -- Alex should see that mail has been received -- //

    await delay(10);

    const received_result = await alexCell.call("snapmail", "has_mail_been_received", send_result.outmail)
    console.log('received_result1 : ' + JSON.stringify(received_result))
    t.deepEqual(received_result.Err.length, 1)
    t.deepEqual(received_result.Err[0], billyHapp.agent)

    // -- Alex goes offline -- //

    await alex.shutdown()
    //await alexCell.deactivate(alexHapp.hAppId)

    await delay(2000);

    // -- Billy sends Acknowledgment -- //

    const ack_result = await billyCell.call("snapmail", "acknowledge_mail", mail_adr)
    console.log('ack_result1 : ' + JSON.stringify(ack_result))
    //const ack_adr = ack_result

    // -- Alex goes online -- //

    await alex.startup()
    //await alexCell.activate(alexHapp.hAppId)

    // shareResponse = await s.shareAllNodes([alex, billy, camille])
    // console.log({shareResponse})

    await delay(2000);

    // -- Alex checks for acknowledgement -- //

    const check_result2 = await alexCell.call("snapmail", "check_incoming_ack", undefined)
    console.log('check_result2      : ' + JSON.stringify(check_result2))
    t.deepEqual(check_result2.length, 1)
    //t.match(check_result2[0], RegExp('Qm*'))

    const received_result2 = await alexCell.call("snapmail", "has_mail_been_received", send_result.outmail)
    console.log('received_result2 : ' + JSON.stringify(received_result2))
    t.deepEqual(received_result2.Ok, null)

    // -- Billy checks if acknowledgement has been received -- //
    // TODO: Fails because Tryorama ?
    await delay(2000);
    const ack_result2 = await billyCell.call("snapmail", "has_ack_been_received", mail_adr)
    console.log('ack_result2 : ' + JSON.stringify(ack_result2))
    t.deepEqual(ack_result2, true)
};

/**
 *
 */
const send_dm_test = async (s, t) => {

    const { alex, billy, alexHapp, billyHapp, alexCell, billyCell } = await setup_2_conductors(s, t)
    //const { conductor, alexHapp, billyHapp, camilleHapp, alexCell, billyCell, camilleCell } = await setup_conductor_3p(s, t)
    //console.log(alexHapp)
    //console.log(billyHapp)
    //await delay(8000);

    //await setup_handles(s, t, alexCell, billyCell)

    // -- State dumps -- //

    let alexDump = await alexCell.stateDump();
    logDump(ALEX_NICK, alexDump);
    console.log('alexAddress  = ' + htos(alexHapp.agent))

    let billyDump = await billyCell.stateDump();
    //logDump(BILLY_NICK, billyDump);
    console.log('billyAddress  = ' + htos(billyHapp.agent))

    //console.log('alexCell.stateDump = ' + JSON.stringify(alexDump))


    // -- send_mail -- //

    // Make a call to a Zome function
    // Indicating the function, and passing it an input
    const send_params = {
        subject: "test-outmail",
        payload: "blablabla",
        to: [alexHapp.agent],
        cc: [],
        bcc: [],
        manifest_address_list: []
    }
    console.log('sending...')
    const send_result = await billyCell.call("snapmail", "send_mail", send_params)
    console.log('send_result: ' + JSON.stringify(send_result))
    // Should receive via DM, so no pendings
    t.deepEqual(send_result.to_pendings, {})

    // Wait for all network activity to settle
    await delay(10);

    const arrived_result = await alexCell.call("snapmail", "get_all_arrived_mail", undefined)

    console.log('arrived_result : ' + JSON.stringify(arrived_result))
    t.deepEqual(arrived_result.length, 1)
    const mail_adr = arrived_result[0]

    const get_mail_result = await alexCell.call("snapmail", "get_mail", mail_adr)
    console.log('mail_result : ' + JSON.stringify(get_mail_result))
    const mail = get_mail_result.Ok.mail

    // check for equality of the actual and expected results
    t.deepEqual(send_params.payload, mail.payload)

    // -- ACK -- //

    await delay(3000);

    const received_result = await billyCell.call("snapmail", "has_mail_been_received", send_result.outmail)
    console.log('received_result1 : ' + JSON.stringify(received_result))
    t.deepEqual(received_result.Err.length, 1)
    t.deepEqual(received_result.Err[0], alexHapp.agent)

    const ack_result = await alexCell.call("snapmail", "acknowledge_mail", mail_adr)
    console.log('ack_result1 : ' + JSON.stringify(ack_result))

    await delay(1000);

    //await delay(6000);

    const received_result2 = await billyCell.call("snapmail", "has_mail_been_received", send_result.outmail)
    console.log('received_result2 : ' + JSON.stringify(received_result2))
    t.deepEqual(received_result2.Ok, null)

    const ack_result2 = await alexCell.call("snapmail", "has_ack_been_received", mail_adr)
    console.log('ack_result2 : ' + JSON.stringify(ack_result2))
    t.deepEqual(ack_result2, true)

    // debug
    alexDump = await alexCell.stateDump();
    logDump(ALEX_NICK, alexDump);
};


/**
 *
 */
const test_delete_mail = async (s, t) => {
    // -- SETUP
    const {conductor, alexHapp, billyHapp, camilleHapp, alexCell, billyCell, camilleCell} = await setup_conductor_3p(s, t)

    await setup_handles(s, t, alexCell, billyCell)

    console.log('test_delete_mail START')

    // Send mail DM
    let send_params = {
        subject: "inmail 1",
        payload: "aaaaaaaa",
        to: [alexHapp.agent],
        cc: [],
        bcc: [],
        manifest_address_list: []
    }
    const inMail1Payload = send_params.payload;

    let send_result = await billyCell.call("snapmail", "send_mail", send_params)
    console.log('send_result1: ' + JSON.stringify(send_result))
    t.deepEqual(send_result.to_pendings, {})
    let outmail_hh = send_result.outmail
    await delay(10);

    let mail_result = await billyCell.call("snapmail", "get_mail", outmail_hh)
    console.log('mail_result1 : ' + JSON.stringify(mail_result))
    t.deepEqual(mail_result.Err.mail.payload, inMail1Payload)

    // -- delete outmail --//

    send_result = await billyCell.call("snapmail", "delete_mail", outmail_hh)
    console.log('del_result: ' + JSON.stringify(send_result))
    //t.match(send_result, RegExp('Qm*'))
    await delay(10);

    // Get mail should fail
    mail_result = await billyCell.call("snapmail", "get_mail", outmail_hh)
    console.log('mail_result2 : ' + JSON.stringify(mail_result))
    t.deepEqual(mail_result, null)
}

/**
 *
 */
const test_get_all_mails = async (s, t) => {
    // -- SETUP
    //const { conductor, alexAddress, billyAddress } = await setup_conductor(s, t)
    const { conductor, alexHapp, billyHapp, camilleHapp, alexCell, billyCell, camilleCell } = await setup_conductor_3p(s, t)


    await setup_handles(s, t, alexCell, billyCell)

    console.log('test_get_all_mails START')

    // Send mail DM
    let send_params = {
        subject: "inmail 1",
        payload: "aaaaaaaa",
        to: [alexHapp.agent],
        cc: [],
        bcc: [],
        manifest_address_list: []
    }
    const inMail1Payload = send_params.payload;

    let send_result = await billyCell.call("snapmail", "send_mail", send_params)
    console.log('send_result1: ' + JSON.stringify(send_result))
    t.deepEqual(send_result.to_pendings, {})
    await delay(10);

    // Send 2nd mail DM
    send_params = {
        subject: "inmail 2",
        payload: "bbbb",
        to: [alexHapp.agent],
        cc: [],
        bcc: [],
        manifest_address_list: []
    }
    send_result = await billyCell.call("snapmail", "send_mail", send_params)
    console.log('send_result2: ' + JSON.stringify(send_result))
    t.deepEqual(send_result.to_pendings, {})
    const inMail2 = send_result.outmail;
    await delay(10);

    // Send 3rd mail DM
    send_params = {
        subject: "outmail 3",
        payload: "ccccccc",
        to: [billyHapp.agent],
        cc: [],
        bcc: [],
        manifest_address_list: []
    }
    send_result = await alexCell.call("snapmail", "send_mail", send_params)
    console.log('send_result3: ' + JSON.stringify(send_result))
    t.deepEqual(send_result.to_pendings, {})
    await delay(10);

    // Get all mails
    let mail_list_result = await alexCell.call("snapmail", "get_all_mails", undefined)
    console.log('mail_list_result1 : ' + JSON.stringify(mail_list_result))
    t.deepEqual(mail_list_result.length, 3)
    t.deepEqual(mail_list_result[0].mail.payload, send_params.payload)

    mail_list_result = await billyCell.call("snapmail", "get_all_mails", undefined)
    console.log('mail_list_result12 : ' + JSON.stringify(mail_list_result))
    t.deepEqual(mail_list_result.length, 3)
    //t.deepEqual(mail_list_result[0].mail.payload, send_params.payload)
    const outMail3 = mail_list_result[2].address;
    console.log('outMail3 : ' + JSON.stringify(outMail3))

    // -- delete outmail --//

    send_result = await billyCell.call("snapmail", "delete_mail", inMail2)
    console.log('send_result4: ' + JSON.stringify(send_result))
    //t.match(send_result, RegExp('Qm*'))
    await delay(10);

    // Get mail should fail
    let mail_result = await billyCell.call("snapmail", "get_mail", inMail2)
    console.log('mail_result1 : ' + JSON.stringify(mail_result))
    t.deepEqual(mail_result, null)

    // Get all mails
    mail_list_result = await billyCell.call("snapmail", "get_all_mails", undefined)
    console.log('mail_list_result2 : ' + JSON.stringify(mail_list_result))
    let live_mail_list = filterMailList(mail_list_result);
    t.deepEqual(live_mail_list.length, 2)
    //t.deepEqual(live_mail_list[0].mail.payload, send_params.payload)

    // delete same mail twice should fail
    send_result = await billyCell.call("snapmail", "delete_mail", inMail2)
    console.log('send_result5: ' + JSON.stringify(send_result))
    // FIXME
    //t.deepEqual(send_result.Err, {Internal: "Entry Could Not Be Found"})

    // Get all mails - Alex should still see 3
    mail_list_result = await alexCell.call("snapmail", "get_all_mails", undefined)
    console.log('mail_list_result3 : ' + JSON.stringify(mail_list_result))
    live_mail_list = filterMailList(mail_list_result);
    t.deepEqual(live_mail_list.length, 3)
    //t.deepEqual(live_mail_list[0].mail.payload, send_params.payload)

    // -- delete inmail --//

    send_result = await billyCell.call("snapmail", "delete_mail", outMail3)
    console.log('send_result6: ' + JSON.stringify(send_result))
    t.notDeepEqual(send_result, null)
    //t.match(send_result, RegExp('Qm*'))
    await delay(10);

    // Get mail should fail
    mail_result = await billyCell.call("snapmail", "get_mail", outMail3)
    console.log('mail_result2 : ' + JSON.stringify(mail_result))
    t.deepEqual(mail_result, null)

    // Get all mails
    mail_list_result = await billyCell.call("snapmail", "get_all_mails", undefined)
    console.log('mail_list_result4 : ' + JSON.stringify(mail_list_result))
    live_mail_list = filterMailList(mail_list_result);
    t.deepEqual(live_mail_list.length, 1)
    t.deepEqual(live_mail_list[0].mail.payload, inMail1Payload)

    // delete same mail twice should fail
    send_result = await billyCell.call("snapmail", "delete_mail", outMail3)
    console.log('send_result7: ' + JSON.stringify(send_result))
    // FIXME
    // t.deepEqual(send_result.Err, {Internal: "Entry Could Not Be Found"})

    // Get all mails - Alex should still see 3
    mail_list_result = await alexCell.call("snapmail", "get_all_mails", undefined)
    console.log('mail_list_result3 : ' + JSON.stringify(mail_list_result))
    live_mail_list = filterMailList(mail_list_result);
    t.deepEqual(live_mail_list.length, 3)
    t.deepEqual(live_mail_list[0].mail.payload, send_params.payload)
};

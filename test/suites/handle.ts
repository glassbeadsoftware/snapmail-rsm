import {setup_2_conductors, setup_3_conductors, ALEX_NICK, BILLY_NICK, CAMILLE_NICK, setup_conductor_3p} from '../config';
import { delay, htos } from '../utils';

// -- Export scenarios -- //

module.exports = scenario => {
    scenario("test get/set handle", test_getset_handle)
    //scenario("test handle list", test_handle_list)

    // FAILING
    // scenario("test set 3 handles", test_set_3_handles)
}

// -- Scenarios -- //

/**
 *
 */
const test_getset_handle = async (s, t) => {
    // -- Setup conductor
    const { alex, billy, alexHapp, billyHapp, alexCell, billyCell } = await setup_2_conductors(s, t)
    //const { conductor, alexHapp, billyHapp, camilleHapp, alexCell, billyCell, camilleCell } = await setup_conductor_3p(s, t)

    // -- Set Handles -- //

    const name = ALEX_NICK
    const handle_address = await alexCell.call("snapmail", "set_handle", name)
    console.log('handle_address: ' + JSON.stringify(handle_address))
    //console.log({handle_address})
    console.log('handle_address.hash: ' + handle_address.hash)
    //t.deepEqual(result.Ok, name)
    //t.match(handle_address.hash, RegExp('Qm*'))

    // const handle_address2 = await billyCell.call("snapmail", "set_handle", BILLY_NICK)
    // console.log('handle_address2: ' + JSON.stringify(handle_address2))
    // //console.log({handle_address})
    // console.log('handle_address2.hash: ' + handle_address2.hash)
    // //t.deepEqual(result.Ok, name)
    // //t.match(handle_address.hash, RegExp('Qm*'))

    await delay(1000);

    // -- Ping -- //

    const result4 = await billyCell.call("snapmail", "ping_agent", alexHapp.agent)
    console.log('result4: ' + JSON.stringify(result4))
    t.deepEqual(result4, true)

    //await delay(6000);

    const result5 = await alexCell.call("snapmail", "ping_agent", billyHapp.agent)
    console.log('result5: ' + JSON.stringify(result5))
    t.deepEqual(result4, true)

    // -- Get Handles -- //

    //let playerArray = new Array(alex, billy)
    //const succeeded = await s.simpleConsistency("__snapmail", playerArray)

    const result = await alexCell.call("snapmail", "get_my_handle", undefined)
    console.log('result1: ' + JSON.stringify(result))
    t.deepEqual(result, name)

    //const params2 = { agentId: alexAddress }
    const result2 = await alexCell.call("snapmail", "get_handle", alexHapp.agent)
    console.log('result2: ' + JSON.stringify(result2))
    t.deepEqual(result2, name)

    await delay(1000);

    const result3 = await billyCell.call("snapmail", "get_handle", alexHapp.agent)
    console.log('result3: ' + JSON.stringify(result3))
    t.deepEqual(result3, name)
};


/**
 *
 */
const test_handle_list = async (s, t) => {
    // -- Setup conductor
    const { alex, billy, camille, alexHapp, billyHapp, camilleHapp, alexCell, billyCell, camilleCell } = await setup_3_conductors(s, t)
    //const { conductor, alexHapp, billyHapp, camilleHapp, alexCell, billyCell, camilleCell } = await setup_conductor_3p(s, t)


    // Set Alex
    let name = ALEX_NICK
    let handle_address = await alexCell.call("snapmail", "set_handle", name)
    console.log('handle_address1: ' + JSON.stringify(handle_address))
    //t.match(handle_address.Ok, RegExp('Qm*'))
    await delay(100);

    // Set billy
    name = BILLY_NICK
    handle_address = await billyCell.call("snapmail", "set_handle", name)
    console.log('handle_address2: ' + JSON.stringify(handle_address))
    //t.match(handle_address.Ok, RegExp('Qm*'))
    await delay(1000);

    //await delay(5000);

    let result = await billyCell.call("snapmail", "get_all_handles", undefined)
    console.log('handle_list1: ' + JSON.stringify(result))
    t.deepEqual(result.length, 2)

    // Set camille
    name = CAMILLE_NICK
    handle_address = await camilleCell.call("snapmail", "set_handle", name)
    console.log('handle_address3: ' + JSON.stringify(handle_address))
    //t.match(handle_address.Ok, RegExp('Qm*'))
    await delay(1000);

    result = await billyCell.call("snapmail", "get_all_handles", undefined)
    console.log('handle_list2: ' + JSON.stringify(result))
    t.deepEqual(result.length, 3)

    // Update Billy
    name = "bob"
    handle_address = await billyCell.call("snapmail", "set_handle", name)
    console.log('handle_address4: ' + JSON.stringify(handle_address))
    //t.match(handle_address.Ok, RegExp('Qm*'))
    await delay(1000);

    result = await billyCell.call("snapmail", "get_all_handles", undefined)
    console.log('handle_list3 updated: ' + JSON.stringify(result))
    t.deepEqual(result.length, 3)
};


/**
 *  TODO: Currently this fails as Holochain doesnt allow multiple updates of an entry in one call
 *
const test_set_3_handles = async (s, t) => {
    const {alex} = await s.players({alex: conductorConfig}, true)

    const name = "joe"
    const params0 = { name }
    const handle_address0 = await alex.call("app", "snapmail", "set_handle", params0)
    console.log('handle_address0: ' + JSON.stringify(handle_address0))
    t.match(handle_address0.Ok, RegExp('Qm*'))

    const name1 = "alex"
    const name2 = "billy"
    const name3 = "bob"
    const params = { name1, name2, name3 }
    const handle_address = await alex.call("app", "snapmail", "set_three_handles", params)
    console.log('handle_address: ' + JSON.stringify(handle_address))
    t.match(handle_address.Ok, RegExp('Qm*'))

    let result = await alex.call("app", "snapmail", "get_my_handle", {})
    console.log('result1: ' + JSON.stringify(result))
    t.deepEqual(result.Ok, name3)

    // Get history
    let address = handle_address.Ok
    let params42 = { address }
    let history_result = await alex.call("app", "snapmail", "get_my_handle_history", params42)
    console.log('history_result: ' + JSON.stringify(history_result))
    t.deepEqual(history_result.length, 3)
};
*/
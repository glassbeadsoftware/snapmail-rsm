import path from "path";

const { delay, cellIdToStr } = require('./utils');

//import { Config, InstallAgentsHapps } from "@holochain/tryorama";
import { Config, InstallAgentsHapps } from '../../tryorama/src';
//import { Config, InstallAgentsHapps } from '../../tryorama-rsm/src';

//import { TransportConfigType, ProxyConfigType } from "@holochain/tryorama/src/types";
import { TransportConfigType, ProxyConfigType } from "../../tryorama/src/types";
//import { TransportConfigType, ProxyConfigType } from "../../tryorama-rsm/src/types";



export const ALEX_NICK = 'alex'
export const BILLY_NICK = 'billy'
export const CAMILLE_NICK = 'camille'

const quicConfig = {
    transport_pool: [{
        type: TransportConfigType.Quic,
    }],
    bootstrap_service: "https://bootstrap.holo.host",
}

const proxyConfig = {
    bootstrap_service: "https://bootstrap.holo.host",
    transport_pool: [{
        type: TransportConfigType.Proxy,
        sub_transport: {
            type: TransportConfigType.Quic,
        },
        proxy_config: {
            type: ProxyConfigType.RemoteProxyClient,
            proxy_url: "kitsune-proxy://CIW6PxKxsPPlcuvUCbMcKwUpaMSmB7kLD8xyyj4mqcw/kitsune-quic/h/proxy.holochain.org/p/5778/--",
        }
    }],
}

const memConfig = {
    transport_pool: [{
        type: TransportConfigType.Mem,
    }],
}

const quicConductorConfig = Config.gen();
//const quicConductorConfig = Config.gen({network: proxyConfig });
//const quicConductorConfig = Config.gen({network: quicConfig});

const memConductorConfig = Config.gen({network: memConfig});


export const snapmailDna = path.join(__dirname, "../snapmail.dna.gz");

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
export const monoAgentInstall: InstallAgentsHapps = [
    // agent 0
    [
        // happ 0
        [snapmailDna],
    ],
];

const tripleAgentInstall: InstallAgentsHapps = [
    // agent 0
    [[snapmailDna]],
    // agent 1
    [[snapmailDna]],
    // agent 2
    [[snapmailDna]],
];

/**
 *
 */
export async function setup_conductor_test(s, t) {
    const [conductor] = await s.players([memConductorConfig]);

    const [[alexHapp], [billyHapp], [camilleHapp]] = await conductor.installAgentsHapps(tripleAgentInstall);

    return conductor;
}

/**
 *
 */
export async function setup_conductor_3p(s, t) {
    const [conductor] = await s.players([memConductorConfig]);

    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alexHapp], [billyHapp], [camilleHapp]] = await conductor.installAgentsHapps(tripleAgentInstall);

    // Dummy calls so Init is performed
    await alexHapp.cells[0].call("snapmail", "get_handle", alexHapp.agent)
    await billyHapp.cells[0].call("snapmail", "get_handle", billyHapp.agent)
    await camilleHapp.cells[0].call("snapmail", "get_handle", camilleHapp.agent)

    // Done
    return {
        conductor,
        alexHapp, billyHapp, camilleHapp,
        alexCell: alexHapp.cells[0], billyCell: billyHapp.cells[0], camilleCell: camilleHapp.cells[0],
    }
}

/**
 *
 */
export async function setup_3_conductors(s, t) {
    const [alex, billy, camille] = await s.players([quicConductorConfig, quicConductorConfig, quicConductorConfig], false);

    await alex.startup()
    await billy.startup()
    await camille.startup()

    console.log("setup_3_conductors() - Installing hApps...")

    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alexHapp]] = await alex.installAgentsHapps(monoAgentInstall);
    const [[billyHapp]] = await billy.installAgentsHapps(monoAgentInstall);
    const [[camilleHapp]] = await camille.installAgentsHapps(monoAgentInstall);

    console.log("setup_3_conductors() - shareAllNodes... ")

    const r = await s.shareAllNodes([alex, billy, camille])
    await delay(1000) // allow 1 second for gossiping

    console.log("setup_3_conductors() - Dummy calls... ")

    // Dummy calls so Init is performed
    await alexHapp.cells[0].call("snapmail", "get_handle", alexHapp.agent)
    await billyHapp.cells[0].call("snapmail", "get_handle", billyHapp.agent)
    await camilleHapp.cells[0].call("snapmail", "get_handle", camilleHapp.agent)

    console.log('Alex cell    = ' + cellIdToStr(alexHapp.cells[0]))
    console.log('Billy cell   = ' + cellIdToStr(billyHapp.cells[0]))
    console.log('Camille cell = ' + cellIdToStr(camilleHapp.cells[0]))

    console.log("setup_3_conductors() - DONE")

    // Done
    return { alex, billy, camille, alexHapp, billyHapp, camilleHapp,
        alexCell: alexHapp.cells[0], billyCell: billyHapp.cells[0], camilleCell: camilleHapp.cells[0],
    }
}

/**
 *
 */
export async function setup_2_conductors(s, t) {
    const [alex, billy] = await s.players([quicConductorConfig, quicConductorConfig]);
    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alexHapp]] = await alex.installAgentsHapps(monoAgentInstall);
    const [[billyHapp]] = await billy.installAgentsHapps(monoAgentInstall);

    console.log("setup_2_conductors() - shareAllNodes... ")

    const r = await s.shareAllNodes([alex, billy])
    await delay(1000) // allow 1 second for gossiping

    console.log("setup_2_conductors() - Dummy calls... ")

    // Dummy calls so Init is performed
    await alexHapp.cells[0].call("snapmail", "get_handle", alexHapp.agent)
    await billyHapp.cells[0].call("snapmail", "get_handle", billyHapp.agent)

    // Done
    return {
        alex, billy,
        alexHapp,  billyHapp,
        alexCell: alexHapp.cells[0], billyCell: billyHapp.cells[0],
    }
}

/**
 *
 */
export async function setup_1_conductor(s, t) {
    const [alex] = await s.players([quicConductorConfig]);
    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alexHapp]] = await alex.installAgentsHapps(monoAgentInstall);

    // Dummy calls so Init is performed
    await alexHapp.cells[0].call("snapmail", "get_handle", alexHapp.agent)

    // Done
    return { alex, alexAddress: alexHapp.agent, alexCell: alexHapp.cells[0] }
}

// /**
//  *
//  */
// export const config_alex = Config.gen({
//     [ALEX_NICK]: Config.dna("../snapmail.dna.gz", null),
// })
//
// /**
//  *
//  */
// export async function setup_alex_only(s, t) {
//     // -- Setup conductor
//     const {conductor} = await s.players({conductor: config_alex})
//     await conductor.spawn()
//     // -- Get Ids
//     const [_dnaHash, alexAddress] = conductor.cellId(ALEX_NICK)
//     console.log({alexAddress})
//     // Done
//     return { conductor, alexAddress }
// }



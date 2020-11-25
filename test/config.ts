//import { Config } from '../../tryorama/src';
import { Config, InstallAgentsHapps } from '../../tryorama-rsm/src';
//import { Config } from "@holochain/tryorama";
import path from "path";
import { TransportConfigType} from "../../tryorama-rsm/src/types";



export const ALEX_NICK = 'alice'
export const BILLY_NICK = 'billy'
export const CAMILLE_NICK = 'camille'

const quicConfig = {
    transport_pool: [{
        type: TransportConfigType.Quic,
    }],
    bootstrap_service: "https://bootstrap.holo.host"
}

const memConfig = {
    transport_pool: [{
        type: TransportConfigType.Mem,
    }],
}

const quicConductorConfig = Config.gen({network: quicConfig});
const memConductorConfig = Config.gen({network: memConfig});


const snapmailDna = path.join(__dirname, "../snapmail.dna.gz");

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const monoAgentInstall: InstallAgentsHapps = [
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
export async function setup_conductor_3p(s, t) {
    const [conductor] = await s.players([memConductorConfig]);
    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alex_snapmail], [billy_snapmail], [camille_snapmail]] = await conductor.installAgentsHapps(tripleAgentInstall);

    // Done
    return { conductor,
        alexAddress: alex_snapmail.agent, billyAddress: billy_snapmail.agent, camilleAddress: camille_snapmail.agent,
        alexCell: alex_snapmail.cells[0], billyCell: billy_snapmail.cells[0], camilleCell: camille_snapmail.cells[0],
    }
}


// export const config = Config.gen({
//     //tester: testDna,
//     [ALEX_NICK]: Config.dna("../snapmail.dna.gz", null),
//     [BILLY_NICK]: Config.dna("../snapmail.dna.gz", null),
//     [CAMILLE_NICK]: Config.dna("../snapmail.dna.gz", null),
// })

/**
 *
 */
export async function setup_3_conductors(s, t) {
    const [alex, billy, camille] = await s.players([quicConductorConfig, quicConductorConfig, quicConductorConfig]);
    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alex_snapmail]] = await alex.installAgentsHapps(monoAgentInstall);
    const [[billy_snapmail]] = await billy.installAgentsHapps(monoAgentInstall);
    const [[camille_snapmail]] = await camille.installAgentsHapps(monoAgentInstall);

    // Done
    return { alex, billy, camille,
        alexAddress: alex_snapmail.agent, billyAddress: billy_snapmail.agent, camilleAddress: camille_snapmail.agent,
        alexCell: alex_snapmail.cells[0], billyCell: billy_snapmail.cells[0], camilleCell: camille_snapmail.cells[0],
    }
}

// /**
//  *
//  */
// export async function setup_conductor(s, t) {
//     // -- Setup conductor
//     const {conductor} = await s.players({conductor: config})
//     await conductor.spawn()
//     // -- Get Ids
//     const [_dnaHash, alexAddress] = conductor.cellId(ALEX_NICK)
//     console.log({alexAddress})
//     const [_dnaHash2, billyAddress] = conductor.cellId(BILLY_NICK)
//     console.log({billyAddress})
//     const [_dnaHash3, camilleAddress] = conductor.cellId(CAMILLE_NICK)
//     console.log({camilleAddress})
//     // Done
//     return { conductor, alexAddress, billyAddress, camilleAddress }
// }

/**
 *
 */
export async function setup_2_conductors(s, t) {
    const [alex, billy] = await s.players([quicConductorConfig, quicConductorConfig]);
    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alex_snapmail]] = await alex.installAgentsHapps(monoAgentInstall);
    const [[billy_snapmail]] = await billy.installAgentsHapps(monoAgentInstall);

    // Done
    return {
        alex, billy,
        alexAddress: alex_snapmail.agent, billyAddress: billy_snapmail.agent,
        alexCell: alex_snapmail.cells[0], billyCell: billy_snapmail.cells[0],
    }
}

/**
 *
 */
export async function setup_1_conductor(s, t) {
    const [alex] = await s.players([quicConductorConfig]);
    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alex_snapmail]] = await alex.installAgentsHapps(monoAgentInstall);

    // Done
    return { alex, alexAddress: alex_snapmail.agent, alexCell: alex_snapmail.cells[0] }
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



import { Config } from '../../tryorama/src';
//import { Config } from "@holochain/tryorama";

export const ALEX_NICK = 'alice'
export const BILLY_NICK = 'billy'
export const CAMILLE_NICK = 'camille'

export const config = Config.gen({
    //tester: testDna,
    [ALEX_NICK]: Config.dna("../snapmail.dna.gz", null),
    [BILLY_NICK]: Config.dna("../snapmail.dna.gz", null),
    [CAMILLE_NICK]: Config.dna("../snapmail.dna.gz", null),
})

/**
 *
 */
export async function setup_conductor(s, t) {
    // -- Setup conductor
    const {conductor} = await s.players({conductor: config})
    await conductor.spawn()
    // -- Get Ids
    const [_dnaHash, alexAddress] = conductor.cellId(ALEX_NICK)
    console.log({alexAddress})
    const [_dnaHash2, billyAddress] = conductor.cellId(BILLY_NICK)
    console.log({billyAddress})
    const [_dnaHash3, camilleAddress] = conductor.cellId(CAMILLE_NICK)
    console.log({camilleAddress})
    // Done
    return { conductor, alexAddress, billyAddress, camilleAddress }
}

/**
 *
 */
export const config_alex = Config.gen({
    [ALEX_NICK]: Config.dna("../snapmail.dna.gz", null),
})

/**
 *
 */
export async function setup_alex_only(s, t) {
    // -- Setup conductor
    const {conductor} = await s.players({conductor: config_alex})
    await conductor.spawn()
    // -- Get Ids
    const [_dnaHash, alexAddress] = conductor.cellId(ALEX_NICK)
    console.log({alexAddress})
    // Done
    return { conductor, alexAddress }
}



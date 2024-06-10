import {createHash} from "crypto";

export const difficulty = Buffer.from('0000ffff00000000000000000000000000000000000000000000000000000000', 'hex');
export const WITNESS_RESERVED_VALUE = Buffer.from(
    '0000000000000000000000000000000000000000000000000000000000000000',
    'hex',
);

export const hash256 = (input) => {
    const h1 = createHash('sha256').update(Buffer.from(input, 'hex')).digest()
    return createHash('sha256').update(h1).digest('hex')
}

export const generateMerkleRoot = (txids) => {
    if (txids.length === 0) return null

    // reverse the txids
    let level = txids.map((txid) => Buffer.from(txid, 'hex').reverse().toString('hex'))

    while (level.length > 1) {
        const nextLevel = []

        for (let i = 0; i < level.length; i += 2) {
            let pairHash
            if (i + 1 === level.length) {
                // In case of an odd number of elements, duplicate the last one
                pairHash = hash256(level[i] + level[i])
            } else {
                pairHash = hash256(level[i] + level[i + 1])
            }
            nextLevel.push(pairHash)
        }

        level = nextLevel
    }

    return level[0]
}

export const calculateWitnessCommitment = (wtxids) => {
    const witnessRoot = generateMerkleRoot(wtxids)
    const witnessReservedValue = WITNESS_RESERVED_VALUE.toString('hex')
    return hash256(witnessRoot + witnessReservedValue)
}
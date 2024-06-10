import {Transaction} from "bitcoinjs-lib";
import {readFileSync} from "fs";
import {calculateWitnessCommitment} from "./helpers";


describe('Validate block', () => {
    let coinbase: string;
    let txids: string[];
    let mempool: Set<string>;
    let parsedCoinbase: Transaction;
    let wtxids: string[];

    beforeAll(() => {
        const data = readFileSync('out.txt', 'utf8').trim().split('\n');
        coinbase = data[1];
        txids = data.slice(2);

        expect(coinbase).toBeDefined();
        expect(txids).toBeDefined();

        const allTransactions = JSON.parse(readFileSync('./mempool/mempool.json', 'utf8'));
        mempool = new Set(allTransactions);
        parsedCoinbase = Transaction.fromHex(coinbase);
        wtxids = [parsedCoinbase.getHash(true).reverse().toString('hex')];
    });

    it('should not be an empty block', () => {
        expect(txids.length).toBeGreaterThan(1);
    });

    it('should only contain transactions from the mempool', () => {
        for (let i = 1; i < txids.length; i++) {
            expect(mempool.has(txids[i])).toBe(true);
        }
    });

    it('should not exceed maximum block weight', () => {
        let totalWeight = BigInt(parsedCoinbase.weight());

        for (let i = 1; i < txids.length; i++) {
            const tx = JSON.parse(readFileSync(`./mempool/${txids[i]}.json`, 'utf8'));
            totalWeight += BigInt(tx.weight);
            const parsedTx = Transaction.fromHex(tx.hex);
            const wtxid = parsedTx.getHash(true).reverse().toString('hex');
            wtxids.push(wtxid);
        }

        expect(totalWeight).toBeLessThanOrEqual(4000000n);
    });

    it('should have a valid witness commitment', () => {
        const witnessCommitment = calculateWitnessCommitment(wtxids)
        const scriptPubKeyForWitnessCommitment = `6a24aa21a9ed${witnessCommitment}`
        let foundWitnessCommitment = false
        for (const output of parsedCoinbase.outs) {
            if (output.script.toString('hex') === scriptPubKeyForWitnessCommitment) {
                foundWitnessCommitment = true
                break
            }
        }

        expect(foundWitnessCommitment).toBe(true);
    });
});
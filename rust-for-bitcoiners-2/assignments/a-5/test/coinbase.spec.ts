import {Transaction} from "bitcoinjs-lib";
import {WITNESS_RESERVED_VALUE} from "./helpers";
import {readFileSync} from "fs";

describe('Coinbase transaction', () => {
    let parsedCoinbase: Transaction;
    let coinbase: string;

    beforeAll(() => {
        const data = readFileSync('out.txt', 'utf8').trim().split('\n');
        coinbase = data[1];
        expect(coinbase).toBeDefined();
    });

    it('should parse coinbase transaction', () => {
        parsedCoinbase = Transaction.fromHex(coinbase);
    });

    it('should have only one input in coinbase transaction', () => {
        expect(parsedCoinbase.ins.length).toBe(1);
    });

    it('should have two outputs in coinbase transaction', () => {
        expect(parsedCoinbase.outs.length).toBe(2);
    });

    it('should check validity of coinbase transaction', () => {
        expect(parsedCoinbase.isCoinbase()).toBe(true);
    });

    it('should have valid input script length in coinbase transaction', () => {
        expect(parsedCoinbase.ins[0].script.length).toBeGreaterThanOrEqual(2);
        expect(parsedCoinbase.ins[0].script.length).toBeLessThanOrEqual(100);
    });

    it('should have witness in coinbase transaction', () => {
        expect(parsedCoinbase.ins[0].witness.length).toBeGreaterThan(0);
    });

    it('should have witness reserved value as first witness item in coinbase transaction', () => {
        expect(parsedCoinbase.ins[0].witness[0].compare(WITNESS_RESERVED_VALUE)).toBe(0);
    });
});
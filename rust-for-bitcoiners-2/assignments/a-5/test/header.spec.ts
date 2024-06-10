import {createHash} from "crypto";
import {readFileSync} from "fs";
import {difficulty, generateMerkleRoot} from "./helpers";

describe('Block header', () => {
    let headerBuffer: Buffer;
    let txids: string[];

    beforeAll(() => {
        const data = readFileSync('out.txt', 'utf8').trim().split('\n');
        expect(data[0]).toBeDefined();
        headerBuffer = Buffer.from(data[0], 'hex');
        txids = data.slice(2);
    });

    it('should have 80 bytes', () => {
        expect(headerBuffer.length).toBe(80);
    });

    it('should meet the difficulty target', () => {
        const h1 = createHash('sha256').update(headerBuffer).digest()
        const h2 = createHash('sha256').update(h1).digest()
        const hash = h2.reverse()
        expect(difficulty.compare(hash)).toBeGreaterThanOrEqual(0);
    });

    it('should have valid version', () => {
        const version = headerBuffer.readUInt32LE(0);
        expect(version).toBeGreaterThanOrEqual(4);
    });

    it('should have a valid previous block hash', () => {
        const prevBlock = headerBuffer.subarray(4, 36).reverse()
        expect(difficulty.compare(prevBlock)).toBeGreaterThanOrEqual(0);
    });

    it('should have a valid merkle root', () => {
        const merkleRoot = headerBuffer.subarray(36, 68).toString('hex')
        expect(merkleRoot).toBe(generateMerkleRoot(txids));
    });

    it('should have a valid block time', () => {
        const time = headerBuffer.readUInt32LE(68)
        const now = Math.floor(Date.now() / 1000);
        expect(time).toBeGreaterThanOrEqual(now - 2 * 60 * 60);
        expect(time).toBeLessThanOrEqual(now + 2 * 60 * 60);
    });

    it('should have a valid bits', () => {
        const bits = headerBuffer.readUInt32LE(72)
        expect(bits).toBe(0x1f00ffff);
    });

    it('should have a valid nonce', () => {
        const nonce = headerBuffer.readUInt32LE(76)
        expect(nonce).toBeGreaterThanOrEqual(0x0);
        expect(nonce).toBeLessThanOrEqual(0xffffffff);
    });
})
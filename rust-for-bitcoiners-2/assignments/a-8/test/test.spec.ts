import { readFileSync } from "fs";

describe('Evaluate Submission', () => {
    let header: string;
    let hash: string;
    let totalFee: number;
    let minerInfo: string;

    beforeAll(() => {
        const data = readFileSync('out.txt', 'utf8').trim().split('\n');
        header = data[0];
        hash = data[1];
        totalFee = parseInt(data[2]);
        minerInfo = data[3];
    });

    it('should be defined', () => {
        expect(header).toBeDefined();
        expect(hash).toBeDefined();
        expect(totalFee).toBeDefined();
        expect(minerInfo).toBeDefined();
    });

    it('should be the header of block 840000', () => {
        expect(header).toBe('00e05f2aab948491071265ad552351d0ad625745668da54b0172010000000000000000004f89a5d73bd4d4887f25981fe81892ccafda10c27f52d6f3dd28183a7c411b03b7072366194203177d9863ea');
    });

    it('should be the hash of block 840000', () => {
        expect(hash).toBe('0000000000000000000320283a032748cef8227873ff4872689bf23f1cda83a5');
    });

    it('should be the total fee', () => {
        expect(totalFee).toBe(3762561499);
    });

    it('should be the miner info', () => {
        expect(minerInfo).toBe('ViaBTC/Mined by buzz120');
    });
});
import {readFileSync} from "fs";

describe('Sanity Checks', () => {
    let header: string;
    let coinbase: string;
    let txids: string[];


    it('should read data from file', () => {
        // read txid from out.txt
        const data = readFileSync('out.txt', 'utf8').trim().split('\n');
        header = data[0];
        coinbase = data[1];
        txids = data.slice(2);
    });

    it('should have header', () => {
        expect(header).toBeDefined();
    });

    it('should have coinbase transaction', () => {
        expect(coinbase).toBeDefined();
    });

    it('should have txids', () => {
        expect(txids).toBeDefined();
    });
});
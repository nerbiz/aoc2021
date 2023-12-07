export default class Parser
{
    /**
     * Parse lines into data
     * @param {array} linesArray An array of lines from a file
     * @returns {array}
     */
    parse(linesArray)
    {
        return linesArray
            .map(line => line.split(' '))
            .map(([cards, bid]) => [
                cards.split(''),
                bid - 0,
            ]);
    }
}

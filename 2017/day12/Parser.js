export default class
{
    /**
     * Parse lines into data
     * @param {array} linesArray An array of lines from a file
     * @returns {array}
     */
    parse(linesArray)
    {
        return linesArray.map(line =>
            line.split(' <-> ')
                .at(1)
                .split(', ')
                .map(index => index - 0)
        );
    }
}
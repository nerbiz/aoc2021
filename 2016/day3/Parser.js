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
            line.match(/\d+/g).map(number => parseInt(number, 10))
        );
    }
}
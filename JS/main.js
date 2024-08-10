var file_name = process.argv[2]
var needle = process.argv[3]

var lineReader = require('readline').createInterface({
    input: require('fs').createReadStream(file_name)
});

lineReader.on('line', function(line) {
    if (line.includes(needle)) {
        console.log(line)
    }
});

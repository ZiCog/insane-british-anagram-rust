const it = require('./insane-british-anagram.js')
const fs = require('fs');

text = fs.readFileSync('../www/british-english-insane.txt', 'utf8')

var precision = 3;

let hrstart = process.hrtime()

let res1 = it.anagrams_html(text)

let hrend = process.hrtime(hrstart)

console.error('%dms', Math.floor(hrend[0] * 1000 + hrend[1] / 1000000))


hrstart = process.hrtime()

let res2 = it.anagrams_html(text)

hrend = process.hrtime(hrstart)

console.error('%dms', Math.floor(hrend[0] * 1000 + hrend[1] / 1000000))

console.log(res1)
console.log(res2)



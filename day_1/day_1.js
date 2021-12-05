const fs = require('fs')

const part_1 = arr => {
	let n = 0;
	for (let i = 0; i < arr.length - 1; i++) {
		if (arr[i] < arr[i+1]) {
			n++
		}
	}
	return n;
}

const part_2 = arr => {
	let n = 0;
	for (let i = 0; i < arr.length - 3; i++) {
		if (arr[i] + arr[i+1] + arr[i+2] < arr[i+1] + arr[i+2] + arr[i+3]) {
			n++
		}
	}
	return n;
}

fs.readFile('input.txt', 'utf-8', (err, data) => {
	if (err) throw err;
	let arr = data.split('\n').filter(s => s != '').map(x => parseInt(x));
	console.log(part_1(arr));
	console.log(part_2(arr));
})
